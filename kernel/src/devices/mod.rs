//! Bounded device metadata and PCI configuration parsing.

pub mod pci;
pub mod registry;
pub mod resource;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceSubsystemStatus {
    MetadataAvailable,
    HardwareDiscoveryUnavailable,
}

pub const fn init() -> DeviceSubsystemStatus {
    DeviceSubsystemStatus::HardwareDiscoveryUnavailable
}
