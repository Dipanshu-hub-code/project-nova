//! Allocation-free parser for the project's small application manifest format.

use core::str::FromStr;

pub const MAX_CAPABILITIES: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedText<const N: usize> {
    bytes: [u8; N],
    len: usize,
}

impl<const N: usize> FixedText<N> {
    pub const fn new() -> Self {
        Self {
            bytes: [0; N],
            len: 0,
        }
    }

    pub fn from_str(value: &str) -> Result<Self, ManifestError> {
        if value.len() > N {
            return Err(ManifestError::ValueTooLong);
        }
        let mut text = Self::new();
        text.bytes[..value.len()].copy_from_slice(value.as_bytes());
        text.len = value.len();
        Ok(text)
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.len]).expect("FixedText only stores valid UTF-8")
    }
}

impl<const N: usize> Default for FixedText<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppManifest {
    id: FixedText<64>,
    name: FixedText<128>,
    version: FixedText<32>,
    exec: FixedText<256>,
    icon: FixedText<256>,
    capabilities: [Option<FixedText<32>>; MAX_CAPABILITIES],
    capability_count: usize,
}

impl AppManifest {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn version(&self) -> &str {
        self.version.as_str()
    }

    pub fn executable_path(&self) -> &str {
        self.exec.as_str()
    }

    pub fn icon_path(&self) -> Option<&str> {
        (!self.icon.as_str().is_empty()).then(|| self.icon.as_str())
    }

    pub const fn capability_count(&self) -> usize {
        self.capability_count
    }

    pub fn capability(&self, index: usize) -> Option<&str> {
        self.capabilities
            .get(index)
            .and_then(Option::as_ref)
            .map(FixedText::as_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestError {
    InvalidFormat,
    MissingField,
    ValueTooLong,
    UnknownCapability,
    TooManyCapabilities,
    InvalidPath,
    DuplicateField,
}

impl FromStr for AppManifest {
    type Err = ManifestError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let mut manifest = Self {
            id: FixedText::new(),
            name: FixedText::new(),
            version: FixedText::new(),
            exec: FixedText::new(),
            icon: FixedText::new(),
            capabilities: [None; MAX_CAPABILITIES],
            capability_count: 0,
        };
        let mut seen_id = false;
        let mut seen_name = false;
        let mut seen_version = false;
        let mut seen_exec = false;
        let mut seen_icon = false;
        let mut seen_capabilities = false;

        for line in source.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line == "[application]" || line == "[id]" {
                continue;
            }

            let (key, raw_value) = line.split_once('=').ok_or(ManifestError::InvalidFormat)?;
            let key = key.trim();
            match key {
                "id" => {
                    ensure_first(seen_id)?;
                    let value = parse_nonempty_value(raw_value.trim())?;
                    manifest.id = FixedText::from_str(value)?;
                    seen_id = true;
                }
                "name" => {
                    ensure_first(seen_name)?;
                    let value = parse_nonempty_value(raw_value.trim())?;
                    manifest.name = FixedText::from_str(value)?;
                    seen_name = true;
                }
                "version" => {
                    ensure_first(seen_version)?;
                    let value = parse_nonempty_value(raw_value.trim())?;
                    manifest.version = FixedText::from_str(value)?;
                    seen_version = true;
                }
                "exec" => {
                    ensure_first(seen_exec)?;
                    let value = parse_nonempty_value(raw_value.trim())?;
                    validate_path(value)?;
                    manifest.exec = FixedText::from_str(value)?;
                    seen_exec = true;
                }
                "icon" => {
                    ensure_first(seen_icon)?;
                    let value = parse_nonempty_value(raw_value.trim())?;
                    validate_path(value)?;
                    manifest.icon = FixedText::from_str(value)?;
                    seen_icon = true;
                }
                "caps" => {
                    if seen_capabilities {
                        return Err(ManifestError::DuplicateField);
                    }
                    parse_capabilities(raw_value.trim(), &mut manifest)?;
                    seen_capabilities = true;
                }
                _ => return Err(ManifestError::InvalidFormat),
            }
        }

        if !(seen_id && seen_name && seen_version && seen_exec) {
            return Err(ManifestError::MissingField);
        }
        Ok(manifest)
    }
}

fn ensure_first(seen: bool) -> Result<(), ManifestError> {
    if seen {
        Err(ManifestError::DuplicateField)
    } else {
        Ok(())
    }
}

fn parse_value(raw_value: &str) -> Result<&str, ManifestError> {
    let value = raw_value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .ok_or(ManifestError::InvalidFormat)?;
    if value.contains('"') {
        return Err(ManifestError::InvalidFormat);
    }
    Ok(value)
}

fn parse_nonempty_value(raw_value: &str) -> Result<&str, ManifestError> {
    let value = parse_value(raw_value)?;
    if value.is_empty() {
        return Err(ManifestError::InvalidFormat);
    }
    Ok(value)
}

fn validate_path(path: &str) -> Result<(), ManifestError> {
    if !path.starts_with('/')
        || path
            .split('/')
            .any(|component| component == "." || component == "..")
    {
        return Err(ManifestError::InvalidPath);
    }
    Ok(())
}

fn parse_capabilities(raw_value: &str, manifest: &mut AppManifest) -> Result<(), ManifestError> {
    if manifest.capability_count != 0 {
        return Err(ManifestError::DuplicateField);
    }
    let list = raw_value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .ok_or(ManifestError::InvalidFormat)?;

    for raw_capability in list.split(',') {
        let raw_capability = raw_capability.trim();
        if raw_capability.is_empty() {
            continue;
        }
        let capability = parse_value(raw_capability)?;
        if !matches!(capability, "fs.read" | "fs.write" | "graphics.window") {
            return Err(ManifestError::UnknownCapability);
        }
        if manifest.capability_count == MAX_CAPABILITIES {
            return Err(ManifestError::TooManyCapabilities);
        }
        if manifest.capabilities[..manifest.capability_count]
            .iter()
            .flatten()
            .any(|existing| existing.as_str() == capability)
        {
            return Err(ManifestError::DuplicateField);
        }
        manifest.capabilities[manifest.capability_count] = Some(FixedText::from_str(capability)?);
        manifest.capability_count += 1;
    }
    Ok(())
}
