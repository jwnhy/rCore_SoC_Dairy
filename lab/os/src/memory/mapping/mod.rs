use alloc::vec;
use alloc::vec::Vec;

pub use page_table_entry::Flags;

use crate::memory::address::{VirtualAddress};
use crate::memory::config::{KERNEL_END_ADDRESS, MEMORY_END_ADDRESS};
use crate::memory::mapping::mapping::Mapping;
use crate::memory::mapping::memory_set::MemorySet;
use crate::memory::mapping::segment::{MapType, Segment};
use crate::memory::MemoryResult;
use crate::memory::range::Range;

pub mod memory_set;
pub mod mapping;
mod page_table;
mod page_table_entry;
pub(crate) mod segment;

pub fn new_kernel() -> MemoryResult<MemorySet> {
    extern "C" {
        fn text_start();
        fn rodata_start();
        fn data_start();
        fn bss_start();
    }
    let segments = vec![
        Segment {
            map_type: MapType::Linear,
            range: Range::<VirtualAddress>::from(
                (text_start as usize)..(rodata_start as usize),
            ).into(),
            flags: Flags::VALID | Flags::READABLE | Flags::EXECUTABLE,
        },
        Segment {
            map_type: MapType::Linear,
            range: Range::<VirtualAddress>::from(
                (rodata_start as usize)..(data_start as usize),
            )
                .into(),
            flags: Flags::VALID | Flags::READABLE,
        },
        Segment {
            map_type: MapType::Linear,
            range: Range::<VirtualAddress>::from(
                (data_start as usize)..(bss_start as usize),
            )
                .into(),
            flags: Flags::VALID | Flags::READABLE | Flags::WRITABLE,
        },
        Segment {
            map_type: MapType::Linear,
            range: Range::from(
                VirtualAddress::from(bss_start as usize)..*KERNEL_END_ADDRESS,
            ),
            flags: Flags::VALID | Flags::READABLE | Flags::WRITABLE,
        },
        Segment {
            map_type: MapType::Linear,
            range: Range::from(
                *KERNEL_END_ADDRESS..VirtualAddress::from(MEMORY_END_ADDRESS),
            ),
            flags: Flags::VALID | Flags::READABLE | Flags::WRITABLE,
        },
    ];

    let mut mapping = Mapping::new()?;
    let mut allocated_pairs = Vec::new();
    let mut memory_set = MemorySet { mapping, segments, allocated_pairs };
    memory_set.map()?;
    Ok(memory_set)
}