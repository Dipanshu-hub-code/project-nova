# Phase 9 Architecture

Phase 9 adds allocation-free application metadata, a bounded registry, and
small in-memory desktop-service foundations. It does not add executable loading,
user-mode execution, IPC, or a displayed graphical desktop.

## Application lifecycle boundary

Manifest parsing → registry registration are implemented. Process creation,
window ownership, event delivery to applications, termination, and cleanup
require a verified process model and are explicitly unavailable.

The registry is owned by its caller and uses fixed capacity; there is no
`static mut` registry. The launcher returns an explicit
`ProcessExecutionUnavailable` error rather than simulating a successful launch.

## Service boundaries

- Notifications use a fixed-size FIFO queue and bounded text fields.
- Settings are an in-memory theme value and are not persisted.
- Session startup reports unavailable until display, input, and process
  backends are available.
- Logging exposes structured record types but is not connected to a sink.
- IPC is deferred until endpoints can be backed by real process ownership and
  capability checks.

## Manifest format

The parser accepts a small allocation-free key/value format with `id`, `name`,
`version`, and `exec` fields, an optional `icon`, and optional `caps` list.
Supported capabilities are `fs.read`, `fs.write`, and `graphics.window`.
Paths must be absolute and cannot contain `.` or `..` path components.
