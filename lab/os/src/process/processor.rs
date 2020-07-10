use lazy_static::lazy_static;
use alloc::sync::Arc;
use crate::process::thread::Thread;
use crate::process::scheduler::SchedulerImpl;
lazy_static! {
    /// 全局的 [`Processor`]
    pub static ref PROCESSOR: UnsafeWrapper<Processor> = Default::default();
}


#[derive(Default)]
pub struct Processor {
    current_thread: Option<Arc<Thread>>,
    scheduler: SchedulerImpl<Arc<Thread>>
}

impl Processor {
    pub fn run(&mut self) -> ! {
        extern "C" {
            fn __restore(context: usize);
        }
        let context = self.current_thread().run();
        unsafe {
            __restore(context as usize);
        }
        unreachable!()
    }
}