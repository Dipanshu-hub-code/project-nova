//! Session lifecycle API. Starting a desktop session requires input, display,
//! and application execution backends that are not available yet.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionError {
    DesktopBackendsUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Session {
    active: bool,
}

impl Session {
    pub const fn new() -> Self {
        Self { active: false }
    }

    pub const fn is_active(&self) -> bool {
        self.active
    }

    pub fn start(&mut self) -> Result<(), SessionError> {
        Err(SessionError::DesktopBackendsUnavailable)
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}
