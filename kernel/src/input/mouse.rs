//! Mouse polling API; no mouse hardware driver is installed yet.

use super::events::MouseEvent;
use super::DriverStatus;

pub const fn init() -> DriverStatus {
    DriverStatus::Unavailable
}

pub const fn poll_mouse() -> Option<MouseEvent> {
    None
}
