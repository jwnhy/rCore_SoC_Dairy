use lazy_static::lazy_static;
use spin::{Mutex, RwLock};
use crate::memory::address::VirtualAddress;
use crate::memory::range::Range;
use alloc::sync::Arc;
use crate::interrupt::context::Context;
use crate::process::process::Process;
use crate::process::kernel_stack::{KernelStack, KERNEL_STACK};
use core::mem::size_of;
use crate::memory::MemoryResult;


pub type ThreadID = isize;
lazy_static! {
    pub static ref THREAD_COUNTER: Mutex<ThreadID> = Mutex::new(0);
}

pub struct Thread {
    pub id: ThreadID,
    pub stack: Range<VirtualAddress>,
    pub process: Arc<RwLock<Process>>,
    pub inner: Mutex<ThreadInner>,
}

pub struct ThreadInner {
    pub context: Option<Context>,
    pub sleeping: bool,VirtualAddress
}

impl Thread {
    pub fn inner(&self) -> spin::MutexGuard<ThreadInner> {
        self.inner.lock()
    }

    pub fn prepare(&self) -> *mut Context {
        self.process.write().memory_set.map();
        self.process.read().memory_set.flush();
        let parked_frame = self.inner().context.take().unwrap();

        if self.process.read().is_user {
            KERNEL_STACK.push_context(parked_frame)
        } else {
            let context = (parked_frame.sp() - size_of::<Context>()) as *mut Context;
            unsafe { *context = parked_frame }
            context
        }
    }

    pub fn park(&self, context: Context) {
        assert!(self.inner().context.is_none());
        self.inner().context.replace(context);
    }

    pub fn new(
        process: Arc<RwLock<Process>>,
        entry_point: usize,
        arguments: Option<[&usize]>,
    ) -> MemoryResult<Arc<Thread>> {
        let stack = process.write()
    }
}