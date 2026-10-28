# Native Applications

## Status

**Blocked by missing dependencies** for native applications: Nova has no
user-mode executable loader/process isolation, functional syscall entry,
filesystem, graphical window/input service, or IPC. The SDK templates run as
host programs and are not Nova binaries.

The Phase 14 shared UI model is **implemented and tested** as a host-side
library: common design tokens, keyboard focus traversal, button activation
state, bounded UTF-8 single-line editing, and simple layout values. It does not
render or create windows.

## Integration requirements

Native apps must use Nova's application manager and public SDK once those
interfaces are functional. Each app must declare identity and permissions,
handle service errors, release resources on close, and avoid unrestricted
kernel privileges. Do not treat package metadata or the browser preview as an
installed application.

## Build and tests

From the repository root, run `cargo test -p nova-ui --lib` and
`cargo check --workspace`. The SDK template build commands test host programs
only. See [the Phase 14 test record](./phase-14-testing.md) for what was
verified; no QEMU application integration has been verified.
