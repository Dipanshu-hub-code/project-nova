//! Input API. Device support is intentionally reported as unavailable until
//! keyboard and mouse drivers are implemented.

pub mod events;
pub mod keyboard;
pub mod mouse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverStatus {
    Unavailable,
}

pub fn init() -> DriverStatus {
    DriverStatus::Unavailable
}
