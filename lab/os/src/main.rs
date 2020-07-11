#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]

extern crate alloc;

use os::interrupt;
use os::memory;
use os::println;
use os::process::process::Process;
use os::process::processor::PROCESSOR;
use os::process::thread::Thread;
use os::memory::mapping::{new_kernel, Flags};
use os::process::config::STACK_SIZE;
use core::mem::size_of;
use os::interrupt::context::Context;

global_asm!(include_str!("asm/entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    let memory_set = memory::init();
    crate::interrupt::init();

    let process = Process::new_kernel().unwrap();

    for message in 0..8 {
        let thread = Thread::new(
            process.clone(),            // 使用同一个进程
            sample_process as usize,    // 入口函数
            Some(&[message]),           // 参数
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