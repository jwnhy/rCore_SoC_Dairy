mod range;
mod heap;
pub mod address;
mod config;
pub mod mapping;
pub mod frame;

pub type MemoryResult<T> = Result<T, &'static str>;
pub fn init(){
    heap::init();
}