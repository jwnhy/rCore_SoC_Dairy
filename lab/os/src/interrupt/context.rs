use riscv::register::{sstatus::Sstatus};

#[repr(C)]
#[derive(Debug)]
pub struct Context {
    pub x: [usize; 32],
    // 32 General Purpose Register
    pub sstatus: Sstatus,
    // Status registers, like enable/disable of interrupt
    pub sepc: usize,            // Previous program counter
}