//! Driver interfaces only; no PCI access or device drivers are active yet.

pub mod driver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverSubsystemStatus {
    InterfacesAvailable,
    DriversUnavailable,
}

pub const fn init() -> DriverSubsystemStatus {
    DriverSubsystemStatus::DriversUnavailable
}
