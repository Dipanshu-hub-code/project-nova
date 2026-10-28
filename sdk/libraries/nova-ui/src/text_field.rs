//! UTF-8-safe single-line text editing state.

use nova_std::NovaError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextFieldError {
    MissingAccessibleLabel,
    InvalidMaximumLength,
    InvalidValue,
    ValueTooLong,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextField {
    accessible_label: String,
    value: String,
    cursor: usize,
    maximum_bytes: usize,
    enabled: bool,
}

impl TextField {
    pub fn new(
        accessible_label: impl Into<String>,
        maximum_bytes: usize,
    ) -> Result<Self, TextFieldError> {
        let accessible_label = accessible_label.into();
        if accessible_label.trim().is_empty() {
            return Err(TextFieldError::MissingAccessibleLabel);
        }
        if maximum_bytes == 0 {
            return Err(TextFieldError::InvalidMaximumLength);
        }
        Ok(Self {
            accessible_label,
            value: String::new(),
            cursor: 0,
            maximum_bytes,
            enabled: true,
        })
    }

    pub fn with_value(
        accessible_label: impl Into<String>,
        value: impl Into<String>,
        maximum_bytes: usize,
    ) -> Result<Self, TextFieldError> {
        let mut field = Self::new(accessible_label, maximum_bytes)?;
        let value = value.into();
        if value.len() > maximum_bytes {
            return Err(TextFieldError::ValueTooLong);
        }
        if value.chars().any(char::is_control) {
            return Err(TextFieldError::InvalidValue);
        }
        field.cursor = value.len();
        field.value = value;
        Ok(field)
    }

    pub fn accessible_label(&self) -> &str {
        &self.accessible_label
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub const fn cursor_byte_offset(&self) -> usize {
        self.cursor
    }

    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn move_left(&mut self) {
        if self.enabled {
            self.cursor = self.value[..self.cursor]
                .char_indices()
                .next_back()
                .map_or(0, |(index, _)| index);
        }
    }

    pub fn move_right(&mut self) {
        if self.enabled {
            self.cursor += self.value[self.cursor..]
                .chars()
                .next()
                .map_or(0, char::len_utf8);
        }
    }

    pub fn move_home(&mut self) {
        if self.enabled {
            self.cursor = 0;
        }
    }

    pub fn move_end(&mut self) {
        if self.enabled {
            self.cursor = self.value.len();
        }
    }

    pub fn insert(&mut self, text: &str) -> Result<(), NovaError> {
        if !self.enabled {
            return Err(NovaError::PermissionDenied);
        }
        if text.chars().any(char::is_control)
            || self.value.len().saturating_add(text.len()) > self.maximum_bytes
        {
            return Err(NovaError::InvalidArgument);
        }
        self.value.insert_str(self.cursor, text);
        self.cursor += text.len();
        Ok(())
    }

    pub fn backspace(&mut self) -> bool {
        if !self.enabled || self.cursor == 0 {
            return false;
        }
        let previous = self.value[..self.cursor]
            .char_indices()
            .next_back()
            .map_or(self.cursor, |(index, _)| index);
        self.value.replace_range(previous..self.cursor, "");
        self.cursor = previous;
        true
    }

    pub fn delete_forward(&mut self) -> bool {
        if !self.enabled || self.cursor == self.value.len() {
            return false;
        }
        let end = self.cursor
            + self.value[self.cursor..]
                .chars()
                .next()
                .map_or(0, char::len_utf8);
        self.value.replace_range(self.cursor..end, "");
        true
    }

    pub fn clear(&mut self) -> Result<(), NovaError> {
        if !self.enabled {
            return Err(NovaError::PermissionDenied);
        }
        self.value.clear();
        self.cursor = 0;
        Ok(())
    }
}
