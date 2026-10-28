use nova_std::NovaError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    text: String,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Result<Self, NovaError> {
        let text = text.into();
        if text.len() > 4096 {
            return Err(NovaError::InvalidArgument);
        }
        Ok(Self { text })
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
