# 7月4日，Day 8

## Lab 2 & 3

终于把 Lab 2 & 3 调通啦，一个坑爹问题调了一下午，没救了。

#### Frame Allocator/Tracker

这两者组合起来是为了记录分配出去的物理页，这些页是紧随在 `KERNEL_END_ADDRESS` 之后的，即除内核所需的内存之外的所有可用内存。

`Tracker` 类似于一个智能指针，负责记录物理页的地址和页号。

```Rust
impl FrameTracker {
    /// 帧的物理地址
    pub fn address(&self) -> PhysicalAddress {
        self.0
    }
    /// 帧的物理页号
    pub fn page_number(&self) -> PhysicalPageNumber {
        PhysicalPageNumber::from(self.0)
    }
}
```

而 `Allocator` 则记录已用的和未用的内存空间。其只需要实现 `alloc` 与 `dealloc` 两个方法即可，具体的实现我们可以临时更换。

正如 [day6](https://github.com/JohnWestonNull/rCore_SoC_Dairy/blob/master/entry/day6.md) 中所说，页表实际上就是一个多层树形结构。那么页表肯定也要存储在内存中，所以也需要依赖上面的 `Frame` 机制来实现。

#### 页表与页表项

出于 RISCV 优秀的设计，页表的大小正好与一个页的大小相等，在 Sv39 中均为 4 KiB，因此我们完全可以分配一个 `Frame` 来存储页表/页表项。

rCore 的做法相当漂亮，实现了 `FrameTracker` 到 `PageTableTracker` 的转换，达到了分配后立刻就能使用的页表项目的。

![img](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-3/pics/sv39_pte.jpg)

![img](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-3/pics/sv39_rwx.jpg)

在内核重映射的开始阶段，我们需要一个 `boot_page_table` 用来在页表初始化之前，进行内核的各项操作，不然直接使用虚拟地址会让所有地址找不到对应的内存。

启动用的页表结构相当简单，只有单吉页的设计，每一个页表项都映射一个 1 GiB 的吉页。

```asm
boot_page_table:
    .quad 0
    .quad 0
    # 0x8000_0000[VPN2] = (0x8000_0000 >> 30) = 0x10 = 2 (Second Entry)
    .quad (0x80000 << 10) | 0xcf    # entry: 0x8000_0000 -> 0x8000_0000
    .zero 507 * 8
    # 0xffff_ffff_8000_0000[VPN2] = (0xffff_ffff_8000_0000 >> 30)= 510
    .quad (0x80000 << 10) | 0xcf    # entry: 0xffff_ffff_8000_0000 -> 0x8000_0000
    .quad 0
```

之所以保留一个 `0x8000_0000` -> `0x8000_0000` 的原因是诸如 `PC` 之类的寄存器，在系统启动时即被使用，内部的地址仍为单纯的物理地址，所以需要保留一个原始映射。

#### TLB

由于内存很慢，CPU 很快，因此 CPU 会缓存页表中的内容，所以在将页表写入 `satp` 寄存器之后我们需要用 `sfence.vma` 命令刷新 TLB 缓存来达到页表切换的目的。

#### 内存区段 Segment

在使用物理内存时，显然对成段的内存需求是更多的，因此需要定义一种数据结构，它组织了成段的内存，同时定义了其读写权限。但是又足够灵活，可以对页采用不同的映射方案。

```rust
pub enum MapType {
    Linear,
    Framed,
}
pub struct Segment {
    pub map_type: MapType,
    pub page_range: Range<VirtualPageNumber>,
    pub flags: Flags,
}
```

其中 `map_type` 指出了这块区段的映射方式，在内核内存中，通常使用 `Linear` 的方式直接将内核内存线性映射到高内存地址区域。而在用户内存中，则更多采用**分页**的分配方式以提高对内存的利用率。

在线性分配方式中，我们可以直接将 `VirtualAddress` 利用 `address.rs` 中定义的默认实现转化为 `PhysicalAddress` 。

在分页分配方式中，我们则需要调用 `FrameAllocator` 分配一个页面供该区段使用。

```Rust
pub fn map(&mut self, segment: &Segment) -> MemoryResult<Vec<(VirtualPageNumber, FrameTracker)>> {
    if let Some(ppn_iter) = segment.iter_mapped() {
        for (vpn, ppn) in segment.page_range.iter().zip(ppn_iter) {
            self.map_one(vpn, ppn, segment.flags)?;
            }
        Ok(vec![])
    } else {
        let mut allocated_pairs = vec![];
        for vpn in segment.page_range.iter() {
            let frame: FrameTracker = FRAME_ALLOCATOR.lock().alloc()?;
            self.map_one(vpn, frame.page_number(), segment.flags)?;
            allocated_pairs.push((vpn, frame));
        }
        Ok(allocated_pairs)
    }
}
```

#### 内存集合 MemorySet

当然一个用户程序不可能用的全是一整段的内存，中间肯定会有各种间隔。因此我们还需要再抽象出一个结构。

```Rust
pub struct MemorySet {
    pub mapping: Mapping,
    pub segments: Vec<Segment>,
    pub allocated_pairs: Vec<(VirtualPageNumber, FrameTracker)>
}
```

它记录了内存中各个区段的位置，分配出去的页，以及使用的映射方式和根页表位置。

在处理下午的问题时，我写了个 `memory_check` 函数来保证新页表中关于内核区段的内容是和 `boot_page_table` 等效的。

尽管问题并不出在这，它依赖了教程中提供的 `lookup` 函数来搜索页表项。

````Rust
pub fn memory_check(&self) {
    for segment in &self.segments {
        for vpn in segment.page_range.iter() {
            let va = VirtualAddress::from(vpn); 
            let pa = Mapping::lookup(Some(self.mapping.root_ppn.0), va).unwrap();
            println!("{:x?} -> {:x?}", va, pa);
            assert_eq!(PhysicalAddress::from(va), pa);
        }
    }
}
````

#### 下午的问题

我忘记将内核区域中的 `boot_stack` 放置在 `.bss` 区段而是放置在了 `.text` 区段，导致新页表生效时， `.text` 区段被设置成为只读，而中断处理函数则不断试图访问栈，导致死循环。

