mod stacked_allocator;

use spin::Mutex;
use lazy_static::lazy_static;
use crate::memory::config::{KERNEL_END_ADDRESS, MEMORY_END_ADDRESS};
use crate::memory::address::{PhysicalPageNumber, PhysicalAddress};
use crate::memory::range::Range;
use crate::memory::frame::frame_tracker::FrameTracker;
use crate::memory::MemoryResult;

type AllocatorImpl = stacked_allocator::StackedAllocator;

pub trait Allocator {
    fn new(capacity: usize) -> Self;
    fn alloc(&mut self) -> Option<usize>;
    fn dealloc(&mut self, index: usize);
}

pub struct FrameAllocator<T: Allocator> {
    start_ppn: PhysicalPageNumber,
    allocator: T,
}

impl<T: Allocator> FrameAllocator<T> {
    pub fn new(range: impl Into<Range<PhysicalPageNumber>> + Copy) -> Self {
        FrameAllocator {
            start_ppn: range.into().start,
            allocator: T::new(range.into().len()),
        }
    }

    pub fn alloc(&mut self) -> MemoryResult<FrameTracker> {
        self.allocator
            .alloc()
            .ok_or("no available frame to allocator")
            .map(|offset| FrameTracker(PhysicalAddress::from(self.start_ppn + offset)))
    }

    pub fn dealloc(&mut self, frame: &FrameTracker) {
        self.allocator.dealloc(frame.page_number() - self.start_ppn);
    }
}

lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator<AllocatorImpl>>
        = Mutex::new(
            FrameAllocator::new(
                Range::from(
                    PhysicalPageNumber::ceil(PhysicalAddress::from(*KERNEL_END_ADDRESS))
                    ..PhysicalPageNumber::floor(MEMORY_END_ADDRESS)
                )));
}

