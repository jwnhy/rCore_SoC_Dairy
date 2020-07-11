use crate::process::scheduler::fifo_scheduler::FifoScheduler;
use crate::process::thread::Thread;
use alloc::sync::Arc;

mod fifo_scheduler;
pub type SchedulerImpl<ThreadType> = FifoScheduler<ThreadType>;
pub trait Scheduler<ThreadType: Clone + Eq>: Default {
    fn add_thread<T>(&mut self, thread: ThreadType, priority: T);
    fn get_next(&mut self) -> Option<ThreadType>;
    fn remove_thread(&mut self, thread: &ThreadType);
    fn set_priority<T>(&mut self, thread: ThreadType, priority: T);
}