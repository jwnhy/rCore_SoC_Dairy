use crate::memory::address::{PhysicalAddress, PhysicalPageNumber};

pub struct FrameTracker(PhysicalAddress);

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

    }
}