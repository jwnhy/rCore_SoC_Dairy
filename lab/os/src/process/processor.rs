use alloc::sync::Arc;
use core::cell::UnsafeCell;

use hashbrown::HashSet;
use lazy_static::lazy_static;
use super::unsafe_wrapper::UnsafeWrapper;

use crate::interrupt::context::Context;
use crate::process::scheduler::{Scheduler, SchedulerImpl};
use crate::process::thread::Thread;
use crate::memory::mapping::new_kernel;

lazy_static! {
    /// 全局的 [`Processor`]
    pub static ref PROCESSOR: UnsafeWrapper<Processor> = Default::default();
}


#[derive(Default)]
pub struct Processor {
    current_thread: Option<Arc<Thread>>,
    scheduler: SchedulerImpl<Arc<Thread>>,
    sleeping_threads: HashSet<Arc<Thread>>,
}

impl Processor {
    pub fn run(&mut self) -> ! {
        extern "C" {
            fn __restore(context: usize);
        }
        if let Some(thread) = &self.current_thread {
            let context = thread.prepare();
            unsafe {
                __restore(context as usize);
            }
        }
        panic!("no thread to run, shutting down");
    }

    pub fn tick(&mut self, context: &mut Context) -> *mut Context {
        if let Some(next_thread) = self.scheduler.get_next() {
            if next_thread == *self.current_thread.as_ref().unwrap() {
                // 没有更换线程，直接返回 Context
                context
            } else {
                // 准备下一个线程
                let next_context = next_thread.prepare();
                print!("preparing {} ", next_thread.id);
                let current_thread = self.current_thread.replace(next_thread).unwrap();
                println!("parking {}", current_thread.id);

                // 储存当前线程 Context
                current_thread.park(*context);
                // 返回下一个线程的 Context
                next_context
            }
        } else {
            panic!("all threads terminated, shutting down");
        }
    }
    pub fn add_thread(&mut self, thread: Arc<Thread>) {
        if self.current_thread.is_none() {
            self.current_thread = Some(thread.clone());
        }
        self.scheduler.add_thread(thread, 0);
    }

    pub fn current_thread(&self) -> Arc<Thread> {
        self.current_thread.as_ref().unwrap().clone()
    }

    pub fn sleep_current_thread(&mut self) {
        let current_thread = self.current_thread();
        current_thread.inner.lock().sleeping = true;
        self.scheduler.remove_thread(&current_thread);
        self.sleeping_threads.insert(current_thread);
    }

    pub fn wake_thread(&mut self, thread: Arc<Thread>) {
        thread.inner.lock().sleeping = false;
        self.sleeping_threads.remove(&thread);
        self.scheduler.add_thread(thread, 0);
    }
}