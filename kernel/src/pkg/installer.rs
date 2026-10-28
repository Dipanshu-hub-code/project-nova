//! Installation boundary. Package installation is intentionally unavailable.

use crate::pkg::metadata::PackageManifest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionState {
    Planned,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallError {
    FilesystemUnavailable,
    SignatureVerificationUnavailable,
    NetworkUnavailable,
}

pub struct Installer {
    state: TransactionState,
}

impl Installer {
    pub const fn new() -> Self {
        Self {
            state: TransactionState::Planned,
        }
    }

    pub const fn state(&self) -> TransactionState {
        self.state
    }

    pub fn install(&mut self, _manifest: &PackageManifest) -> Result<(), InstallError> {
        self.state = TransactionState::Unavailable;
        Err(InstallError::FilesystemUnavailable)
    }
}

impl Default for Installer {
    fn default() -> Self {
        Self::new()
    }
}
