//! Firmware loading is unsupported until verified storage and signature checks exist.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirmwareError {
    LoadingUnavailable,
}

pub fn load_firmware(_device: &str, _path: &str) -> Result<(), FirmwareError> {
    Err(FirmwareError::LoadingUnavailable)
}
