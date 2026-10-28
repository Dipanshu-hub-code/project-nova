# Phase 14 Initial Repository Audit

## Scope and baseline

This audit records the repository state before Phase 14 changes. The baseline
passed `./tools/test.sh` (32 kernel library tests, 2 `nova-std` tests, 2
`nova-ui` tests, and 1 `nova-dev` test, plus workspace checking),
`cargo check -p nova-kernel --target x86_64-unknown-none`, and
`./tools/build.sh`. The boot-image build is not a QEMU integration test.

The project is an experimental kernel and host-side SDK, not a complete
operating system. The classifications below distinguish checked code and
metadata foundations from functioning hardware, services, and applications.

## Subsystem classifications

| Subsystem | Classification | Evidence and limitation |
|---|---|---|
| Kernel boot and build | Implemented and verified | x86_64 `no_std` kernel builds and boot image is generated; startup reports diagnostics and halts. |
| Memory and scheduling | Partially implemented | Early kernel foundations exist; there is no user process scheduler or isolated user address space. |
| Hardware abstraction and drivers | Partially implemented | Resource descriptions and PCI configuration parsing exist; no active storage, network, audio, keyboard, mouse, or framebuffer drivers. |
| Graphics primitives | Implemented but unverified on hardware | Checked framebuffer drawing code exists, but boot does not supply or configure an active graphical framebuffer. |
| Compositor and window manager | Partially implemented | Bounded in-memory window metadata and framebuffer composition exist; no active display path or input-driven window lifecycle. |
| Desktop shell and launcher | Partially implemented | Kernel-side panel/dock foundations and separate browser prototypes exist; no running graphical shell. |
| Application lifecycle | Missing | Application manifests/registry exist, but launcher and termination report execution unavailable. |
| User-mode isolation and syscall entry | Missing | Experimental syscall identifiers and validation exist; dispatch returns `NotSupported`. |
| Nova standard library | Partially implemented | Host-buildable facade exists; filesystem, process, time, and related OS operations return `NotSupported`. |
| GUI framework | Partially implemented | `nova-ui` now supplies tested host-side design tokens and control/layout state; rendering, window creation, and UI initialization remain unavailable. |
| Filesystem and storage | Missing | SDK file operations return `NotSupported`; no usable filesystem or storage driver is connected. |
| Networking | Missing | Device metadata exists, but no network driver, interface service, connection manager, or socket API is available. |
| Audio | Missing | No audio driver or audio service is available. |
| Accounts and security | Partially implemented | Bounded account metadata, pure permission helpers, and bounded audit queue exist; authentication, enforcement, and cryptography are unavailable. |
| Application permissions | Partially implemented | Permission policy helpers exist; there is no execution boundary that enforces application permissions. |
| Package manager | Partially implemented | Bounded package/repository metadata and dependency checks exist; fetch, signature verification, installation, persistence, updates, and removal are unavailable. |
| Software center | Partially implemented | A VGA text screen and browser prototype exist; neither is connected to package installation. |
| Notifications | Partially implemented | Bounded in-memory queue exists; no native app delivery or persistent preference service is connected. |
| Settings | Partially implemented | In-memory light/dark theme value exists; settings are not persisted or wired to a running desktop. |
| Clipboard | Missing | No cross-application clipboard service or IPC is available. |
| File associations and search | Missing | No association registry, indexed search, or application/file-opening service exists. |
| Developer SDK and tooling | Partially implemented | Host templates and `nova-dev` CLI work as host tools; generated applications are not Nova executables. |
| QEMU configuration | Implemented but unverified for Phase 14 | Build/run scripts exist; no Phase 14 graphical or application integration was verified in QEMU. |

No identified subsystem depends on a missing package dependency for this
milestone. The primary blockers are missing kernel/runtime services and
hardware integration, not missing third-party crates.

## Phase 14 outcome

Milestone 1 is partially implemented at the SDK model level: shared theme
tokens, focus traversal, keyboard-aware buttons, UTF-8-safe text editing, and
checked layout values have unit tests. These primitives are not rendered in
the kernel desktop. The remaining milestones are blocked for native OS
acceptance until application execution, a graphical window/input path, and
filesystem/system-service APIs exist. See
[`phase-14-architecture.md`](./phase-14-architecture.md) and
[`phase-14-testing.md`](./phase-14-testing.md).
