# Project Nova OS Terminal Command Reference

## Scope and accuracy

**Nova does not currently have an interactive terminal, shell, command
dispatcher, or user-mode process execution.** The kernel boots to a VGA text
status/preview screen and halts. It does not accept typed commands. The
Linux-inspired command names in this document are an inventory of requested
utilities, not a claim that they run on Nova.

The only project-specific command-line program currently implemented is
`nova-dev`, a **host development tool**. It creates ordinary host Rust projects
and invokes Cargo. Host shell, Cargo, QEMU, and repository scripts run on the
development machine, not inside Nova.

There is no supported syntax or option list for a command marked
`NOT IMPLEMENTED`; examples of such commands are deliberately omitted so they
cannot be mistaken for working operations. See the [availability
matrix](docs/cli/COMMAND_IMPLEMENTATION_STATUS.md) and [command
index](docs/cli/COMMAND_INDEX.md).

## Discoverability and command help

There is no Nova `help`, `man`, command completion, or command-not-found
implementation because Nova has no shell. On the host, use:

- This file for command support and development usage.
- [`docs/cli/COMMAND_INDEX.md`](docs/cli/COMMAND_INDEX.md) for a searchable
  category index.
- [`docs/cli/COMMAND_IMPLEMENTATION_STATUS.md`](docs/cli/COMMAND_IMPLEMENTATION_STATUS.md)
  for command status and evidence.
- `nova-dev` with no arguments to display its usage line. There is no
  command-specific `--help` implementation.

## Category A: Essential terminal commands

No commands in this category can currently run in Nova. `clear`, `echo`,
`date`, and similar functions of the host's shell are not Nova built-ins.
`nova-dev` is only a host program and cannot be invoked from the kernel's VGA
screen.

| Name | Status | Notes |
|---|---|---|
| `help` | NOT IMPLEMENTED | No command dispatcher or help system. |
| `clear` | NOT IMPLEMENTED | The kernel clears its VGA text buffer during boot, but this is not a command. |
| `exit` | NOT IMPLEMENTED | No shell session exists to exit. |
| `history` | NOT IMPLEMENTED | No shell input/history exists. |
| `echo` | NOT IMPLEMENTED | No Nova shell built-in. |
| `pwd` | NOT IMPLEMENTED | No Nova shell or filesystem. |
| `whoami` | NOT IMPLEMENTED | Account metadata is not exposed through a command; authentication is unavailable. |
| `date` | NOT IMPLEMENTED | No Nova clock/RTC command or timer service. |
| `hostname` | NOT IMPLEMENTED | No Nova hostname command or configuration service. |
| `uname` | NOT IMPLEMENTED | No Nova system-information command. |

No supported Nova syntax, options, output, or exit codes exist for these names.

## Category B: File and directory management

There is no usable Nova filesystem, storage driver, or shell. The SDK file API
returns `NotSupported`. Do not use any of the following as though it would
browse or modify Nova data.

| Name | Status | Notes |
|---|---|---|
| `ls`, `cd`, `mkdir`, `rmdir`, `touch` | NOT IMPLEMENTED | No filesystem or shell. |
| `cp`, `mv`, `rm` | NOT IMPLEMENTED | File copy, move, and delete cannot be performed by Nova commands. |
| `cat`, `less`, `head`, `tail` | NOT IMPLEMENTED | No Nova file-reading command. |
| `file`, `stat`, `find`, `tree` | NOT IMPLEMENTED | No filesystem metadata, search, or tree utility. |

No options or recursive-operation semantics are supported. Destructive
operations are not available. See [file-manager status](docs/file-manager.md).

## Category C: Text processing and search

Nova has no shell pipes, streams, command chaining, or text-utility runtime.
These requested names are not available:

| Names | Status |
|---|---|
| `grep`, `sort`, `uniq`, `wc`, `cut`, `tr`, `diff`, `sed`, `awk` | NOT IMPLEMENTED |

There is no Nova command syntax or option support. On a host development
machine, use that machine's own tools on host files; this does not inspect
files within a running Nova OS.

