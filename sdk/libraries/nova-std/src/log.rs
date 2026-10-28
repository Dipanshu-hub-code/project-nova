use crate::NovaError;

pub fn write_log(_message: &str) -> Result<(), NovaError> {
    Err(NovaError::NotSupported)
}
