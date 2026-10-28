# Phase 14 Architecture and Dependency Plan

## Current boundary

Project Nova currently boots a kernel that prints diagnostics and halts. The
SDK and its templates are host-buildable, but there is no user-mode process
loader, syscall entry, Nova filesystem, active graphical framebuffer, or
keyboard/mouse driver. The experimental syscall dispatcher returns
`NotSupported`; therefore a desktop application cannot truthfully use Nova
filesystem, process, network, audio, package, or window services yet.

The existing kernel service models remain authoritative for their limited
metadata. Phase 14 must not introduce parallel package databases, settings
stores, or claims of service integration.

## Application dependency order

The intended native dependency path is:

1. Establish safe user address spaces, privilege transitions, process lifecycle,
   and a versioned syscall entry.
2. Provide filesystem/storage and permission enforcement; then connect
   `nova-std` file operations.
3. Connect framebuffer/display and input drivers to the renderer, compositor,
   window manager, and event loop; expose a documented `nova-ui` drawing and
   window API.
4. Add IPC and authenticated service identity for clipboard, notifications,
   settings, and application-to-desktop integration.
5. Add actual network/audio drivers and service APIs as hardware support permits.
6. Build native applications on those stable APIs; connect the software center
   only after repository transport, package verification, authorization,
   transactions, and persistence are implemented.

This order preserves the existing kernel architecture and avoids duplicating
or bypassing system services.

## Milestone 1: shared UI model

`sdk/libraries/nova-ui` contains:

- Light and dark palette tokens and shared spacing/corner-radius values.
- A bounded focus manager with explicit registration order and Tab/Shift+Tab
  traversal operations.
- Buttons with variants, enabled/focused state, click eligibility, and
  keyboard activation checks.
- A single-line UTF-8 text field with accessible label, bounded input,
  character-boundary cursor motion, and safe deletion.
- Reusable size, inset, and stack-layout values with underflow checks.

The SDK uses these values as host-side presentation/state primitives. It does
not paint pixels, dispatch device input, create a Nova window, or implement an
accessibility bridge. Applications should resolve the active theme at render
time so a future desktop theme change can be reflected without restart; theme
selection currently comes from the kernel's in-memory Light/Dark settings
model and is not connected to this SDK or a running desktop.

## Application and service mapping

| Application/service | Intended Nova interfaces | Current status |
|---|---|---|
| File manager and editor | Permission-checked filesystem, file metadata, file associations, window/input APIs | Filesystem and native window APIs unavailable |
| Terminal | Process creation/termination, command interpreter, console/clipboard APIs | Process execution and shell unavailable |
| System monitor | Process enumeration, memory/system metrics, authorized control APIs | No process service or metrics API |
| Settings | Existing settings service, persistence, display/input/device services | Only in-memory Light/Dark value exists |
| Software center | Existing package/repository database and secure transaction service | Installer explicitly unavailable |
| Network manager | Network-interface and connection service | No networking backend |
| Audio controls | Audio device/volume service | No audio backend |
| Clipboard and notifications | Authenticated IPC and desktop services | Queue-only notifications; clipboard/IPC unavailable |
| Search and file associations | Permission-filtered app/filesystem indexes and launcher | No index, filesystem, file-type registry, or launcher execution |
| Desktop recovery | Process/window lifecycle and authenticated session manager | No running shell, process manager, or native window lifecycle |

Native applications must remain least-privileged. They must not receive direct
kernel privileges to compensate for missing APIs, and executables must never
be launched merely because a file was selected.

## Host preview distinction

`preview/nova-desktop.html` and `preview/software-center.html` are separate
browser prototypes. Their simulated controls are not backed by kernel services
and are not native applications. No Phase 14 claim treats those previews as
an operating-system desktop.
