use riscv::register::{sstatus::Sstatus};
use bitflags::_core::ops::DerefMut;
use core::mem::zeroed;
use riscv::register::sstatus::SPP::{User, Supervisor};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Context {
    pub x: [usize; 32],
    // 32 General Purpose Register
    pub sstatus: Sstatus,
    // Status registers, like enable/disable of interrupt
    pub sepc: usize,            // Previous program counter
}

impl Default for Context {
    fn default() -> Context {
            unsafe {
                zeroed()
            }
        }
}

impl Context {
    pub fn sp(&self) -> usize {
        self.x[2]
    }

    pub fn set_sp(&mut self, value: usize) -> &mut Self {
        self.x[2] = value;
        self
    }

    pub fn ra(&self) -> usize {
        self.x[1]
    }

    pub fn set_ra(&mut self, value: usize) -> &mut Self {
        self.x[1] = value;
        self
    }

    pub fn set_arguments(&mut self, arguments:&[usize]) -> &mut Self {
        assert!(arguments.len() <= 8);
        self.x[10..(10 + arguments.len())].copy_from_slice(arguments);
        self
    }

    pub fn new(
        stack_top: usize,
        entry_point: usize,
        arguments: Option<&[usize]>,
        is_user: bool
    ) -> Self {
        let mut context = Self::default();

        context.set_sp(stack_top);
        if let Some(arguments) = arguments {
            context.set_arguments(arguments);
        }
        context.sepc = entry_point;
        if is_user {
            context.sstatus.set_spp(User);
        } else {
            context.sstatus.set_spp(Supervisor);
        }
        context.sstatus.set_spie(true);
        context
    }
}