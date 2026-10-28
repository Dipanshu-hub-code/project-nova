//! Safe application-registry facade; launching remains unavailable.

use crate::runtime::launcher::{self, LaunchError};
use crate::runtime::manifest::AppManifest;
use crate::runtime::registry::{AppRegistry, RegistryError};

pub struct AppManager {
    registry: AppRegistry,
}

impl AppManager {
    pub const fn new() -> Self {
        Self {
            registry: AppRegistry::new(),
        }
    }

    pub fn register(&mut self, manifest: AppManifest) -> Result<u32, RegistryError> {
        self.registry.register(manifest)
    }

    pub fn launch(&self, app_id: u32) -> Result<(), LaunchError> {
        launcher::launch(app_id)
    }

    pub const fn registry(&self) -> &AppRegistry {
        &self.registry
    }
}

impl Default for AppManager {
    fn default() -> Self {
        Self::new()
    }
}
