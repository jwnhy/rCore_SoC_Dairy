use crate::memory::mapping::mapping::Mapping;
use alloc::vec::Vec;
use crate::memory::mapping::segment::Segment;
use crate::memory::address::{VirtualPageNumber, VirtualAddress, PhysicalAddress};
use crate::memory::frame::frame_tracker::FrameTracker;
#[derive(Debug)]
pub struct MemorySet {
    pub mapping: Mapping,
    pub segments: Vec<Segment>,
    pub allocated_pairs: Vec<(VirtualPageNumber, FrameTracker)>
}

impl MemorySet {
    pub fn memory_check(&self) {
        for segment in &self.segments {
            for vpn in segment.page_range.iter() {
                let va = VirtualAddress::from(vpn);
                let pa = Mapping::lookup(Some(self.mapping.root_ppn.0), va).unwrap();
                println!("{:x?}->{:x?}", va, pa);
                assert_eq!(PhysicalAddress::from(va), pa);
            }
        }
    }
    pub fn flush(&self) {
        self.mapping.flush()
    }
}