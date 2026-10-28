//! Minimal lifecycle interface for future device drivers.

use crate::devices::registry::{DeviceEntry, DeviceId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    NoMatchingDevice,
    InitializationFailed,
    ResourceUnavailable,
    UnsupportedOperation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverState {
    Unbound,
    Bound,
    Active,
    Failed,
}

pub trait Driver {
    fn name(&self) -> &'static str;
    fn matches(&self, device: &DeviceEntry) -> bool;
    fn init(&mut self, device_id: DeviceId) -> Result<(), DriverError>;
    fn state(&self) -> DriverState;
}
