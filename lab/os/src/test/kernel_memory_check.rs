use crate::memory::mapping::memory_set::MemorySet;
use crate::memory::mapping::mapping::Mapping;
use crate::memory::address::{PhysicalAddress, VirtualAddress};
use crate::memory::mapping::Flags;
use crate::println;

pub fn kernel_memory_check(memory_set: &MemorySet) {
    for segment in &memory_set.segments {
        let flags = segment.flags;
        for vpn in segment.page_range.iter() {
            let va = VirtualAddress::from(vpn);
            let (pa, entry) = Mapping::lookup(Some(memory_set.mapping.root_ppn.0), va).unwrap();
            assert_eq!(pa, PhysicalAddress::from(va));
            assert_eq!(entry.flags(), flags);
        }
    }
    let mut sp = 0;
    unsafe {
        llvm_asm!("mv $0, sp":"=r"(sp):::);
        let (_, entry) = Mapping::lookup(Some(memory_set.mapping.root_ppn.0), VirtualAddress(sp)).unwrap();
        assert_eq!(entry.flags(), Flags::VALID|Flags::READABLE|Flags::WRITABLE);
    }
}