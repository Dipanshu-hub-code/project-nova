//! Pure PCI configuration parser and callback-based bus scanner.
//!
//! Reading PCI configuration space requires platform I/O support and is not
//! performed by this module.

use crate::devices::registry::DeviceEntry;

pub const PCI_CONFIG_SIZE: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub revision_id: u8,
}

impl PciDevice {
    pub fn from_config(
        bus: u8,
        device: u8,
        function: u8,
        config: &[u8; PCI_CONFIG_SIZE],
    ) -> Option<Self> {
        if device >= 32 || function >= 8 {
            return None;
        }
        let vendor_id = u16::from_le_bytes([config[0], config[1]]);
        if vendor_id == 0 || vendor_id == u16::MAX {
            return None;
        }

        Some(Self {
            bus,
            device,
            function,
            vendor_id,
            device_id: u16::from_le_bytes([config[2], config[3]]),
            revision_id: config[0x08],
            prog_if: config[0x09],
            subclass: config[0x0a],
            class_code: config[0x0b],
        })
    }

    pub const fn to_registry_entry(self) -> DeviceEntry {
        DeviceEntry::new(
            self.vendor_id,
            self.device_id,
            self.class_code,
            self.subclass,
        )
    }
}

pub fn enumerate_bus<F, C>(bus: u8, mut read_config: F, mut on_device: C)
where
    F: FnMut(u8, u8, u8, &mut [u8; PCI_CONFIG_SIZE]) -> bool,
    C: FnMut(PciDevice),
{
    for device in 0..32_u8 {
        let mut config = [0; PCI_CONFIG_SIZE];
        if !read_config(bus, device, 0, &mut config) {
            continue;
        }
        let Some(function_zero) = PciDevice::from_config(bus, device, 0, &config) else {
            continue;
        };
        let multifunction = config[0x0e] & 0x80 != 0;
        on_device(function_zero);

        if multifunction {
            for function in 1..8_u8 {
                let mut config = [0; PCI_CONFIG_SIZE];
                if read_config(bus, device, function, &mut config) {
                    if let Some(pci_device) = PciDevice::from_config(bus, device, function, &config)
                    {
                        on_device(pci_device);
                    }
                }
            }
        }
    }
}
