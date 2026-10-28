//! Caller-owned bounded registry; no global mutable device table.

pub const MAX_DEVICES: usize = 32;
pub const MAX_DRIVER_NAME: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DeviceState {
    Discovered,
    Identified,
    ResourcesAssigned,
    DriverMatched,
    Initialized,
    Active,
    Failed,
    Unsupported,
    Removed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceId(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryError {
    Full,
    InvalidDevice,
    DriverNameTooLong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceEntry {
    id: DeviceId,
    state: DeviceState,
    vendor_id: u16,
    device_id: u16,
    class_code: u8,
    subclass: u8,
    driver_name: [u8; MAX_DRIVER_NAME],
    driver_name_len: usize,
}

impl DeviceEntry {
    pub const fn new(vendor_id: u16, device_id: u16, class_code: u8, subclass: u8) -> Self {
        Self {
            id: DeviceId(0),
            state: DeviceState::Discovered,
            vendor_id,
            device_id,
            class_code,
            subclass,
            driver_name: [0; MAX_DRIVER_NAME],
            driver_name_len: 0,
        }
    }

    pub const fn id(&self) -> DeviceId {
        self.id
    }

    pub const fn state(&self) -> DeviceState {
        self.state
    }

    pub const fn vendor_id(&self) -> u16 {
        self.vendor_id
    }

    pub const fn device_id(&self) -> u16 {
        self.device_id
    }

    pub const fn class_code(&self) -> u8 {
        self.class_code
    }

    pub const fn subclass(&self) -> u8 {
        self.subclass
    }

    pub fn driver_name(&self) -> &str {
        core::str::from_utf8(&self.driver_name[..self.driver_name_len])
            .expect("driver names are copied from valid UTF-8")
    }
}

pub struct DeviceRegistry {
    entries: [Option<DeviceEntry>; MAX_DEVICES],
    len: usize,
}

impl DeviceRegistry {
    pub const fn new() -> Self {
        Self {
            entries: [None; MAX_DEVICES],
            len: 0,
        }
    }

    pub fn register(&mut self, mut entry: DeviceEntry) -> Result<DeviceId, RegistryError> {
        let index = self
            .entries
            .iter()
            .position(Option::is_none)
            .ok_or(RegistryError::Full)?;
        let id = DeviceId(index as u8);
        entry.id = id;
        self.entries[index] = Some(entry);
        self.len += 1;
        Ok(id)
    }

    pub fn get(&self, id: DeviceId) -> Option<&DeviceEntry> {
        self.entries.get(id.0 as usize).and_then(Option::as_ref)
    }

    pub fn set_state(&mut self, id: DeviceId, state: DeviceState) -> Result<(), RegistryError> {
        let entry = self
            .entries
            .get_mut(id.0 as usize)
            .and_then(Option::as_mut)
            .ok_or(RegistryError::InvalidDevice)?;
        entry.state = state;
        Ok(())
    }

    pub fn set_driver(&mut self, id: DeviceId, name: &str) -> Result<(), RegistryError> {
        if name.len() > MAX_DRIVER_NAME {
            return Err(RegistryError::DriverNameTooLong);
        }
        let entry = self
            .entries
            .get_mut(id.0 as usize)
            .and_then(Option::as_mut)
            .ok_or(RegistryError::InvalidDevice)?;
        entry.driver_name[..name.len()].copy_from_slice(name.as_bytes());
        entry.driver_name_len = name.len();
        Ok(())
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl Default for DeviceRegistry {
    fn default() -> Self {
        Self::new()
    }
}
