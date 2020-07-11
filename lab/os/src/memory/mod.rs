
pub mod range;
pub mod heap;
pub mod address;
pub mod config;
pub mod mapping;
pub mod frame;
use crate::test::kernel_memory_check::kernel_memory_check;
use crate::memory::mapping::memory_set::MemorySet;

pub type MemoryResult<T> = Result<T, &'static str>;

pub fn init() {
    heap::init();
    println!("heap initialized");

    // let mut memory_set = mapping::new_kernel().ok().unwrap();
    // //kernel_memory_check(&memory_set);
    // memory_set.flush();
    // println!("new mapping initialized");
    // memory_set
}