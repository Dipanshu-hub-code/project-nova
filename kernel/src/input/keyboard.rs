//! Keyboard polling API; no keyboard hardware driver is installed yet.

use super::events::KeyEvent;
use super::DriverStatus;

pub const fn init() -> DriverStatus {
    DriverStatus::Unavailable
}

pub const fn poll_key() -> Option<KeyEvent> {
    None
}
