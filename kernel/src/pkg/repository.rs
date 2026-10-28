//! In-memory repository configuration; network fetching is not implemented.

use crate::pkg::metadata::{FixedString, MAX_ID_LEN};

pub const MAX_REPOSITORIES: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryError {
    Full,
    Duplicate,
    InvalidName,
    InvalidUrl,
    NotFound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Repository {
    name: FixedString<MAX_ID_LEN>,
    url: FixedString<256>,
    enabled: bool,
}

impl Repository {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    pub const fn enabled(&self) -> bool {
        self.enabled
    }
}

pub struct RepositorySystem {
    repositories: [Option<Repository>; MAX_REPOSITORIES],
    count: usize,
}

impl RepositorySystem {
    pub const fn new() -> Self {
        Self {
            repositories: [None; MAX_REPOSITORIES],
            count: 0,
        }
    }

    pub fn add(&mut self, name: &str, url: &str) -> Result<(), RepositoryError> {
        if name.is_empty()
            || name.len() > MAX_ID_LEN
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            return Err(RepositoryError::InvalidName);
        }
        let valid_scheme_and_location =
            if let Some(authority_and_path) = url.strip_prefix("https://") {
                let authority = authority_and_path
                    .split(|character| matches!(character, '/' | '?' | '#'))
                    .next()
                    .unwrap_or("");
                !authority.is_empty() && !authority.contains('@')
            } else if let Some(path) = url.strip_prefix("file://") {
                !path.is_empty()
            } else {
                false
            };
        if !valid_scheme_and_location
            || url.len() > 256
            || url
                .bytes()
                .any(|byte| byte.is_ascii_whitespace() || byte.is_ascii_control())
        {
            return Err(RepositoryError::InvalidUrl);
        }
        if self
            .repositories
            .iter()
            .flatten()
            .any(|repository| repository.name() == name)
        {
            return Err(RepositoryError::Duplicate);
        }
        let slot = self
            .repositories
            .iter()
            .position(Option::is_none)
            .ok_or(RepositoryError::Full)?;
        let name = FixedString::parse(name).map_err(|_| RepositoryError::InvalidName)?;
        let url = FixedString::parse(url).map_err(|_| RepositoryError::InvalidUrl)?;
        self.repositories[slot] = Some(Repository {
            name,
            url,
            enabled: true,
        });
        self.count += 1;
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Repository> {
        self.repositories
            .iter()
            .flatten()
            .find(|repository| repository.name() == name)
    }

    pub fn get_enabled_at(&self, index: usize) -> Option<&Repository> {
        self.repositories
            .iter()
            .flatten()
            .filter(|repository| repository.enabled())
            .nth(index)
    }

    pub fn set_enabled(&mut self, name: &str, enabled: bool) -> Result<(), RepositoryError> {
        let repository = self
            .repositories
            .iter_mut()
            .flatten()
            .find(|repository| repository.name() == name)
            .ok_or(RepositoryError::NotFound)?;
        repository.enabled = enabled;
        Ok(())
    }

    pub fn enabled_count(&self) -> usize {
        self.repositories
            .iter()
            .flatten()
            .filter(|repository| repository.enabled())
            .count()
    }

    pub const fn len(&self) -> usize {
        self.count
    }
}

impl Default for RepositorySystem {
    fn default() -> Self {
        Self::new()
    }
}
