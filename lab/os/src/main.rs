#![no_std]
#![no_main]
#![feature(global_asm)]

global_asm!(include_str!("asm/entry.asm"));

use os::println;
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Hello rCore-tutorial");
    panic!("end of rust main");
}