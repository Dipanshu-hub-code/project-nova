//! Signature verification is unavailable until trusted keys and crypto exist.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityError {
    VerificationUnavailable,
}

pub fn verify_kernel_image(_image: &[u8], _signature: &[u8]) -> Result<(), IntegrityError> {
    Err(IntegrityError::VerificationUnavailable)
}

pub fn verify_package_signature(_package: &[u8], _signature: &[u8]) -> Result<(), IntegrityError> {
    Err(IntegrityError::VerificationUnavailable)
}
