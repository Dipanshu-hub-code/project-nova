//! Authentication fails closed until a trusted password hasher and RNG exist.

use crate::security::accounts::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthError {
    AuthenticationUnavailable,
}

pub fn authenticate(_username: &str) -> Result<UserId, AuthError> {
    Err(AuthError::AuthenticationUnavailable)
}

pub fn change_password(_uid: UserId) -> Result<(), AuthError> {
    Err(AuthError::AuthenticationUnavailable)
}