## Category D: System information

The kernel has limited boot-time diagnostic text, but no interactive command,
timer, memory accounting, scheduler, or process metrics API.

| Names | Status | Notes |
|---|---|---|
| `uptime`, `free`, `df`, `du`, `ps`, `top`, `vmstat`, `lscpu`, `lsmem` | NOT IMPLEMENTED | No corresponding Nova runtime metric command/API. |
| `dmesg`, `journalctl` | NOT IMPLEMENTED | No command-accessible kernel ring buffer or journal. Boot diagnostics are printed once to VGA/serial. |
| `uname` | NOT IMPLEMENTED | No system-information utility. |

The kernel reports a boot memory-region count but not free/used memory or
CPU/process statistics. Do not treat that startup line as a system-monitoring
interface.

## Category E: Process and service management

There is no scheduler, process table, interactive job control, or service
manager. The application registry stores metadata only; process launch and
termination fail as unavailable.

| Names | Status | Notes |
|---|---|---|
| `ps`, `top`, `kill`, `jobs`, `bg`, `fg` | NOT IMPLEMENTED | No Nova process or shell job-control APIs. |
| `systemctl`, `service` | NOT IMPLEMENTED | Nova does not implement systemd or a Linux-compatible service manager. |

The absence of a command does not mean that no kernel initialization code
runs: startup initializes metadata foundations, prints status, and halts.

## Category F: Software and package management

There is no `nova-pkg` executable or graphical install workflow. The kernel
contains bounded package metadata and dependency-planning code, but has no
repository transport, signature verification, archive extraction, persistent
database, or installer. The VGA software-center screen is an explicitly
offline preview, not a package manager.

| Name | Status | Notes |
|---|---|---|
| `nova-pkg` and `nova-pkg search/install/remove/update/upgrade/list/info/verify/clean` | NOT IMPLEMENTED | No package CLI; installation is unavailable. |
| Graphical Software Center | PARTIAL | Static VGA text preview and separate browser prototype only; no installation or launch. |
| `nova-dev package` | NOT IMPLEMENTED | Recognized by the host tool but returns an explicit unavailable error because packaging/signing are absent. |

Do not treat example `nova-pkg` syntax in project proposals or older plans as
implemented command syntax. See [software-center status](docs/software-center.md)
and [package architecture](docs/package-management-architecture.md).

## Category G: Project development commands (host only)

Run development commands on the host machine from the repository root unless
otherwise noted. The project recommends Ubuntu or WSL2 for boot-image/QEMU
work. Requirements include the configured Rust nightly toolchain, the
`x86_64-unknown-none` target, `bootimage`, and QEMU for emulation.

### Command: `nova-dev`

**Status:** HOST ONLY

**Purpose:** Scaffolds an ordinary Rust host project or forwards selected
Cargo operations from the current directory.

**Syntax:** `nova-dev <new NAME|build|run|test|clean|info|doctor|package>`

**Examples:**

```text
nova-dev
nova-dev info
nova-dev doctor
nova-dev new demo-app
```

**Expected behavior:** With no argument, prints usage and exits with status 2.
Unknown commands print an error and usage. Runtime errors are written to stderr
and exit with status 1. `info` reports the experimental/unavailable Nova API
state. `doctor` checks host Cargo, Rust, QEMU, and the bare-metal target.

**Options:** No global options or `--help` flag are implemented. The command
does not forward extra arguments to its subcommands.

**Requirements:** Host Rust toolchain; working directory permissions. `doctor`
does not install missing dependencies.

**Common errors:** Invalid subcommand; missing Cargo project for Cargo-backed
operations; unavailable executable in `doctor`; invalid/occupied project name
for `new`.

