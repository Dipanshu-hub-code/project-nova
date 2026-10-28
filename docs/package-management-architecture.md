# Package Management Architecture

## Current foundation

- `kernel/src/pkg/metadata.rs`: bounded parser for a documented TOML subset,
  package IDs and versions, dependency declarations, and path validation.
- `kernel/src/pkg/database.rs`: caller-owned in-memory records for installed
  package metadata. It is not persistent and does not own installed files.
- `kernel/src/pkg/repository.rs`: caller-owned repository configuration. No
  index parser, network client, or certificate validation exists yet.
- `kernel/src/pkg/resolver.rs`: bounded validation of dependencies against
  records already in the in-memory database.
- `kernel/src/pkg/installer.rs`: explicit unavailable boundary; it never
  reports a package as installed.

There is currently no CLI or graphical software center. A future host-side
client and GUI should use a shared package backend rather than invoke privileged
kernel installation logic.

## Required future data flow

Repository index → resolve candidates → download → verify signature and digest
→ validate and stage archive paths → commit files and database transaction →
register application → refresh launcher.

Every step in this flow is future work. Do not treat repository configuration
or dependency validation as a completed package installation system.
