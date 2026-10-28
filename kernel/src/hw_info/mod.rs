//! Hardware summary built from actual registry entries; no device scan is run.

use crate::devices::registry::DeviceRegistry;

pub struct HardwareInfo<'a> {
    registry: &'a DeviceRegistry,
}

impl<'a> HardwareInfo<'a> {
    pub const fn new(registry: &'a DeviceRegistry) -> Self {
        Self { registry }
    }

    pub const fn device_count(&self) -> usize {
        self.registry.len()
    }
}
