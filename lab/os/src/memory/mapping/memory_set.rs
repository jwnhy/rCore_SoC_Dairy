use alloc::vec::Vec;

use crate::memory::address::{PhysicalAddress, VirtualAddress, VirtualPageNumber};
use crate::memory::frame::frame_tracker::FrameTracker;
use crate::memory::mapping::mapping::Mapping;
use crate::memory::mapping::segment::Segment;
use crate::memory::MemoryResult;

#[derive(Debug)]
pub struct MemorySet {
    pub mapping: Mapping,
    pub segments: Vec<Segment>,
    pub allocated_pairs: Vec<(VirtualPageNumber, FrameTracker)>,
}

impl MemorySet {
    pub fn flush(&self) {
        self.mapping.flush()
    }

    pub fn map(&mut self) -> MemoryResult<()> {
        for segment in self.segments.iter() {
            let new_pair = self.mapping.map(segment);
            self.allocated_pairs.extend(new_pair?)
        }
        Ok(())
    }
}