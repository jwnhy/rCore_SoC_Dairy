pub mod config;
pub mod inode_ext;
pub mod stdin;
pub mod stdout;
use lazy_static::lazy_static;
use rcore_fs::vfs::FsError;

pub type FsResult<T> = Result<T, FsError>;