use crate::memory::{address::*, mapping::Flags, range::Range};

#[derive(Debug)]
pub enum MapType {
    Linear,
    Framed,
}

#[derive(Debug)]
pub struct Segment {
    pub map_type: MapType,
    pub page_range: Range<VirtualPageNumber>,
    pub flags: Flags,
}

impl Segment {
    pub fn iter_mapped(&self) -> Option<impl Iterator<Item=PhysicalPageNumber>> {
        match self.map_type {
            MapType::Linear => Some(self.page_range.iter().map(PhysicalPageNumber::from)),
            MapType::Framed => None
        }
    }
}

