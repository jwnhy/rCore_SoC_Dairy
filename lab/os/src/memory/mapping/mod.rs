mod memory_set;
pub mod mapping;
mod page_table;
mod page_table_entry;
pub(crate) mod segment;

pub use page_table_entry::Flags;

use crate::memory::MemoryResult;
use crate::memory::mapping::memory_set::MemorySet;
use crate::memory::mapping::segment::{MapType, Segment};
use crate::memory::range::Range;
use crate::memory::address::{VirtualAddress, VirtualPageNumber};
use crate::memory::config::{MEMORY_END_ADDRESS, KERNEL_END_ADDRESS};
use crate::memory::mapping::mapping::Mapping;
use crate::memory::frame::frame_tracker::FrameTracker;
use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::vec;


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
            page_range: Range::<VirtualAddress>::from(
                (text_start as usize)..(rodata_start as usize),
            ).into(),
            flags: Flags::VALID | Flags::READABLE | Flags::EXECUTABLE,
        },
        Segment {
            map_type: MapType::Linear,
            page_range: Range::<VirtualAddress>::from(
                (rodata_start as usize)..(data_start as usize),
            )
                .into(),
            flags: Flags::VALID | Flags::READABLE,
        },
        Segment {
            map_type: MapType::Linear,
            page_range: Range::<VirtualAddress>::from(
                (data_start as usize)..(bss_start as usize),
            )
                .into(),
            flags: Flags::VALID | Flags::READABLE | Flags::WRITABLE,
        },
        Segment {
            map_type: MapType::Linear,
            page_range: Range::from(
                VirtualAddress::from(bss_start as usize)..*KERNEL_END_ADDRESS,
            ),
            flags: Flags::VALID | Flags::READABLE | Flags::WRITABLE,
        },
        Segment {
            map_type: MapType::Linear,
            page_range: Range::from(
                *KERNEL_END_ADDRESS..VirtualAddress::from(MEMORY_END_ADDRESS),
            ),
            flags: Flags::VALID | Flags::READABLE | Flags::WRITABLE,
        },
    ];

    let mut mapping = Mapping::new()?;
    let mut allocated_pairs = Vec::new();

    for segment in segments.iter() {
        let new_pair = mapping.map(segment);
        allocated_pairs.extend(new_pair?)
    }
    Ok(MemorySet{mapping, segments, allocated_pairs})
}