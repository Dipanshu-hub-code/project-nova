//! Hardware abstraction foundations.
//!
//! Interrupt routing, timers, and MMIO mapping require platform support that
//! is not available in the current kernel.

pub mod arch;
pub mod interrupts;
pub mod memory;
pub mod time;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HalStatus {
    FoundationsAvailable,
    HardwareServicesUnavailable,
}

pub const fn init() -> HalStatus {
    HalStatus::HardwareServicesUnavailable
}
