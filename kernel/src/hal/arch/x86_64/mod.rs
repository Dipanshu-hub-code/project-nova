//! x86_64-specific hardware services not yet connected to the boot path.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchStatus {
    InitializationUnavailable,
}

pub const fn init() -> ArchStatus {
    ArchStatus::InitializationUnavailable
}
