use crate::NovaError;

pub fn uptime_ms() -> Result<u64, NovaError> {
    Err(NovaError::NotSupported)
}
