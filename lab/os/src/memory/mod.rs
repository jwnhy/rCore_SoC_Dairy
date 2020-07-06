
pub mod range;
mod heap;
pub mod address;
mod config;
pub mod mapping;
pub mod frame;
use crate::test::kernel_memory_check::kernel_memory_check;

pub type MemoryResult<T> = Result<T, &'static str>;

pub fn init() {
    heap::init();
    let mut memory_set = mapping::new_kernel().ok().unwrap();
    memory_set.map();
    kernel_memory_check(&memory_set);
    memory_set.flush();
}