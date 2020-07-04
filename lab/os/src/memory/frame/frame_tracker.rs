use crate::memory::address::{PhysicalAddress, PhysicalPageNumber};
use crate::memory::frame::allocator::FRAME_ALLOCATOR;
#[derive(Debug)]
pub struct FrameTracker(pub PhysicalAddress);

impl FrameTracker {
    pub fn address(&self) -> PhysicalAddress {
        self.0
    }

    pub fn page_number(&self) -> PhysicalPageNumber {
        PhysicalPageNumber::from(self.0)
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        FRAME_ALLOCATOR.lock().dealloc(self)
    }
}