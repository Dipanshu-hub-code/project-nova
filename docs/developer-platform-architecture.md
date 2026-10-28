# Developer Platform Architecture

## Components

- `kernel/src/abi/`: versioned syscall identifiers and errors, separated from
  private kernel modules.
- `kernel/src/syscall/`: validation-only dispatcher. It never dereferences
  the supplied argument words and currently returns `NotSupported`.
- `sdk/libraries/nova-std/`: host-buildable, explicit-`Result` API facade.
- `sdk/libraries/nova-ui/`: small host-buildable widget metadata and a window
  type whose `show` operation reports unavailable.
- `tools/nova-dev/`: host CLI for scaffolding ordinary Rust projects and
  invoking Cargo without a shell.
- `sdk/templates/`: CLI and GUI sample crates that compile on the host and
  report missing Nova services accurately.

## Versioning and support

The syscall identifiers are marked experimental ABI v1; they are not a stable
binary compatibility promise because no syscall entry point exists. SDK
versioning follows the crate package versions. The current `.npkg` format has
no packager or signing implementation.

`nova-dev new`, `build`, `run`, `test`, and `clean` create/manage standard host
Rust projects. This must not be confused with compiling or launching a
user-mode Nova application.
