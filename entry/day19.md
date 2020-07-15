# 7月15日，Day 19

## Lab 4 小结

感觉应该开始写 Rust 练习题了

#### 进程

进程是 OS 中**资源分配**的最小单元, 正常来说进程还应该管理使用的其他资源, 而 rCore 的实现较为简单, 只需要实现进程页表的映射即可.

```rust
#[derive(Debug)]
pub struct Process {
    pub is_user: bool,			// 是否为用户进程
    pub memory_set: MemorySet,  // 页表映射
}
```

如何新建一个进程则更需要讨论, 因为涉及到各种内存分配的问题.

一个最基本的进程只需要维护好一个**内核页表**的映射即可. 例如.

```rust
pub fn new_kernel() -> MemoryResult<Arc<RwLock<Self>>> {
    use crate::memory::mapping::new_kernel;
    Ok(Arc::new(RwLock::new(Self{
        is_user: false,
        memory_set: new_kernel()?
    })))
}
```

之所以需要维护**内核页表**的原因是不管什么进程, 我们总需要调用中断处理函数来进行中断的处理, 而这个函数恰恰需要**内核页表**.

进程还需要管理的是各个线程之间的内存资源, 毕竟每个线程都需要自己的空间运行. 因此我们还需要一个函数为各个线程分配内存 (分配完了就随线程使用了, 当然目前只用来分配线程的栈空间)

```rust
pub fn alloc_page_range(
    &mut self,
    size: usize,
    flags: Flags
) -> MemoryResult<Range<VirtualAddress>> {
    let alloc_size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
    let mut range = Range::<VirtualAddress>::from(0x1000000..0x1000000+alloc_size);
    while self.memory_set.overlap_with(range.into()) {
        range.start += alloc_size;
        range.end += alloc_size;
    }
    self.memory_set.add_segment(Segment
    {
        map_type: MapType::Framed,
        range,
        flags: flags | Flags::user(self.is_user),
    }
    ,None)?;
    Ok(Range::from(range.start..(range.start+size)))
}
```

这部分空间相当于直接在进程的 `memory_set` 上加了一部分. 以页的方式分配的内存并对其进行映射.

现在 `memory_set` 的结构

- .text 段
- .rodata 段
- .data 段
- .bss 段
- 线程 1 内存映射
- 线程 2 内存映射
- ...

rCore 的进程其实相当简单, 就只做了内存管理.

#### 线程

线程是 OS 中**程序运行**的最小单位. rCore 的线程其实也挺简单的, 实现了 创建-准备-停止-启动 的生命周期循环.

```Rust
#[derive(Debug)]
pub struct Thread {
    pub id: ThreadID,
    pub stack: Range<VirtualAddress>,
    pub process: Arc<RwLock<Process>>,
    pub inner: Mutex<ThreadInner>,
}

#[derive(Debug)]
pub struct ThreadInner {
    pub context: Option<Context>,
    pub sleeping: bool,
}
```

线程必须记录拥有自己的进程, 自己的栈, 以及自己的上下文, 其中上下文经常被修改因此独立出来.

```rust
pub fn prepare(&self) -> *mut Context {
    self.process.read().memory_set.flush();
    let parked_frame = self.inner().context.take().unwrap();

    KERNEL_STACK.push_context(parked_frame)

}

pub fn park(&self, context: Context) {
    assert!(self.inner().context.is_none());
    self.inner().context.replace(context);
}
```

准备线程的过程很简单, 将存储的上下文取出, 放入内核栈中, 这样在 `__restore` 时会直接将该线程上下文还原.

停止线程的方式正好相反,  `__interrupt` 会把上下文作为参数传入, 线程直接存起来就好.