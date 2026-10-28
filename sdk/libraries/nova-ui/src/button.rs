use nova_std::NovaError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Destructive,
    Quiet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Button {
    label: String,
    variant: ButtonVariant,
    enabled: bool,
    focused: bool,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Result<Self, NovaError> {
        let label = label.into();
        if label.trim().is_empty() {
            return Err(NovaError::InvalidArgument);
        }
        if label.len() > 128 {
            return Err(NovaError::InvalidArgument);
        }
        Ok(Self {
            label,
            variant: ButtonVariant::Primary,
            enabled: true,
            focused: false,
        })
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub const fn variant(&self) -> ButtonVariant {
        self.variant
    }

    pub fn set_variant(&mut self, variant: ButtonVariant) {
        self.variant = variant;
    }

    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.focused = false;
        }
    }

    pub const fn is_focused(&self) -> bool {
        self.focused
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused && self.enabled;
    }

    pub fn click(&self) -> bool {
        self.enabled
    }

    pub fn activate_by_keyboard(&self, key: ActivationKey) -> bool {
        self.enabled && self.focused && matches!(key, ActivationKey::Enter | ActivationKey::Space)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationKey {
    Enter,
    Space,
    Other,
}
