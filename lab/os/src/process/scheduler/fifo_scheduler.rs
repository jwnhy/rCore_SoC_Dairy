use super::Scheduler;
use alloc::vec::Vec;
use crate::process::thread::Thread;
use alloc::sync::Arc;

pub struct FifoScheduler<ThreadType: Clone + Eq> {
    queue: Vec<ThreadType>
}

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