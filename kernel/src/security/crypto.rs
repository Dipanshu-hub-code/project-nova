//! No cryptographic primitive is enabled without an audited library and RNG.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoError {
    SecureRandomUnavailable,
    ConstantTimePrimitiveUnavailable,
}

pub fn secure_random_fill(_buffer: &mut [u8]) -> Result<(), CryptoError> {
    Err(CryptoError::SecureRandomUnavailable)
}

pub fn constant_time_compare(_left: &[u8], _right: &[u8]) -> Result<bool, CryptoError> {
    Err(CryptoError::ConstantTimePrimitiveUnavailable)
}
