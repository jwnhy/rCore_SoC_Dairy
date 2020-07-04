#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
extern crate alloc;
global_asm!(include_str!("asm/entry.asm"));

use os::interrupt;
use os::memory;
use os::println;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    interrupt::init();
    memory::init();

    let remap = memory::mapping::new_kernel().unwrap().flush();

    use crate::memory::address::{VirtualAddress, PhysicalAddress};
    use os::memory::mapping::mapping::Mapping;

    println!("kernel remapped");
    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    }

    use alloc::boxed::Box;
    use alloc::vec::Vec;

    // 动态内存分配测试
    let v = Box::new(5);
    assert_eq!(*v, 5);
    let mut vec = Vec::new();
    for i in 0..10000 {
        vec.push(i);
    }
    for i in 0..10000 {
        assert_eq!(vec[i], i);
    }
    println!("heap test passed");

    use crate::memory::frame::allocator::FRAME_ALLOCATOR;
    for _ in 0..2 {
        let frame_0 = match FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        let frame_1 = match FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        println!("{} and {}", frame_0.address(), frame_1.address());
    }

    loop{}
}