//! Parser for the documented, allocation-free Nova package manifest subset.

use core::str::FromStr;

pub const MAX_ID_LEN: usize = 64;
pub const MAX_NAME_LEN: usize = 128;
pub const MAX_VERSION_LEN: usize = 32;
pub const MAX_PATH_LEN: usize = 256;
pub const MAX_DEPENDENCIES: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestError {
    InvalidFormat,
    DuplicateField,
    MissingField,
    ValueTooLong,
    UnsupportedArchitecture,
    InvalidPath,
    InvalidDependency,
    TooManyDependencies,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedString<const N: usize> {
    bytes: [u8; N],
    len: usize,
}

impl<const N: usize> FixedString<N> {
    pub const fn empty() -> Self {
        Self {
            bytes: [0; N],
            len: 0,
        }
    }

    pub(crate) fn parse(value: &str) -> Result<Self, ManifestError> {
        if value.is_empty() {
            return Err(ManifestError::InvalidFormat);
        }
        if value.len() > N {
            return Err(ManifestError::ValueTooLong);
        }

        let mut result = Self::empty();
        result.bytes[..value.len()].copy_from_slice(value.as_bytes());
        result.len = value.len();
        Ok(result)
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.len])
            .expect("fixed string is only constructed from valid UTF-8")
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackageId(FixedString<MAX_ID_LEN>);

impl PackageId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn parse(value: &str) -> Result<Self, ManifestError> {
        if !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_'))
        {
            return Err(ManifestError::InvalidFormat);
        }
        Ok(Self(FixedString::parse(value)?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version(FixedString<MAX_VERSION_LEN>);

impl Version {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn parse(value: &str) -> Result<Self, ManifestError> {
        if value
            .split('.')
            .any(|part| part.is_empty() || part.parse::<u64>().is_err())
        {
            return Err(ManifestError::InvalidFormat);
        }
        Ok(Self(FixedString::parse(value)?))
    }

    pub fn is_at_least(&self, minimum: &Self) -> bool {
        let mut actual = self.as_str().split('.');
        let mut required = minimum.as_str().split('.');
        loop {
            let left = actual.next();
            let right = required.next();
            if left.is_none() && right.is_none() {
                return true;
            }
            let left = left
                .map(|part| part.parse::<u64>().expect("validated version"))
                .unwrap_or(0);
            let right = right
                .map(|part| part.parse::<u64>().expect("validated version"))
                .unwrap_or(0);
            if left != right {
                return left > right;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dependency {
    id: PackageId,
    minimum_version: Option<Version>,
}

impl Dependency {
    const EMPTY: Self = Self {
        id: PackageId(FixedString::empty()),
        minimum_version: None,
    };

    pub fn id(&self) -> &PackageId {
        &self.id
    }

    pub const fn minimum_version(&self) -> Option<&Version> {
        self.minimum_version.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackageManifest {
    id: PackageId,
    name: FixedString<MAX_NAME_LEN>,
    version: Version,
    architecture: FixedString<16>,
    exec_path: FixedString<MAX_PATH_LEN>,
    icon_path: Option<FixedString<MAX_PATH_LEN>>,
    dependencies: [Dependency; MAX_DEPENDENCIES],
    dependency_count: usize,
}

impl PackageManifest {
    const fn empty() -> Self {
        Self {
            id: PackageId(FixedString::empty()),
            name: FixedString::empty(),
            version: Version(FixedString::empty()),
            architecture: FixedString::empty(),
            exec_path: FixedString::empty(),
            icon_path: None,
            dependencies: [Dependency::EMPTY; MAX_DEPENDENCIES],
            dependency_count: 0,
        }
    }

    pub fn id(&self) -> &PackageId {
        &self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn architecture(&self) -> &str {
        self.architecture.as_str()
    }

    pub fn exec_path(&self) -> &str {
        self.exec_path.as_str()
    }

    pub fn icon_path(&self) -> Option<&str> {
        self.icon_path.as_ref().map(FixedString::as_str)
    }

    pub fn dependencies(&self) -> &[Dependency] {
        &self.dependencies[..self.dependency_count]
    }

    pub fn validate_paths(&self) -> Result<(), ManifestError> {
        validate_install_path(self.exec_path())?;
        if let Some(icon_path) = self.icon_path() {
            validate_install_path(icon_path)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Section {
    None,
    Package,
    Exec,
    Desktop,
    Dependencies,
}

impl FromStr for PackageManifest {
    type Err = ManifestError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let mut manifest = Self::empty();
        let mut section = Section::None;
        let mut seen: u16 = 0;

        for raw_line in source.lines() {
            let line = raw_line.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                section = match line {
                    "[package]" => Section::Package,
                    "[exec]" => Section::Exec,
                    "[desktop]" => Section::Desktop,
                    "[dependencies]" => Section::Dependencies,
                    _ => return Err(ManifestError::InvalidFormat),
                };
                continue;
            }

            let (key, value) = line.split_once('=').ok_or(ManifestError::InvalidFormat)?;
            let key = key.trim();
            let value = value.trim();
            let bit = match (section, key) {
                (Section::Package, "id") => 1 << 0,
                (Section::Package, "name") => 1 << 1,
                (Section::Package, "version") => 1 << 2,
                (Section::Package, "architecture") => 1 << 3,
                (Section::Exec, "path") => 1 << 4,
                (Section::Desktop, "icon") => 1 << 5,
                (Section::Dependencies, "requires") => 1 << 6,
                _ => return Err(ManifestError::InvalidFormat),
            };
            if seen & bit != 0 {
                return Err(ManifestError::DuplicateField);
            }
            seen |= bit;

            match (section, key) {
                (Section::Package, "id") => {
                    manifest.id = PackageId::parse(parse_quoted(value)?)?;
                }
                (Section::Package, "name") => {
                    manifest.name = FixedString::parse(parse_quoted(value)?)?;
                }
                (Section::Package, "version") => {
                    manifest.version = Version::parse(parse_quoted(value)?)?;
                }
                (Section::Package, "architecture") => {
                    let architecture = parse_quoted(value)?;
                    if architecture != "x86_64" {
                        return Err(ManifestError::UnsupportedArchitecture);
                    }
                    manifest.architecture = FixedString::parse(architecture)?;
                }
                (Section::Exec, "path") => {
                    manifest.exec_path = FixedString::parse(parse_quoted(value)?)?;
                }
                (Section::Desktop, "icon") => {
                    manifest.icon_path = Some(FixedString::parse(parse_quoted(value)?)?);
                }
                (Section::Dependencies, "requires") => {
                    parse_dependencies(value, &mut manifest)?;
                }
                _ => return Err(ManifestError::InvalidFormat),
            }
        }

        if seen & 0b0001_1111 != 0b0001_1111 {
            return Err(ManifestError::MissingField);
        }
        manifest.validate_paths()?;
        Ok(manifest)
    }
}

fn parse_quoted(value: &str) -> Result<&str, ManifestError> {
    let value = value.trim();
    if value.len() < 2 || !value.starts_with('"') || !value.ends_with('"') {
        return Err(ManifestError::InvalidFormat);
    }
    let inner = &value[1..value.len() - 1];
    if inner.contains('"')
        || inner.contains('\\')
        || inner.bytes().any(|byte| byte.is_ascii_control())
    {
        return Err(ManifestError::InvalidFormat);
    }
    Ok(inner)
}

fn parse_dependencies(value: &str, manifest: &mut PackageManifest) -> Result<(), ManifestError> {
    let value = value.trim();
    if !value.starts_with('[') || !value.ends_with(']') {
        return Err(ManifestError::InvalidFormat);
    }
    let contents = value[1..value.len() - 1].trim();
    if contents.is_empty() {
        return Ok(());
    }

    for item in contents.split(',') {
        if manifest.dependency_count == MAX_DEPENDENCIES {
            return Err(ManifestError::TooManyDependencies);
        }
        let item = parse_quoted(item.trim())?.trim();
        let (id, minimum_version) = match item.split_once(">=") {
            Some((id, version)) => (
                PackageId::parse(id.trim())?,
                Some(Version::parse(version.trim())?),
            ),
            None if !item.contains('<') && !item.contains('=') && !item.contains('>') => {
                (PackageId::parse(item)?, None)
            }
            None => return Err(ManifestError::InvalidDependency),
        };
        if manifest.dependencies[..manifest.dependency_count]
            .iter()
            .any(|dependency| dependency.id == id)
        {
            return Err(ManifestError::InvalidDependency);
        }
        manifest.dependencies[manifest.dependency_count] = Dependency {
            id,
            minimum_version,
        };
        manifest.dependency_count += 1;
    }
    Ok(())
}

fn validate_install_path(path: &str) -> Result<(), ManifestError> {
    let Some(relative_path) = path.strip_prefix("/usr/") else {
        return Err(ManifestError::InvalidPath);
    };
    if path.contains('\\')
        || relative_path
            .split('/')
            .any(|part| part.is_empty() || matches!(part, "." | ".."))
    {
        return Err(ManifestError::InvalidPath);
    }
    Ok(())
}

pub fn validate_archive_path(path: &str) -> Result<(), ManifestError> {
    if path.is_empty()
        || path.starts_with('/')
        || path.contains('\\')
        || path.contains(':')
        || path
            .split('/')
            .any(|part| part.is_empty() || matches!(part, "." | ".."))
    {
        return Err(ManifestError::InvalidPath);
    }
    Ok(())
}
