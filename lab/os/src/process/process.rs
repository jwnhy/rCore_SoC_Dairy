use crate::memory::mapping::memory_set::MemorySet;
use crate::memory::MemoryResult;
use alloc::sync::Arc;
use spin::RwLock;
use crate::memory::mapping::Flags;
use crate::memory::range::Range;
use crate::memory::address::VirtualAddress;
use crate::memory::config::PAGE_SIZE;
use crate::memory::mapping::segment::{Segment, MapType};
#[derive(Debug)]
pub struct Process {
    pub is_user: bool,
    pub memory_set: MemorySet,
}

#[allow(unused)]
impl Process {
    pub fn new_kernel() -> MemoryResult<Arc<RwLock<Self>>> {
        use crate::memory::mapping::new_kernel;
        Ok(Arc::new(RwLock::new(Self{
            is_user: false,
            memory_set: new_kernel()?
        })))
    }

    pub fn alloc_page_range(
        &mut self,
        size: usize,
        flags: Flags
    ) -> MemoryResult<Range<VirtualAddress>> {
        let alloc_size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        let mut range = Range::<VirtualAddress>::from(0x1000000..0x1000000+alloc_size);
        while self.memory_set.overlap_with(range.into()) {
            range.start += alloc_size;
            range.end += alloc_size;
        }
        self.memory_set.add_segment(Segment
        {
            map_type: MapType::Framed,
            range,
            flags: flags | Flags::user(self.is_user),
        }
        ,None)?;
        Ok(Range::from(range.start..(range.start+size)))
    }
}