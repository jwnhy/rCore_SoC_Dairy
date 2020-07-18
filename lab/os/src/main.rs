#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]

extern crate alloc;

use core::mem::size_of;

use os::interrupt;
use os::interrupt::context::Context;
use os::memory;
use os::memory::mapping::{Flags, new_kernel};
use os::memory::config::KERNEL_END_ADDRESS;
use os::println;
use os::process::config::STACK_SIZE;
use os::process::process::Process;
use os::process::processor::PROCESSOR;
use os::process::thread::Thread;
use os::memory::address::PhysicalAddress;

global_asm!(include_str!("asm/entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main(_hart_id: usize, dtb_pa: PhysicalAddress) -> ! {
    let memory_set = memory::init();
    crate::interrupt::init();
    println!("{:x?}", dtb_pa);
    println!("{:x?}", PhysicalAddress::from(*KERNEL_END_ADDRESS));

    let process = Process::new_kernel().unwrap();

    for message in 0..8 {
        let thread = Thread::new(
            process.clone(),
            sample_process as usize,
            Some(&[message]),
        ).unwrap();
        PROCESSOR.get().add_thread(thread);
    }
    PROCESSOR.get().run();

    loop {}
}

fn sample_process(message: usize) {
    loop {
        println!("thread {}", message);
    }
}