//! Process APIs are declared but unavailable until Nova has a process runtime.

use crate::NovaError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessInfo {
    pub pid: u64,
    pub uid: u32,
    pub state: u8,
}

pub fn exit(_code: i32) -> Result<(), NovaError> {
    Err(NovaError::NotSupported)
}

pub fn get_process_info() -> Result<ProcessInfo, NovaError> {
    Err(NovaError::NotSupported)
}
