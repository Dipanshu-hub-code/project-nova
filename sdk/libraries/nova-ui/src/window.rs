use nova_std::NovaError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    title: String,
}

impl Window {
    pub fn new(title: impl Into<String>) -> Result<Self, NovaError> {
        let title = title.into();
        if title.trim().is_empty() || title.len() > 256 {
            return Err(NovaError::InvalidArgument);
        }
        Ok(Self { title })
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn show(&self) -> Result<(), NovaError> {
        Err(NovaError::NotSupported)
    }
}
