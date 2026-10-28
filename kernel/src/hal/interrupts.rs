//! Interrupt installation is not wired to an IDT in this kernel.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptError {
    InterruptControllerUnavailable,
}

pub fn register_handler(_vector: u8, _handler: extern "C" fn()) -> Result<(), InterruptError> {
    Err(InterruptError::InterruptControllerUnavailable)
}
