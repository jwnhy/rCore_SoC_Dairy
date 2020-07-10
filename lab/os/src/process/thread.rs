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
use crate::process::config::STACK_SIZE;
use crate::memory::mapping::Flags;
use core::hash::{Hash, Hasher};


pub type ThreadID = isize;
static mut THREAD_COUNTER: ThreadID = 0;

pub struct Thread {
    pub id: ThreadID,
    pub stack: Range<VirtualAddress>,
    pub process: Arc<RwLock<Process>>,
    pub inner: Mutex<ThreadInner>,
}

pub struct ThreadInner {
    pub context: Option<Context>,
    pub sleeping: bool,
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
        arguments: Option<&[usize]>,
    ) -> MemoryResult<Arc<Thread>> {
        let stack = process.write().alloc_page_range(STACK_SIZE, Flags::READABLE | Flags::WRITALBE)?;
        let context = Context::new(
            stack.end.into(),
            entry_point,
            arguments,
            process.read().is_user
        );
        let thread = Arc::new(Thread {
            id: unsafe {
                THREAD_COUNTER += 1;
                THREAD_COUNTER
            },
            stack,
            process,
            inner: Mutex::new(
                ThreadInner {
                    context: Some(context),
                    sleeping: false,
                }
            )
        });
        Ok(thread)
    }
}

impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Thread {}

impl Hash for Thread {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_isize(self.id);
    }
}