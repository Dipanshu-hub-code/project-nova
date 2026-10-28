//! Filesystem API declarations; no kernel filesystem syscall is available.

use crate::NovaError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct File;

pub fn open(_path: &str) -> Result<File, NovaError> {
    Err(NovaError::NotSupported)
}

pub fn read(_file: &mut File, _buffer: &mut [u8]) -> Result<usize, NovaError> {
    Err(NovaError::NotSupported)
}

pub fn write(_file: &mut File, _buffer: &[u8]) -> Result<usize, NovaError> {
    Err(NovaError::NotSupported)
}

pub fn close(_file: File) -> Result<(), NovaError> {
    Err(NovaError::NotSupported)
}
