use super::bus::virtio_mimo::virtio_probe;
use core::slice;
use device_tree::{DeviceTree, Node};
use crate::memory::address::VirtualAddress;

const DEVICE_TREE_MAGIC: u32 = 0xd00d_feed;

fn walk(node: &Node) {
    if let Ok(compatible) = node.prop_str("compatible") {
        if compatible == "virtio,mmio" {
            virtio_probe(node)
        }
    }

    for child in node.children.iter() {
        walk(child);
    }
}

struct DtbHeader {
    magic: u32,
    size: u32,
}

pub fn init(dtb_va: VirtualAddress) {
    let header = unsafe { &*(dtb_va.0 as *const DtbHeader) };
    let magic = u32::from_be(header.magic);
    if magic == DEVICE_TREE_MAGIC {
        let size = u32::from_be(header.size);
        let data = unsafe { slice::from_raw_parts(dtb_va.0 as *const u8, size as usize) };
        if let Ok(dt) = DeviceTree::load(data) {
            walk(&dt.root);
        }
    }
}