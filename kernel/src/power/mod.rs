//! Power controls are unavailable until ACPI tables and hardware methods exist.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerError {
    AcpiUnavailable,
}

pub fn shutdown() -> Result<(), PowerError> {
    Err(PowerError::AcpiUnavailable)
}

pub fn reboot() -> Result<(), PowerError> {
    Err(PowerError::AcpiUnavailable)
}
