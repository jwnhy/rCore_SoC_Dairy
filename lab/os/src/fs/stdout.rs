use lazy_static::lazy_static;
use rcore_fs::vfs::{INode, FsError, PollStatus};
use crate::fs::FsResult;
use bitflags::_core::any::Any;

lazy_static!{
    pub static ref STDOUT: Arc<Stdout> = Default::default();
}

#[derive(Default)]
pub struct Stdout;

impl INode for Stdout {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> FsResult<usize> {
        Err(FsError::NotSupported)
    }

    fn write_at(&self, offset: usize, buf: &[u8]) -> FsResult<usize> {
        if offset != 0 {
            Err(FsError::NotSupported)
        } else if let Ok(string) = core::str::from_utf8(buf) {
            print!("{}", string)
        } else {
            Err(FsError::InvalidParam)
        }
    }

    fn poll(&self) -> FsResult<PollStatus> {
        Err(FsError::NotSupported)
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}