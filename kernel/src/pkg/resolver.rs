//! Bounded dependency validation against caller-supplied installed packages.

use crate::pkg::database::{PackageDatabase, MAX_INSTALLED};
use crate::pkg::metadata::{PackageId, PackageManifest};

const MAX_RESOLUTION_DEPTH: usize = MAX_INSTALLED;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolveError {
    MissingDependency,
    VersionConflict,
    CycleDetected,
    ResolutionLimit,
}

pub struct Resolver<'a> {
    database: &'a PackageDatabase,
}

impl<'a> Resolver<'a> {
    pub fn new(database: &'a PackageDatabase) -> Self {
        Self { database }
    }

    pub fn resolve(&self, manifest: &'a PackageManifest) -> Result<(), ResolveError> {
        let mut active: [Option<&'a PackageManifest>; MAX_RESOLUTION_DEPTH] =
            [None; MAX_RESOLUTION_DEPTH];
        let mut resolved: [Option<&'a PackageManifest>; MAX_RESOLUTION_DEPTH] =
            [None; MAX_RESOLUTION_DEPTH];
        let mut resolved_count = 0;
        self.visit(
            manifest,
            manifest,
            &mut active,
            &mut resolved,
            &mut resolved_count,
            0,
        )
    }

    fn visit(
        &self,
        root: &'a PackageManifest,
        manifest: &'a PackageManifest,
        active: &mut [Option<&'a PackageManifest>; MAX_RESOLUTION_DEPTH],
        resolved: &mut [Option<&'a PackageManifest>; MAX_RESOLUTION_DEPTH],
        resolved_count: &mut usize,
        depth: usize,
    ) -> Result<(), ResolveError> {
        if active[..depth]
            .iter()
            .flatten()
            .any(|entry| entry.id() == manifest.id())
        {
            return Err(ResolveError::CycleDetected);
        }
        if resolved[..*resolved_count]
            .iter()
            .flatten()
            .any(|entry| entry.id() == manifest.id())
        {
            return Ok(());
        }
        if depth == MAX_RESOLUTION_DEPTH {
            return Err(ResolveError::ResolutionLimit);
        }
        active[depth] = Some(manifest);

        for dependency in manifest.dependencies() {
            let id: &PackageId = dependency.id();
            let target = if id == root.id() {
                Some(root)
            } else {
                self.database.find(id).map(|installed| installed.manifest())
            }
            .ok_or(ResolveError::MissingDependency)?;

            if dependency
                .minimum_version()
                .is_some_and(|minimum| !target.version().is_at_least(minimum))
            {
                return Err(ResolveError::VersionConflict);
            }
            self.visit(root, target, active, resolved, resolved_count, depth + 1)?;
        }

        active[depth] = None;
        if *resolved_count == MAX_RESOLUTION_DEPTH {
            return Err(ResolveError::ResolutionLimit);
        }
        resolved[*resolved_count] = Some(manifest);
        *resolved_count += 1;
        Ok(())
    }
}
