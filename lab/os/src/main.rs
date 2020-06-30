#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]

global_asm!(include_str!("asm/entry.asm"));

use os::interrupt;
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    interrupt::init();

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    }
    loop {}
}