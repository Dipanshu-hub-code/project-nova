//! Caller-owned, bounded in-memory package database.

use crate::pkg::metadata::{PackageId, PackageManifest};

pub const MAX_INSTALLED: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallState {
    Installed,
    Removed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseError {
    Full,
    AlreadyInstalled,
    NotInstalled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstalledPackage {
    manifest: PackageManifest,
    state: InstallState,
}

impl InstalledPackage {
    pub fn manifest(&self) -> &PackageManifest {
        &self.manifest
    }

    pub const fn state(&self) -> InstallState {
        self.state
    }
}

pub struct PackageDatabase {
    packages: [Option<InstalledPackage>; MAX_INSTALLED],
    count: usize,
}

impl PackageDatabase {
    pub const fn new() -> Self {
        Self {
            packages: [None; MAX_INSTALLED],
            count: 0,
        }
    }

    pub fn install_record(&mut self, manifest: PackageManifest) -> Result<(), DatabaseError> {
        if self.find(manifest.id()).is_some() {
            return Err(DatabaseError::AlreadyInstalled);
        }
        let slot = self
            .packages
            .iter()
            .position(|entry| {
                entry.is_none()
                    || entry
                        .as_ref()
                        .is_some_and(|package| package.state == InstallState::Removed)
            })
            .ok_or(DatabaseError::Full)?;

        self.packages[slot] = Some(InstalledPackage {
            manifest,
            state: InstallState::Installed,
        });
        self.count += 1;
        Ok(())
    }

    pub fn remove(&mut self, id: &PackageId) -> Result<(), DatabaseError> {
        let package = self
            .packages
            .iter_mut()
            .flatten()
            .find(|package| package.state == InstallState::Installed && package.manifest.id() == id)
            .ok_or(DatabaseError::NotInstalled)?;
        package.state = InstallState::Removed;
        self.count -= 1;
        Ok(())
    }

    pub fn find(&self, id: &PackageId) -> Option<&InstalledPackage> {
        self.packages
            .iter()
            .flatten()
            .find(|package| package.state == InstallState::Installed && package.manifest.id() == id)
    }

    pub const fn len(&self) -> usize {
        self.count
    }

    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl Default for PackageDatabase {
    fn default() -> Self {
        Self::new()
    }
}
