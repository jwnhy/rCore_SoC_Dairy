use crate::memory::mapping::page_table_entry::PageTableEntry;
use crate::memory::config::{PAGE_SIZE, PAGE_ENTRY_SIZE};

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

