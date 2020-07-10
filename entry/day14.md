# 7月10日，Day 14

继续学习 lab 4 中各个部分的代码结构并且手敲一遍，没有发现什么特别难以理解的地方，手动实现了一个基于队列做的调度器，后续可能会实现优先队列版本。

```rust
impl Scheduler<Arc<Thread>> for FifoScheduler<Arc<Thread>> {
    fn add_thread<T>(&mut self, thread: Thread, priority: T) {
        self.queue.push(Arc::from(thread))
    }

    fn get_next(&mut self) -> Option<Thread> {
        if self.queue.is_empty() {
            Some(*self.queue.remove(0))
        } else {
            None
        }
    }

    fn remove_thread(&mut self, thread: Thread) {
        for (idx,thrd) in self.queue.iter().enumerate() {
            if thread == **thrd {
                self.queue.remove(idx)
            }
        }
    }

    fn set_priority<T>(&mut self, thread: Thread, priority: T) {
        unimplemented!()
    }
}
```

由于是 FIFO 队列，因此并不能设置优先级。