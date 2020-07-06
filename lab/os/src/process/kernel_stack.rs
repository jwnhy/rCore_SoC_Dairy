use crate::process::config::KERNEL_STACK_SIZE;
use crate::interrupt::context::Context;
use core::mem::size_of;

#[repr(align(16))]
#[repr(C)]
pub struct KernelStack([u8; KERNEL_STACK_SIZE]);
pub static KERNEL_STACK: KernelStack = KernelStack([0; KERNEL_STACK_SIZE]);
impl KernelStack {
    pub fn push_context(&self, context: Context) -> *mut Context {
        let stack_top = &self as *const _ as usize + size_of::<Self>();
        let push_address = (stack_top - size_of::<Context>()) as *mut Context;
        unsafe {
            *push_address = context;
        }
        push_address
    }
}
