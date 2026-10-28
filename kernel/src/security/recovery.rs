//! Recovery actions remain unavailable without verified storage and boot paths.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryError {
    RecoveryUnavailable,
}

pub fn enter_recovery_mode() -> Result<(), RecoveryError> {
    Err(RecoveryError::RecoveryUnavailable)
}
