use alloc::collections::VecDeque;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::kernel::condvar::Condvar;
use crate::fs::FsResult;
use rcore_fs::vfs::{INode, PollStatus, FsError};
use bitflags::_core::any::Any;

lazy_static!{
    pub static ref STDIN: Arc<Stdin> = Default::default();
}

#[derive(Default)]
pub struct Stdin {
    buffer: Mutex<VecDeque<u8>>,
    condvar: Condvar
}

impl INode for Stdin {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> FsResult<usize> {
        if offset != 0 {
            Err(FsError::NotSupported)
        } else if self.buffer.lock().len() == 0 {
            self.condvar.wait();
            Ok(0)
        } else {
            let mut stdin_buffer = self.buffer.lock();
            for (i, byte) in buf.iter_mut().enumerate() {
                if let Some(b) = stdin_buffer.pop_front() {
                    *byte = b
                } else {
                    return Ok(i)
                }
            }
            Ok(buf.len())
        }
    }

    fn write_at(&self, offset: usize, buf: &[u8]) -> FsResult<usize> {
        Err(FsError::NotSupported)
    }

    fn poll(&self) -> FsResult<PollStatus> {
        Err(FsError::NotSupported)
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}