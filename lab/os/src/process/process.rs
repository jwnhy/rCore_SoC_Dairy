use crate::memory::mapping::memory_set::MemorySet;
use crate::memory::MemoryResult;
use alloc::sync::Arc;
use spin::RwLock;

pub struct Process {
    pub is_user: bool,
    pub memory_set: MemorySet,
}

#[allow(unused)]
impl Process {
    pub fn new_kernel() -> MemoryResult<Arc<RwLock<Self>>> {
        use crate::memory::mapping::new_kernel;
        Ok(Arc::new(RwLock::new(Self{
            is_user: false,
            memory_set: new_kernel()?
        })))
    }
}