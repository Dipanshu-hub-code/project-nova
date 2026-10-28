# Project Nova

Project Nova is an experimental x86_64 operating-system kernel project. It
contains a bootable kernel scaffold and early graphics, input, window-manager,
desktop, and application framework code. It is not a complete operating
system.

## Prerequisites

- Rust nightly (the version is selected by `rust-toolchain.toml`)
- The `x86_64-unknown-none` Rust target
- `bootimage` to create a BIOS bootable image
- QEMU (`qemu-system-x86_64`) to run it

Ubuntu or WSL2 is recommended for the boot-image and QEMU steps. In that
environment, install the tools and target with:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
rustup toolchain install nightly --profile minimal
rustup target add x86_64-unknown-none --toolchain nightly
rustup component add rust-src llvm-tools-preview --toolchain nightly
sudo apt update
sudo apt install -y build-essential qemu-system-x86
cargo install bootimage
```

Build and run from the repository root:

```sh
cargo bootimage -p nova-kernel --target x86_64-unknown-none
qemu-system-x86_64 -display none -serial stdio -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-nova-kernel.bin -boot order=c -no-reboot
```

The kernel currently displays startup and boot-memory-map diagnostics on VGA
and COM1, reports panic details when possible, and halts. VGA output scrolls
when the screen fills; serial writes are bounded so missing hardware cannot
stall the kernel.

## Desktop framework

The Phase 8 foundation includes checked framebuffer drawing, a fixed-capacity
window manager, basic panel/dock composition, and placeholder input/application
APIs. It does **not** show a graphical desktop yet: the BIOS bootloader does
not provide a framebuffer, and keyboard, mouse, font, and userspace drivers
are not implemented. See [docs/desktop-architecture.md](docs/desktop-architecture.md).

Run the host-side graphics tests with `cargo test -p nova-kernel --lib`. Build
the boot image with `./tools/build.sh` and run it with `./tools/run_qemu.sh`.

The runtime and services foundation adds a bounded application manifest
registry, an allocation-free manifest parser, notification FIFO, and in-memory
theme settings. Applications cannot be launched, IPC is not available, and no
framebuffer/input drivers are enabled. See the Phase 9 audit and architecture
notes in `docs/`.

The Phase 10 package foundation parses a bounded manifest subset and provides
caller-owned in-memory package/repository registries plus dependency checks.
It does not fetch repositories, verify signatures, unpack archives, persist a
database, or install packages; there is no `nova-pkg` CLI or package-backed
software-center UI. See the Phase 10 documentation in `docs/`.

The Phase 11 hardware foundation adds checked MMIO/resource descriptions, a
bounded device registry, and a host-testable PCI configuration parser. It does
not access PCI hardware or provide active storage, network, audio, input, or
graphics drivers. A VGA text-mode software-center screen is shown at kernel
boot, and `preview/software-center.html` provides a separate responsive host
preview with sample catalog content. Neither preview can install software.
The current BIOS boot path has no graphical framebuffer, so this is not yet a
graphical operating-system desktop. See the Phase 11 audit and
hardware-architecture notes in `docs/` and [the software-center status](docs/software-center.md).

Phase 12 adds bounded in-memory account metadata, pure permission-policy
helpers, and a bounded security-event queue. Authentication, privilege
enforcement, cryptography, encryption, signature verification, persistent
security logging, and recovery are not implemented; unavailable operations
fail closed. Do not use this kernel to store real credentials or sensitive
data. See [the Phase 12 security audit](docs/phase-12-security-audit.md).

For a more complete desktop concept, open [the interactive Nova desktop
preview](preview/nova-desktop.html). It includes a launcher, dock, mock app
windows, software center, notifications, and a mock control center. It is a
browser prototype with simulated controls and placeholder apps—not a GUI
running inside the kernel. A working graphical operating system still needs a
framebuffer/display driver, input drivers, process isolation, filesystems, and
real application services.

Phase 13 adds experimental syscall number definitions, host-buildable
`nova-std`/`nova-ui` crates, host app templates, and a `nova-dev` Cargo helper.
The kernel dispatcher intentionally returns `NotSupported`; these are SDK
shapes and development tools, not working Nova user-space APIs. See
[the Phase 13 audit](docs/phase-13-audit.md) and
[developer platform architecture](docs/developer-platform-architecture.md).

Phase 14 audits these runtime boundaries and adds tested host-side UI
primitives (theme tokens, focus traversal, button state, UTF-8 text input, and
layout values). It does not yet provide native applications: user-mode
execution, filesystem APIs, graphical display/input, networking, audio, and
cross-application IPC remain unavailable. Read the
[Phase 14 audit](docs/phase-14-audit.md),
[architecture and dependency plan](docs/phase-14-architecture.md), and
[native application status](docs/native-applications.md).

Phase 15 optimizes the checked software framebuffer rectangle-fill path and
adds an opt-in host microbenchmark. The measured result applies only to a host
memory buffer; it is not evidence of faster QEMU graphics or a responsive
desktop. See the [Phase 15 audit](docs/phase-15-audit.md) and
[performance/validation record](docs/phase-15-performance.md).
