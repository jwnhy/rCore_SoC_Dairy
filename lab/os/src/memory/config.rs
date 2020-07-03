use lazy_static::lazy_static;
use super::address::VirtualAddress;
use crate::memory::address::PhysicalAddress;
lazy_static!{
    pub static ref KERNEL_END_ADDRESS: VirtualAddress = VirtualAddress(kernel_end as usize);
}

extern "C" {
    fn kernel_end();
}

pub const MEMORY_START_ADDRESS: PhysicalAddress = PhysicalAddress(0x8000_0000);
pub const MEMORY_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x8800_0000);

pub const KERNEL_MAP_OFFSET: usize = 0xffff_ffff_0000_0000;
pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;// 8MiB Kernel Heap, only used in kernel.
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_ENTRY_SIZE: usize = 8;