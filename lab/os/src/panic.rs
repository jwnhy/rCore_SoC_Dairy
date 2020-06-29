use core::panic::PanicInfo;
use crate::sbi::shutdown;
use crate::println;
const RED: &str = "\x1b[1;31m";
const RESET: &str = "\x1b[0m";
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}panic: '{}'{}",RED, info, RESET);
    shutdown();
}

#[no_mangle]
extern "C" fn abort() -> !{
    panic!("abort()")
}