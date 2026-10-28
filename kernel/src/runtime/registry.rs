//! Fixed-capacity registry for validated application manifests.

use super::manifest::AppManifest;

pub const MAX_APPLICATIONS: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Registered,
    Running,
    Exited,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppEntry {
    pub id: u32,
    pub manifest: AppManifest,
    pub state: AppState,
    pub pid: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryError {
    Full,
    DuplicateManifest,
    NotFound,
    InvalidState,
}

pub struct AppRegistry {
    apps: [Option<AppEntry>; MAX_APPLICATIONS],
    next_id: u32,
}

impl AppRegistry {
    pub const fn new() -> Self {
        Self {
            apps: [None; MAX_APPLICATIONS],
            next_id: 1,
        }
    }

    pub fn register(&mut self, manifest: AppManifest) -> Result<u32, RegistryError> {
        if self
            .apps
            .iter()
            .flatten()
            .any(|entry| entry.manifest.id() == manifest.id())
        {
            return Err(RegistryError::DuplicateManifest);
        }
        let slot = self
            .apps
            .iter()
            .position(Option::is_none)
            .ok_or(RegistryError::Full)?;
        let mut id = self.next_id;
        while self.get(id).is_some() {
            id = id.wrapping_add(1).max(1);
        }
        self.next_id = id.wrapping_add(1).max(1);
        self.apps[slot] = Some(AppEntry {
            id,
            manifest,
            state: AppState::Registered,
            pid: None,
        });
        Ok(id)
    }

    pub fn get(&self, id: u32) -> Option<&AppEntry> {
        self.apps.iter().flatten().find(|entry| entry.id == id)
    }

    pub fn mark_running(&mut self, id: u32, pid: u64) -> Result<(), RegistryError> {
        let entry = self.find_mut(id)?;
        if entry.state != AppState::Registered || pid == 0 {
            return Err(RegistryError::InvalidState);
        }
        entry.state = AppState::Running;
        entry.pid = Some(pid);
        Ok(())
    }

    pub fn mark_finished(&mut self, id: u32, failed: bool) -> Result<(), RegistryError> {
        let entry = self.find_mut(id)?;
        if entry.state != AppState::Running {
            return Err(RegistryError::InvalidState);
        }
        entry.state = if failed {
            AppState::Failed
        } else {
            AppState::Exited
        };
        entry.pid = None;
        Ok(())
    }

    fn find_mut(&mut self, id: u32) -> Result<&mut AppEntry, RegistryError> {
        self.apps
            .iter_mut()
            .flatten()
            .find(|entry| entry.id == id)
            .ok_or(RegistryError::NotFound)
    }
}

impl Default for AppRegistry {
    fn default() -> Self {
        Self::new()
    }
}
