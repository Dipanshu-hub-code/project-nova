//! Authenticated encryption is unavailable; this module never encrypts with a stub.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionError {
    AuthenticatedEncryptionUnavailable,
}

pub fn encrypt(_plaintext: &[u8], _key: &[u8]) -> Result<(), EncryptionError> {
    Err(EncryptionError::AuthenticatedEncryptionUnavailable)
}

pub fn decrypt(_ciphertext: &[u8], _key: &[u8]) -> Result<(), EncryptionError> {
    Err(EncryptionError::AuthenticatedEncryptionUnavailable)
}
