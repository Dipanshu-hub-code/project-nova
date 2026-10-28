//! Validation-only syscall dispatcher; no syscall entry or user memory access.

use crate::abi::{Syscall, SyscallError, SYSCALL_API_VERSION};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyscallContext {
    pub api_version: u32,
    pub syscall_number: u32,
    pub arguments: [usize; 6],
}

impl SyscallContext {
    pub const fn new(api_version: u32, syscall_number: u32, arguments: [usize; 6]) -> Self {
        Self {
            api_version,
            syscall_number,
            arguments,
        }
    }

    pub const fn syscall(&self) -> Option<Syscall> {
        Syscall::from_number(self.syscall_number)
    }
}

pub fn dispatch(context: &SyscallContext) -> Result<usize, SyscallError> {
    if context.api_version != SYSCALL_API_VERSION || context.syscall().is_none() {
        return Err(SyscallError::InvalidArgument);
    }

    Err(SyscallError::NotSupported)
}
