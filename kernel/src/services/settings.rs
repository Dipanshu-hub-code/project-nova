//! In-memory settings. Persistence is deferred until filesystem integration.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Settings {
    theme: Theme,
}

impl Settings {
    pub const fn new() -> Self {
        Self {
            theme: Theme::Light,
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    pub const fn theme(&self) -> Theme {
        self.theme
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
