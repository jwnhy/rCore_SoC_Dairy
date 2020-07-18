use alloc::{sync::Arc, vec::Vec};
use lazy_static::lazy_static;
use spin::RwLock;

#[derive(Debug, Eq, PartialEq)]
pub enum DeviceType {
    Block,
}

pub trait Driver: Send + Sync {
    fn device_type(&self) -> DeviceType;

    fn read_block(&self, _block_id: usize, _buf: &mut [u8]) -> bool {
        unimplemented!("not a block driver")
    }

    fn write_block(&self, _block_id: usize, _buf: &[u8]) -> bool {
        unimplemented!("not a block driver")
    }
}

lazy_static! {
    // 存储所有驱动的地方
    pub static ref DRIVERS: RwLock<Vec<Arc<dyn Driver>>> = RwLock::new(Vec::new());
}