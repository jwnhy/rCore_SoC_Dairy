use riscv::register::scause::{Exception, Interrupt, Scause, Trap};
use riscv::register::stvec;

use crate::interrupt::timer;
use crate::process::processor::PROCESSOR;

use super::context::Context;


global_asm!(include_str!("../asm/interrupt.asm"));

pub fn init() {
    unsafe {
        extern "C" {
            fn __interrupt();
            fn __restore();
        }
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
    }
}

#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) -> *mut Context {
    let context = match scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        _ => unimplemented!("{:?}: {:x?}, stval: 0x{:x}", scause.cause(), context, stval),
    };
    context
    // unsafe {
    //     extern "C" {
    //         fn __restore(context: usize);
    //     }
    //     __restore(context as usize);
    // }
}

fn breakpoint(context: &mut Context) -> *mut Context {
    println!("Breakpoint at 0x{:x}", context.sepc);
    for (index, reg_val) in context.x[1..].iter().enumerate() {
        println!("Value of register x{} is {:x}", index + 1, reg_val);
    }
    context.sepc += 2;
    context
}

fn supervisor_timer(context: &mut Context) -> *mut Context {
    timer::tick();
    PROCESSOR.get().tick(context)
}