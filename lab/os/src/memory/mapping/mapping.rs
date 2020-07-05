use alloc::vec;
use alloc::vec::Vec;
use crate::memory::address::{PhysicalAddress, PhysicalPageNumber, VirtualAddress, VirtualPageNumber};
use crate::memory::frame::allocator::FRAME_ALLOCATOR;
use crate::memory::frame::frame_tracker::FrameTracker;
use crate::memory::mapping::Flags;
use crate::memory::mapping::page_table::{PageTable, PageTableTracker};
use crate::memory::mapping::page_table_entry::PageTableEntry;
use crate::memory::mapping::segment::Segment;
use crate::memory::MemoryResult;

#[derive(Default, Debug)]
pub struct Mapping {
    page_tables: Vec<PageTableTracker>,
    pub root_ppn: PhysicalPageNumber,
}

impl Mapping {
    pub fn new() -> MemoryResult<Mapping> {
        let root_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
        let root_ppn = root_table.page_number();
        Ok(
            Mapping {
                page_tables: vec![root_table],
                root_ppn,
            }
        )
    }

    pub fn find_entry(&mut self, vpn: VirtualPageNumber) -> MemoryResult<&mut PageTableEntry> {
        let root_table: &mut PageTable = PhysicalAddress::from(self.root_ppn).deref_kernel();
        let mut entry = &mut root_table.entries[vpn.levels()[0]];
        //println!("[{}] = {:x?}", vpn.levels()[0], entry);
        for vpn_part in &vpn.levels()[1..] {
            if entry.is_empty() {
                let new_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
                let new_ppn = new_table.page_number();
                *entry = PageTableEntry::new(new_ppn, Flags::VALID);
                self.page_tables.push(new_table);
            }
            entry = &mut entry.get_next_table().entries[*vpn_part];
        }
        Ok(entry)
    }

    fn map_one(&mut self, vpn: VirtualPageNumber, ppn: PhysicalPageNumber, flags: Flags) -> MemoryResult<()> {
        let entry = self.find_entry(vpn)?;
        assert!(entry.is_empty(), "virtual address is already mapped");
        *entry = PageTableEntry::new(ppn, flags);

        //println!("{:x?}", Self::lookup(Some(self.root_ppn.0),VirtualAddress::from(vpn)).unwrap());
        Ok(())
    }

    pub fn map(&mut self, segment: &Segment) -> MemoryResult<Vec<(VirtualPageNumber, FrameTracker)>> {
        if let Some(ppn_iter) = segment.iter_mapped() {
            for (vpn, ppn) in segment.page_range.iter().zip(ppn_iter) {
                self.map_one(vpn, ppn, segment.flags)?;
            }
            Ok(vec![])
        } else {
            let mut allocated_pairs = vec![];
            for vpn in segment.page_range.iter() {
                let frame: FrameTracker = FRAME_ALLOCATOR.lock().alloc()?;
                self.map_one(vpn, frame.page_number(), segment.flags)?;
                allocated_pairs.push((vpn, frame));
            }
            Ok(allocated_pairs)
        }
    }

    pub fn lookup(root_ppn: Option<usize>, va: VirtualAddress) -> Option<(PhysicalAddress, PageTableEntry)> {
        let mut current_ppn;
        unsafe {
            llvm_asm!("csrr $0, satp" : "=r"(current_ppn) ::: "volatile");
            current_ppn ^= 8 << 60;
        }
        current_ppn = root_ppn.unwrap_or(current_ppn);

        let root_table: &PageTable =
            PhysicalAddress::from(PhysicalPageNumber(current_ppn)).deref_kernel();
        let vpn = VirtualPageNumber::floor(va);
        let mut entry = &root_table.entries[vpn.levels()[0]];
        // 为了支持大页的查找，我们用 length 表示查找到的物理页需要加多少位的偏移
        let mut length = 12 + 2 * 9;
        for vpn_slice in &vpn.levels()[1..] {
            if entry.is_empty() {
                return None;
            }
            if entry.has_next_level() {
                length -= 9;
                entry = &mut entry.get_next_table().entries[*vpn_slice];
            }
        }
        let base = PhysicalAddress::from(entry.page_number()).0;
        let offset = va.0 & ((1 << length) - 1);
        Some((PhysicalAddress(base + offset), *entry))
    }

    pub fn flush(&self) {
        let new_satp = self.root_ppn.0 | (8 << 60);
        unsafe {
            llvm_asm!("csrw satp, $0" :: "r"(new_satp) :: "volatile");
            llvm_asm!("sfence.vma" :::: "volatile");
        }
    }
}