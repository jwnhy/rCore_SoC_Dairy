use lazy_static::lazy_static;
use rcore_fs::dev::BlockDevice;
use rcore_fs::vfs::FsError;
use rcore_fs::vfs::INode;
use rcore_fs_sfs::SimpleFileSystem;

pub use inode_ext::INodeExt;
pub use stdin::STDIN;
pub use stdout::STDOUT;

use crate::fs::config::BLOCK_CACHE_CAPACITY;

pub mod config;
pub mod inode_ext;
pub mod stdin;
pub mod stdout;

pub type FsResult<T> = Result<T, FsError>;

lazy_static! {
    pub static ref ROOT_INODE: Arc<dyn INode> = {
        for driver in DRIVERS.read().iter() {
            if driver.device_type() == DeviceType::Block {
                let device = BlockDevice(driver.clone());
                let device_with_cache = Arc::new(BlockCache::new(device, BLOCK_CACHE_CAPACITY));
                return SimpleFileSystem::open(device_with_cache)
                    .expect("failed to open SFS")
                    .root_inode();
            }
        }
        panic!("failed to load fs")
    };
}

pub fn init() {
    ROOT_INODE.ls();
    println!("mod fs initialized");
}