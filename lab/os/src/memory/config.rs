use lazy_static::lazy_static;
use super::address::VirtualAddress;
lazy_static!{
    pub static ref KERNEL_END_ADDRESS: VirtualAddress = VirtualAddress(kernel_end as usize);
}

pub const KERNEL_MAP_OFFSET: usize = 0xffff_ffff_0000_0000;
pub const PAGE_SIZE: usize = 4096;