**Related commands:** [`nova-dev new`](#command-nova-dev-new),
[`nova-dev build`](#command-nova-dev-build),
[`nova-dev doctor`](#command-nova-dev-doctor).

**Implementation notes:** This runs on the host. A generated project is not a
Nova executable.

### Command: `nova-dev new`

**Status:** HOST ONLY

**Purpose:** Creates a non-overwriting host Rust project scaffold in the
current directory.

**Syntax:** `nova-dev new <project-name>`

**Examples:**

```text
nova-dev new nova-demo
```

**Expected behavior:** Creates a project directory with `Cargo.toml`,
`src/main.rs`, and `README.md`. The generated program identifies itself as a
host program; it is not runnable on Nova.

**Options:** No flags. Names are limited to 64 bytes, must start with an ASCII
letter or digit, and may contain only ASCII letters, digits, `-`, and `_`.
Existing destination directories are not overwritten.

**Requirements:** Host filesystem write permission in the current directory.

**Common errors:** Invalid name; destination already exists; current directory
cannot be read; scaffold files cannot be created or written. A partial write
can leave the newly created project directory.

**Related commands:** [`nova-dev build`](#command-nova-dev-build),
[`nova-dev run`](#command-nova-dev-run).

**Implementation notes:** This uses the host filesystem and standard Rust
tooling, not Nova filesystem APIs.

### Command: `nova-dev build`

**Status:** HOST ONLY

**Purpose:** Runs `cargo build` in the current host Rust project.

**Syntax:** `nova-dev build`

**Examples:**

```text
cd path/to/host-rust-project
nova-dev build
```

**Expected behavior:** Invokes Cargo directly (not through a shell) and
returns an error if Cargo exits unsuccessfully.

**Options:** No extra Cargo options are forwarded.

**Requirements:** A `Cargo.toml` in the current directory and `cargo` on the
host `PATH`.

**Common errors:** Missing manifest, Cargo not found, or Cargo build failure.

**Related commands:** [`nova-dev test`](#command-nova-dev-test),
[direct host Cargo commands](#direct-host-cargo-commands).

**Implementation notes:** It builds a host project. It does not build the
Nova kernel; use `./tools/build.sh` for the kernel boot image.

### Command: `nova-dev run`

**Status:** HOST ONLY

**Purpose:** Runs `cargo run` in the current host Rust project.

**Syntax:** `nova-dev run`

**Examples:**

```text
cd path/to/host-rust-project
nova-dev run
```

**Expected behavior:** Invokes Cargo directly in the current directory.

**Options:** No extra Cargo options are forwarded.

**Requirements:** A host Rust project with `Cargo.toml` and `cargo` on `PATH`.

**Common errors:** Missing manifest, Cargo not found, compilation error, or
host program failure.

**Related commands:** [`nova-dev build`](#command-nova-dev-build).

**Implementation notes:** This runs a host program, not a Nova user-mode app.

### Command: `nova-dev test`

**Status:** HOST ONLY

**Purpose:** Runs `cargo test` in the current host Rust project.

**Syntax:** `nova-dev test`

**Examples:**

```text
cd path/to/host-rust-project
nova-dev test
```

**Expected behavior:** Invokes Cargo and reports failure if its exit status is
unsuccessful.

**Options:** No extra Cargo options are forwarded.

**Requirements:** A host Rust project with `Cargo.toml` and `cargo` on `PATH`.

**Common errors:** Missing manifest, Cargo not found, or test failure.

**Related commands:** [`./tools/test.sh`](#command-tools-testsh).

**Implementation notes:** For this workspace use `./tools/test.sh`, which
selects the appropriate library tests and checks.

### Command: `nova-dev clean`

**Status:** HOST ONLY

**Purpose:** Runs `cargo clean` in the current host project.

**Syntax:** `nova-dev clean`

**Examples:**

```text
cd path/to/host-rust-project
nova-dev clean
```

**Expected behavior:** Removes Cargo build artifacts for the current host
project according to Cargo behavior.

**Options:** No extra Cargo options are forwarded.

**Requirements:** A `Cargo.toml` and host `cargo`.

**Common errors:** Missing manifest, Cargo not found, or Cargo cleanup failure.

**Related commands:** [`nova-dev build`](#command-nova-dev-build).

**Implementation notes:** This is a destructive host build-artifact cleanup,
not a Nova filesystem command.

### Command: `nova-dev info`

**Status:** HOST ONLY

**Purpose:** Displays the developer tool version and current Nova API support
summary.

**Syntax:** `nova-dev info`

**Examples:**

```text
nova-dev info
```

**Expected behavior:** Prints the tool version, experimental syscall ABI
status, and unavailable window service status.

**Options:** None.

**Requirements:** The host `nova-dev` binary.

**Common errors:** No known command-specific runtime requirement beyond
starting the binary.

**Related commands:** [`nova-dev doctor`](#command-nova-dev-doctor).

**Implementation notes:** This is a host diagnostic, not a Nova command.

### Command: `nova-dev doctor`

**Status:** HOST ONLY

**Purpose:** Checks basic host development prerequisites.

**Syntax:** `nova-dev doctor`

**Examples:**

```text
nova-dev doctor
```

**Expected behavior:** Checks `cargo`, `rustc`, `qemu-system-x86_64`, and the
installed `x86_64-unknown-none` Rust target. It prints `[ok]`, `[missing]`, or
`[unknown]` entries. The current implementation returns success even when it
prints missing prerequisites.

**Options:** None.

**Requirements:** Host `nova-dev`; `rustup` is needed to determine target
installation status.

**Common errors:** Missing tools are reported; this command does not install
them. Missing `rustup` produces unknown target status.

**Related commands:** [`./tools/build.sh`](#command-tools-buildsh),
[`./tools/run_qemu.sh`](#command-tools-run_qemush).

**Implementation notes:** A successful process exit does not guarantee every
prerequisite was found; inspect the printed diagnostics.

### Command: `nova-dev package`

**Status:** NOT IMPLEMENTED

**Purpose:** Intended package creation entry point.

**Syntax:** `nova-dev package`

**Examples:** None; package creation is unavailable.

**Expected behavior:** The host tool recognizes the subcommand but returns an
error stating that the `.npkg` packager and signing workflow are not
implemented.

**Options:** None.

**Requirements:** A future package format implementation and signing workflow.

**Common errors:** Always reports unavailable.

**Related commands:** [Package management](#category-f-software-and-package-management).

**Implementation notes:** This is a placeholder path, not a functioning
package command.

### Host development commands

The following are exact repository workflows, not Nova terminal commands.

<a id="command-tools-testsh"></a>

#### Command: `./tools/test.sh`

**Status:** HOST ONLY

**Purpose:** Runs workspace library tests, `nova-dev` tests, and workspace
checks.

**Syntax:** `./tools/test.sh`

**Examples:** Run from the repository root under Bash (Ubuntu/WSL2 recommended).

**Expected behavior:** Runs `cargo test --workspace --lib`, `cargo test -p
nova-dev`, and `cargo check --workspace`, stopping on the first failure.

**Options:** No script-specific options.

**Requirements:** Bash and the Rust toolchain/dependencies.

**Common errors:** Cargo/test/check failure or missing Bash/toolchain.

**Related commands:** [`./tools/build.sh`](#command-tools-buildsh).

**Implementation notes:** The `--lib` selector avoids trying to host-test the
bare-metal kernel binary.

<a id="command-tools-buildsh"></a>

#### Command: `./tools/build.sh`

**Status:** HOST ONLY

**Purpose:** Creates the BIOS boot image for the Nova kernel.

**Syntax:** `./tools/build.sh`

**Examples:** Run from any working directory where the script is accessible;
it resolves the repository root itself.

**Expected behavior:** Runs `cargo bootimage -p nova-kernel --target
x86_64-unknown-none`.

**Options:** No script-specific options.

**Requirements:** Bash, configured nightly Rust, target, `bootimage`, and
required Cargo dependencies.

**Common errors:** Toolchain, target, dependency, compilation, or bootimage
failures.

**Related commands:** [`./tools/run_qemu.sh`](#command-tools-run_qemush).

**Implementation notes:** Produces a boot image; it does not prove graphical
desktop or application functionality.

<a id="command-tools-run_qemush"></a>

#### Command: `./tools/run_qemu.sh`

**Status:** HOST ONLY

**Purpose:** Boots the existing Nova kernel image in QEMU.

**Syntax:** `./tools/run_qemu.sh [QEMU options]`

**Examples:**

```text
./tools/build.sh
./tools/run_qemu.sh
./tools/run_qemu.sh -display none
```

**Expected behavior:** Runs `qemu-system-x86_64` with a PC machine, 512 MiB
RAM, standard VGA, serial on stdio, BIOS boot order, and no reboot. Extra
arguments are passed to QEMU.

**Options:** QEMU options accepted by the installed QEMU; the script adds no
own option parser.

**Requirements:** Existing `target/x86_64-unknown-none/debug/bootimage-nova-kernel.bin`
and `qemu-system-x86_64` on the host.

**Common errors:** If the image is missing, build first. QEMU may not be
installed or may reject an option.

**Related commands:** [`./tools/build.sh`](#command-tools-buildsh).

**Implementation notes:** QEMU currently shows VGA text startup/status
information and the offline software-center preview, then the kernel halts.
This is not a graphical desktop session.

### Direct host Cargo commands

These are Cargo commands executed on the development host, not Nova shell
commands. The workspace uses the nightly toolchain in `rust-toolchain.toml`.

| Command | Status | Actual use |
|---|---|---|
| `cargo test --workspace --lib` | HOST ONLY | Library tests across workspace members. |
| `cargo test -p nova-dev` | HOST ONLY | Developer CLI tests. |
| `cargo check --workspace` | HOST ONLY | Workspace type/build check. |
| `cargo check -p nova-kernel --target x86_64-unknown-none` | HOST ONLY | Bare-metal kernel target check. |
| `cargo bootimage -p nova-kernel --target x86_64-unknown-none` | HOST ONLY | Boot-image build used by `tools/build.sh`. |
| `cargo fmt --all -- --check` | HOST ONLY | Formatting verification. |
| `cargo clippy` | HOST ONLY | Cargo/rust-clippy may be used if installed, but this repository's prescribed test script does not invoke it; not recorded as Phase validation. |
| `make`, `make run`, `make clean` | NOT IMPLEMENTED | No Makefile was found at repository root. |

## Category H: Boot, kernel, and system diagnostics

Kernel startup diagnostics are written to VGA and serial during boot, not
through interactive commands. The panic handler attempts to display a panic
message; there is no persistent log reader, reboot command, shutdown command,
or recovery shell.

| Name | Status | Notes |
|---|---|---|
| `nova-info`, `nova-log`, `nova-diagnostics` | NOT IMPLEMENTED | No such command binaries. |
| `nova-reboot`, `nova-shutdown` | NOT IMPLEMENTED | No user command or power-management interface. |
| `recovery` / recovery-mode command | NOT IMPLEMENTED | No interactive recovery mode. |
| `dmesg`, `journalctl` | NOT IMPLEMENTED | No command-accessible logs. |

## Category I: Networking

No active network driver, networking stack, DNS service, or network command
exists.

| Names | Status |
|---|---|
| `ip`, `ping`, `traceroute`, `nslookup`, `curl`, `wget`, `netstat`, `ss` | NOT IMPLEMENTED |

There is no supported Nova syntax or connectivity test. Host `curl` is a host
utility and does not imply Nova networking support.

## Category J: Users, permissions, and security

Nova has bounded account metadata and pure permission-policy helpers, but no
authentication, privilege enforcement, filesystem permission enforcement, or
user administration command.

| Names | Status | Notes |
|---|---|---|
| `id`, `groups`, `whoami` | NOT IMPLEMENTED | No interactive account-query command. |
| `chmod`, `chown` | NOT IMPLEMENTED | No usable Nova filesystem. |
| `sudo`, `su` | NOT IMPLEMENTED | No shell, authentication, or privilege-transition command. Do not assume privilege enforcement. |
| `passwd` | NOT IMPLEMENTED | Authentication/password change service unavailable. |
| `nova-lock` / session lock command | NOT IMPLEMENTED | No session-lock command/service. |
| Security-log command | NOT IMPLEMENTED | Bounded in-memory event metadata is not exposed through a command. |

Never put credentials in command examples. Do not attempt to bypass missing
authentication or permission enforcement.

## Category K: Storage and filesystem management

No active storage driver, mounted filesystem, partition utility, or filesystem
repair tool exists.

| Names | Status | Notes |
|---|---|---|
| `lsblk`, `blkid`, `mount`, `umount`, `fsck` | NOT IMPLEMENTED | No storage command interface. |
| `df`, `du` | NOT IMPLEMENTED | No filesystem usage accounting. |

No partitioning, formatting, mounting, unmounting, or repair operation is
supported. Do not infer storage support from MMIO/device metadata.

## Category L: Hardware and device information

The kernel has bounded device/resource metadata and host-testable PCI parsing,
but no interactive hardware command. PCI hardware scanning and active
drivers are unavailable.

| Names | Status | Notes |
|---|---|---|
| `lspci`, `lsusb`, `lscpu`, `lsmem`, `lsmod` | NOT IMPLEMENTED | No corresponding Nova command. |
| Network/audio/battery/graphics inspection commands | NOT IMPLEMENTED | No active interfaces or device-status command API. |

## Category M: Logs, debugging, and troubleshooting

There is no command shell or command-accessible log store. At boot, inspect
the host's QEMU serial output or VGA screen. This exposes only the kernel's
startup messages and panic path.

| Name | Status | Notes |
|---|---|---|
| `dmesg`, `journalctl`, `nova-log`, `nova-diagnostics` | NOT IMPLEMENTED | No log-query or report-generation command. |
| `nova-dev doctor` | HOST ONLY | Checks host development prerequisites, not Nova runtime health. |
| `./tools/run_qemu.sh` | HOST ONLY | Useful for observing current boot text in QEMU. |

For common issues, use the [troubleshooting workflows](docs/cli/EXAMPLES.md#workflow-10-recover-from-common-problems).

## Category N: Desktop environment commands

The current boot process does not start a graphical desktop or application
runtime. Browser preview pages are separate host prototypes.

| Names | Status | Notes |
|---|---|---|
| Launch application / `open` command | NOT IMPLEMENTED | Application execution unavailable. |
| Open file manager / terminal / settings commands | NOT IMPLEMENTED | These native applications do not run in Nova. |
| Display/session/window-manager commands | NOT IMPLEMENTED | No active graphical session or command API. |
| Restart desktop service | NOT IMPLEMENTED | No desktop shell or service manager. |

See [desktop integration status](docs/desktop-integration.md) and [Phase 14
audit](docs/phase-14-audit.md).

## Shell compatibility

Nova does not implement a POSIX shell. The following are all
**NOT IMPLEMENTED** in Nova: command chaining, pipes, input/output
redirection, environment variables, quoting/escaping rules, command history,
tab completion, background jobs, shell scripts, and external executable
lookup. Host Bash examples in development documentation apply only to the
host shell.

## Supported host command quick reference

| Command | Where | Purpose |
|---|---|---|
| `nova-dev new NAME` | Host | Create a host Rust scaffold. |
| `nova-dev build/run/test/clean` | Host | Forward one Cargo operation in the current project. |
| `nova-dev info/doctor` | Host | Print SDK status or check host prerequisites. |
| `./tools/test.sh` | Host | Run the repository's tests and workspace checks. |
| `./tools/build.sh` | Host | Build the kernel boot image. |
| `./tools/run_qemu.sh` | Host | Run the image in QEMU. |
| `cargo fmt --all -- --check` | Host | Check formatting. |
| `cargo check -p nova-kernel --target x86_64-unknown-none` | Host | Check the bare-metal kernel target. |

For complete practical sequences see [`docs/cli/EXAMPLES.md`](docs/cli/EXAMPLES.md).
