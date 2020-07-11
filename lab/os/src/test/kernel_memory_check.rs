use crate::memory::mapping::memory_set::MemorySet;
use crate::memory::mapping::mapping::Mapping;
use crate::memory::address::{PhysicalAddress, VirtualAddress};
use crate::memory::mapping::Flags;
use crate::memory::config::KERNEL_END_ADDRESS;
use crate::println;

pub fn kernel_memory_check(memory_set: &MemorySet) {
    println!("checking memory");
    for (idx, segment) in memory_set.segments[..5].iter().enumerate() {
        let flags = segment.flags;
        for vpn in segment.page_range().iter() {
            let mut va = VirtualAddress::from(vpn);
            let (pa, entry) = Mapping::lookup(Some(memory_set.mapping.root_ppn.0), va).unwrap();
            // println!("{:?} {:?}",entry.flags(), flags);
            // println!("{:x?} {:x?} {:?} {}", va, pa, segment.map_type, idx);
            assert_eq!(pa, PhysicalAddress::from(VirtualAddress::from(vpn)));
            assert_eq!(entry.flags() | flags, entry.flags());
        }
    }
    let mut sp = 0;
    unsafe {
        llvm_asm!("mv $0, sp":"=r"(sp):::);
        let (_, entry) = Mapping::lookup(Some(memory_set.mapping.root_ppn.0), VirtualAddress(sp)).unwrap();
        assert_eq!(entry.flags(), Flags::VALID|Flags::READABLE|Flags::WRITABLE);
    }
    println!("memory checked");
}