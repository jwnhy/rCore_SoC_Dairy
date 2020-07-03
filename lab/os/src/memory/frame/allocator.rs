use spin::Mutex;
use lazy_static::lazy_static;
use super::super::config::{KERNEL_END_ADDRESS, MEMORY_END_ADDRESS};
use super::super::address::{PhysicalPageNumber, PhysicalAddress};

lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator<AllocatorImpl>>
        = Mutex::new(
            FrameAllocator::new(
                Range::from(
                    PhysicalPageNumber::ceil(PhysicalAddress::from(*KERNEL_END_ADDRESS))..PhysicalPageNumber::floor(MEMORY_END_ADDRESS)
                    )));
}