//! Security policy foundations.
//!
//! Account data is caller-owned and volatile. Authentication, privilege
//! enforcement, cryptography, integrity verification, and recovery remain
//! unavailable until the required kernel and storage services exist.

pub mod accounts;
pub mod auth;
pub mod crypto;
pub mod encryption;
pub mod fs_perms;
pub mod integrity;
pub mod logging;
pub mod privilege;
pub mod recovery;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityStatus {
    PolicyFoundationsAvailable,
    AuthenticationUnavailable,
    CryptographyUnavailable,
    EnforcementUnavailable,
}

pub const fn init() -> SecurityStatus {
    SecurityStatus::PolicyFoundationsAvailable
}
