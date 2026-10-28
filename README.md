# Project Nova

Project Nova is an experimental x86_64 operating-system kernel project. The
repository was empty, so this is a minimal, buildable starting point rather
than the phases and subsystems described in the supplied project guide.

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
stall the kernel. The additional kernel modules, tests, scripts, and
documentation listed in the supplied guide were not included because that
guide refers to missing earlier-phase code.
