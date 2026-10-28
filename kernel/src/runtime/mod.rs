//! Bounded application metadata and registry foundations.
//!
//! This module does not load executables or create user-mode processes.

pub mod launcher;
pub mod manifest;
pub mod registry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeStatus {
    RegistryAvailable,
    ProcessExecutionUnavailable,
}

pub const fn init() -> RuntimeStatus {
    RuntimeStatus::ProcessExecutionUnavailable
}
