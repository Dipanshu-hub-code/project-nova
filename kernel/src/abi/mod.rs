//! Versioned application ABI definitions.
//!
//! These definitions are public, but no syscall entry point currently
//! dispatches them. Do not treat this as an operational syscall ABI.

pub const SYSCALL_API_VERSION: u32 = 1;
pub const SYSCALL_COUNT: u32 = 13;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Syscall {
    WriteLog = 0,
    GetProcessInfo = 1,
    OpenFile = 2,
    ReadFile = 3,
    WriteFile = 4,
    CloseFile = 5,
    ListDirectory = 6,
    GetTime = 7,
    CreateWindow = 8,
    DrawRect = 9,
    SendIpc = 10,
    ReceiveIpc = 11,
    Exit = 12,
}

impl Syscall {
    pub const fn from_number(number: u32) -> Option<Self> {
        match number {
            0 => Some(Self::WriteLog),
            1 => Some(Self::GetProcessInfo),
            2 => Some(Self::OpenFile),
            3 => Some(Self::ReadFile),
            4 => Some(Self::WriteFile),
            5 => Some(Self::CloseFile),
            6 => Some(Self::ListDirectory),
            7 => Some(Self::GetTime),
            8 => Some(Self::CreateWindow),
            9 => Some(Self::DrawRect),
            10 => Some(Self::SendIpc),
            11 => Some(Self::ReceiveIpc),
            12 => Some(Self::Exit),
            _ => None,
        }
    }

    pub const fn number(self) -> u32 {
        self as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SyscallError {
    InvalidArgument = 1,
    NotSupported = 2,
    PermissionDenied = 3,
    NotFound = 4,
    IoError = 5,
    NoMemory = 6,
    Busy = 7,
    Interrupted = 8,
}

pub const fn is_valid_syscall(number: u32) -> bool {
    Syscall::from_number(number).is_some()
}
