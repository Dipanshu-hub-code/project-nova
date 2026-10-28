//! Pure permission-policy helpers; there is no filesystem to enforce them.

use crate::security::{accounts::UserId, privilege::PrivilegeContext};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FsPermission {
    Read = 0b100,
    Write = 0b010,
    Execute = 0b001,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionError {
    InvalidMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileAccess {
    owner: UserId,
    owner_mode: u8,
    other_mode: u8,
}

impl FileAccess {
    pub fn new(owner: UserId, owner_mode: u8, other_mode: u8) -> Result<Self, PermissionError> {
        if owner_mode & !0b111 != 0 || other_mode & !0b111 != 0 {
            return Err(PermissionError::InvalidMode);
        }
        Ok(Self {
            owner,
            owner_mode,
            other_mode,
        })
    }

    pub const fn owner(&self) -> UserId {
        self.owner
    }

    pub fn check(&self, context: PrivilegeContext, permission: FsPermission) -> bool {
        if context.is_administrator() {
            return true;
        }
        let mode = if context.uid() == self.owner {
            self.owner_mode
        } else {
            self.other_mode
        };
        mode & permission as u8 != 0
    }
}
