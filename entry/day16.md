# 学习经验分享

#### 如何为 Lab 做准备

- Rust 基础知识（所有权，类型系统，模式匹配）
- RISC-V 汇编语言（算数运算，CSR 操作）
- 基本计算机组成常识（OS Three Easy Pieces / OS Principles and Practice）
- Writing an OS in Rust by Philipp Oppermann
- GitHub 的使用技巧

#### Lab 中的代码学习

- 建议自己对着教程和代码敲一遍，理解代码的组成和调用结构。（不建议直接 C/V）
- 尽信书不如无书，教程有一些小错漏，可以多多在 GitHub 提出 Issue 与 PR
- **单元测试**，**单元测试**，**单元测试**

## 两个亲身经历的 Bug

#### 单元测试的例子——内存布局检查

尽管我们的教程可能并不支持真正意义上的 `cargo test` 单元测试，但是你还是可以实现一些函数在你遇到问题时帮助你调试。

下面是一段我为了解决一个内核页表映射的 bug 时写的一个检查函数。

```RUst
pub fn kernel_memory_check(memory_set: &MemorySet) {
    println!("checking memory");
    for (idx, segment) in memory_set.segments[..5].iter().enumerate() {
        // 遍历前 5 段内核内存
        let flags = segment.flags;
        for vpn in segment.page_range().iter() {
            let mut va = VirtualAddress::from(vpn);
            // 这里修改了 loopup 函数，使它能够从一个给定的页表根节点出发
            let (pa, entry) = Mapping::lookup(Some(memory_set.mapping.root_ppn.0), va).unwrap();
            // 检查新页表与 boot_pagetable 是否一致
            assert_eq!(pa, PhysicalAddress::from(VirtualAddress::from(vpn)));
            // 检查新页表权限
            assert_eq!(entry.flags() | flags, entry.flags());
        }
    }
    // 检查新页表中，sp 栈顶指针是否拥有正确的权限
    let mut sp = 0;
    unsafe {
        llvm_asm!("mv $0, sp":"=r"(sp):::);
        let (_, entry) = Mapping::lookup(Some(memory_set.mapping.root_ppn.0), VirtualAddress(sp)).unwrap();
        assert_eq!(entry.flags(), Flags::VALID|Flags::READABLE|Flags::WRITABLE);
    }
    println!("memory checked");
}
```

这种函数可以解决像下面的问题。

```asm
	# .section .bss.stack
    # .global boot_stack
    # 如果写 entry.asm 时漏了上面两段，会导致 boot_stack 被放置在 .text 区段，当新页表映射完成之后， .text 区段只有只读权限，导致程序死循环
    # 而上面的测试，则可以检查新页表对 sp 的权限是否正确
boot_stack:
    .space 4096 * 16
    .global boot_stack_top

boot_stack_top:
    .section .data
    .align 12
```

#### 警惕自动化

Rust 是一门拥有相当多特性的语言，很多特性相当方便，可以让你省去很多烦恼。但是有的特性会在 debug 时给你带来很多困扰。

```rust
impl Drop for FrameTracker {
    fn drop(&mut self) {
        FRAME_ALLOCATOR.lock().dealloc(self);
    }
}
```

上面是我们在 Lab 2 中实现的自动释放已分配的但无人指向的物理帧的析构函数。

这确实很美好，但是如果当你试图重映射页表时就没有那么美好了。

```rust
pub fn init() {
    heap::init();
    println!("heap initialized");
    let mut memory_set = mapping::new_kernel().ok().unwrap();
    kernel_memory_check(&memory_set);
    memory_set.flush();
    println!("new mapping initialized");
    // 此处页表离开了作用域，但页表的指针仍然指向那些已经被释放掉的物理页面
}
```

`memory_set` 是一个存储了对内核区段的映射以及新页表中各个项的结构，当 `init` 返回时，由于 `memory_set` 离开了作用域，所以其中的成员都会被自动释放掉，这当中也包含了刚刚构造的新页表项，这些项被回收，再分配，当它们被二次写入的时候，就会导致页表的崩溃。

修复方法很简单，让页表被长期持有即可。

```rust
pub fn init() -> MemorySet {
    heap::init();
    println!("heap initialized");
    let mut memory_set = mapping::new_kernel().ok().unwrap();
    kernel_memory_check(&memory_set);
    memory_set.flush();
    println!("new mapping initialized");
    memory_set
    // 页表随着函数返回，被调用者持有
}
```