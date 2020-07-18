use alloc::collections::btree_map::BTreeMap;

use device_tree::{Node, util::SliceRead};
use spin::RwLock;
use virtio_drivers::{DeviceType, VirtIOHeader};

use lazy_static::lazy_static;

use crate::memory::{
    address::{PhysicalAddress, VirtualAddress},
    config::PAGE_SIZE,
    frame::{allocator::FRAME_ALLOCATOR, frame_tracker::FrameTracker},
    mapping::mapping::Mapping,
};

use super::super::block::virtio_blk;

pub fn virtio_probe(node: &Node) {
    let reg = match node.prop_raw("reg") {
        Some(reg) => reg,
        _ => return,
    };
    let pa = PhysicalAddress(reg.as_slice().read_be_u64(0).unwrap() as usize);
    let va = VirtualAddress::from(pa);
    let header = unsafe { &mut *(va.0 as *mut VirtIOHeader) };
    if !header.verify() {
        return;
    }
    match header.device_type() {
        DeviceType::Block => virtio_blk::add_driver(header),
        device => println!("unrecognized virtio device: {:?}", device)
    }
}

lazy_static! {
    pub static ref TRACKERS: RwLock<BTreeMap<PhysicalAddress, FrameTracker>> = RwLock::new(BTreeMap::new());
}

#[no_mangle]
extern "C" fn virtio_dma_alloc(page_num: usize) -> PhysicalAddress {
    let mut pa: PhysicalAddress = Default::default();
    let mut last: PhysicalAddress = Default::default();
    for i in 0..page_num {
        let tracker: FrameTracker = FRAME_ALLOCATOR.lock().alloc().unwrap();
        if i == 0 {
            pa = tracker.address();
        } else {
            assert_eq!(last + PAGE_SIZE, tracker.address()); // 确保分配的物理页连续
        }
        last = tracker.address();
        TRACKERS.write().insert(last, tracker);
    }
    pa
}

#[no_mangle]
extern "C" fn virtio_dma_dealloc(pa: PhysicalAddress, page_num: usize) -> i32 {
    for i in 0..page_num {
        TRACKERS.write().remove(&(pa + i * PAGE_SIZE));
    }
    0
}

#[no_mangle]
extern "C" fn virtio_phys_to_virt(pa: PhysicalAddress) -> VirtualAddress {
    VirtualAddress::from(pa)
}

#[no_mangle]
extern "C" fn virtio_virt_to_phys(va: VirtualAddress) -> PhysicalAddress {
    Mapping::lookup(None, va).unwrap().0
}