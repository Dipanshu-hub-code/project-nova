//! Application API placeholders. Process loading and userspace are not
//! implemented, so these types do not launch executable applications.

pub mod file_manager;
pub mod settings;
pub mod terminal;
pub mod text_editor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationStatus {
    FrameworkOnly,
}

pub const fn init() -> ApplicationStatus {
    ApplicationStatus::FrameworkOnly
}
