#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum NovaError {
    InvalidArgument,
    NotSupported,
    PermissionDenied,
    NotFound,
    IoError,
    NoMemory,
    Busy,
    Interrupted,
}
