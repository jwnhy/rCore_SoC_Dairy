use alloc::collections::VecDeque;
use alloc::sync::Arc;

use spin::Mutex;

use crate::process::thread::Thread;
use crate::process::processor::PROCESSOR;

#[derive(Default)]
pub struct Condvar {
    watchers: Mutex<VecDeque<Arc<Thread>>>
}

impl Condvar {
    pub fn wait(&self) {
        self.watchers
            .lock()
            .push_back(PROCESSOR.get().current_thread());
        PROCESSOR.get().sleep_current_thread();
    }

    pub fn notify_one(&self) {
        if let Some(thread) = self.watchers.lock().pop_front() {
            PROCESSOR.get().wake_thread(thread);
        }
    }
}