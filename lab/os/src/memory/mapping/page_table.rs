use crate::memory::mapping::page_table_entry::PageTableEntry;
use crate::memory::config::{PAGE_SIZE, PAGE_ENTRY_SIZE};
use crate::memory::frame::frame_tracker::FrameTracker;
use crate::memory::address::PhysicalPageNumber;

#[repr(C)]
pub struct PageTable {
    // PageTableSize = PageEntrySize = 4 KiB = 512 * 8 Byte
    pub entries: [PageTableEntry; PAGE_SIZE / PAGE_ENTRY_SIZE]
}

impl PageTable {
    pub fn zero_init(&mut self) {
        self.entries = [Default::default(); PAGE_SIZE / PAGE_ENTRY_SIZE]
    }
}
#[derive(Debug)]
pub struct PageTableTracker(pub FrameTracker);

impl PageTableTracker {
    pub fn new(frame: FrameTracker) -> Self {
        let mut page_table = Self(frame);
        /// Here we use `Deref` trait to convert PageTableTracker into PageTable
        page_table.zero_init();
        page_table
    }
    pub fn page_number(&self) -> PhysicalPageNumber {
        self.0.page_number()
    }
}

impl core::ops::Deref for PageTableTracker {
    type Target = PageTable;
    fn deref(&self) -> &Self::Target {
        /// `Physical Address` -> `Virtual Address` -> `Page Table`
        self.0.address().deref_kernel()
    }
}

impl core::ops::DerefMut for PageTableTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.address().deref_kernel()
    }
}

impl PageTableEntry {
    pub fn get_next_table(&self) -> &'static mut PageTable {
        self.address().deref_kernel()
    }
}