//! Bounded desktop-service foundations; persistence and process integration
//! are not implemented.

pub mod app_manager;
pub mod logging;
pub mod notification;
pub mod session;
pub mod settings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServicesStatus {
    InMemoryServicesAvailable,
    PersistenceUnavailable,
}

pub const fn init() -> ServicesStatus {
    ServicesStatus::PersistenceUnavailable
}
