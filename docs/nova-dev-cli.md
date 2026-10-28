# `nova-dev` CLI

The workspace binary provides:

- `nova-dev new NAME`: create a non-overwriting host Rust project scaffold.
- `build`, `run`, `test`, `clean`: invoke Cargo in the current Rust project.
- `info`: print SDK/API support status.
- `doctor`: check Cargo, Rust, QEMU, and the bare-metal target.
- `package`: report that packaging/signing is unavailable.

Cargo commands are invoked directly (not through a shell). Generated projects
are host Rust programs; they are not executable on Nova. Run `cargo run -p
nova-dev -- info` from the repository root.
