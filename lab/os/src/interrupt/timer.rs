use riscv::register::{sie, sstatus, time};

use crate::sbi::set_timer;

static INTERVAL: usize = 1000000;
pub static mut TICKS: usize = 0;

pub fn init() {
    unsafe {
        sie::set_stimer();
        // 在线程开启前禁用中断机制，开启位置为 `context.rs:67` 在那里设置了寄存器 `spie` 的值，__restore 中的 `sret` 指令会自动将其还原
        sstatus::clear_sie();
    }
    set_next_timeout();
}

fn set_next_timeout() {
    set_timer(time::read() + INTERVAL);
}

pub fn tick() {
    set_next_timeout();
    unsafe {
        TICKS += 1;
            println!("{} ticks", TICKS);
    }
}