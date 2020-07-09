use alloc::vec::Vec;

use crate::memory::address::{PhysicalAddress, VirtualAddress, VirtualPageNumber};
use crate::memory::frame::frame_tracker::FrameTracker;
use crate::memory::mapping::mapping::Mapping;
use crate::memory::mapping::segment::Segment;
use crate::memory::MemoryResult;
use crate::memory::range::Range;

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
            let new_pair = self.mapping.map(segment, None);
            self.allocated_pairs.extend(new_pair?)
        }
        Ok(())
    }

    pub fn overlap_with(&self, range: Range<VirtualAddress>) -> bool {
        for segment in self.segments.iter() {
            if range.overlap_with(&segment.range) {
                return true;
            }
        }
        false
    }

    pub fn add_segment(&mut self, segment: Segment, init_data: Option<&[u8]>) -> MemoryResult<()> {
        assert!(!self.overlap_with(segment.range));
        self.allocated_pairs.extend(self.mapping.map(&segment, init_data)?);
        self.segments.push(segment);
        Ok(())
    }
}