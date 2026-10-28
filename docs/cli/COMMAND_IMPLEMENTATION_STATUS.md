# Command Implementation Status

Status definitions: `IMPLEMENTED` means the command operation exists;
`PARTIAL` means an explicitly limited implementation exists;
`NOT IMPLEMENTED` means no usable command exists; `HOST ONLY` means it is a
development-machine tool, not a Nova command; `PLANNED` means proposed but not
implemented. The **Tested** column refers only to the actual implementation,
not a related metadata helper.

## Existing host command-line surfaces

| Command | Category | Status | Tested | Notes |
|---|---|---|---|---|
| `nova-dev` | Development | HOST ONLY | Yes (project-name validation only) | Rust CLI; usage shown when called without arguments. The unit test covers name validation, not every command path. No global `--help`. |
| `nova-dev new` | Development | HOST ONLY | Yes (name validation only) | Creates a host Rust project without overwriting an existing directory; complete filesystem scaffold flow has no dedicated integration test. |
| `nova-dev build` | Development | HOST ONLY | No command-specific test | Invokes `cargo build` in current directory. |
| `nova-dev run` | Development | HOST ONLY | No command-specific test | Invokes host `cargo run` in current directory. |
| `nova-dev test` | Development | HOST ONLY | No command-specific test | Invokes host `cargo test`; CLI crate tests run separately. |
| `nova-dev clean` | Development | HOST ONLY | No command-specific test | Invokes host `cargo clean`. |
| `nova-dev info` | Development | HOST ONLY | No command-specific test | Prints tool/API availability status. |
| `nova-dev doctor` | Development | HOST ONLY | No command-specific test | Checks cargo, rustc, QEMU, and target; missing tools are printed but do not cause a nonzero result. |
| `nova-dev package` | Package | NOT IMPLEMENTED | No | Recognized placeholder returns unavailable error. |
| `./tools/test.sh` | Development | HOST ONLY | Yes | Workspace library tests, CLI test, and workspace check. |
| `./tools/build.sh` | Development | HOST ONLY | Yes | `cargo bootimage` kernel build script. |
| `./tools/run_qemu.sh` | Diagnostics | HOST ONLY | Yes | QEMU wrapper; text-mode startup has been observed. |
| `cargo test --workspace --lib` | Development | HOST ONLY | Yes | Host test runner command used by repository script. |
| `cargo test -p nova-dev` | Development | HOST ONLY | Yes | Developer CLI tests. |
| `cargo check --workspace` | Development | HOST ONLY | Yes | Workspace host check. |
| `cargo check -p nova-kernel --target x86_64-unknown-none` | Development | HOST ONLY | Yes | Bare-metal target check. |
| `cargo bootimage -p nova-kernel --target x86_64-unknown-none` | Development | HOST ONLY | Yes | Builds boot image. |
| `cargo fmt --all -- --check` | Development | HOST ONLY | Yes | Formatting check. |
| `cargo build` | Development | HOST ONLY | Yes | Invoked by `nova-dev build` and used by workspace builds. |
| `cargo run` | Development | HOST ONLY | No command-specific test | Invoked by `nova-dev run` for host programs. |
| `cargo test` | Development | HOST ONLY | Yes | Invoked by `nova-dev test`; workspace prescribed selector is `--workspace --lib`. |
| `cargo clean` | Development | HOST ONLY | No command-specific test | Invoked by `nova-dev clean` on host artifacts. |
| `cargo clippy` | Development | HOST ONLY | No | Host toolchain command; not part of prescribed repository validation. |
| `make`, `make run`, `make clean` | Development | NOT IMPLEMENTED | No | No repository Makefile. |

## Nova essential and file commands

| Command | Category | Status | Tested | Notes |
|---|---|---|---|---|
| `help` | Essential | NOT IMPLEMENTED | No | No shell/dispatcher. |
| `clear` | Essential | NOT IMPLEMENTED | No | Boot clears VGA internally; no command. |
| `exit` | Essential | NOT IMPLEMENTED | No | No shell session. |
| `history` | Essential | NOT IMPLEMENTED | No | No shell input history. |
| `echo` | Essential | NOT IMPLEMENTED | No | No shell built-ins. |
| `pwd` | Essential | NOT IMPLEMENTED | No | No shell/filesystem. |
| `whoami` | Essential/security | NOT IMPLEMENTED | No | Account metadata is not exposed through a command. |
| `date` | Essential | NOT IMPLEMENTED | No | No clock command. |
| `hostname` | Essential | NOT IMPLEMENTED | No | No hostname command. |
| `uname` | Essential/system | NOT IMPLEMENTED | No | No system-information command. |
| `ls` | Filesystem | NOT IMPLEMENTED | No | No usable filesystem. |
| `cd` | Filesystem | NOT IMPLEMENTED | No | No shell working directory. |
| `mkdir`, `rmdir`, `touch` | Filesystem | NOT IMPLEMENTED | No | No Nova file operations. |
| `cp`, `mv`, `rm` | Filesystem | NOT IMPLEMENTED | No | No copy/move/delete. |
| `cat`, `less`, `head`, `tail` | Filesystem | NOT IMPLEMENTED | No | No Nova file-reading commands. |
| `file`, `stat`, `find`, `tree` | Filesystem | NOT IMPLEMENTED | No | No file metadata/search commands. |

## Nova text, system, and process commands

| Command | Category | Status | Tested | Notes |
|---|---|---|---|---|
| `grep`, `sort`, `uniq`, `wc`, `cut`, `tr`, `diff`, `sed`, `awk` | Text/search | NOT IMPLEMENTED | No | No command runtime, pipes, or redirection. |
| `uptime` | System | NOT IMPLEMENTED | No | HAL time returns `TimerUnavailable`. |
| `free`, `vmstat`, `lscpu`, `lsmem` | System | NOT IMPLEMENTED | No | No allocator/accounting/system metrics API. |
| `df`, `du` | System/storage | NOT IMPLEMENTED | No | No filesystem/storage subsystem. |
| `ps`, `top` | System/process | NOT IMPLEMENTED | No | No process scheduler/table. |
| `dmesg`, `journalctl` | Logs | NOT IMPLEMENTED | No | Boot log printed to VGA/serial; no log query command. |
| `kill`, `jobs`, `bg`, `fg` | Process | NOT IMPLEMENTED | No | No process execution or shell job control. |
| `systemctl`, `service` | Services | NOT IMPLEMENTED | No | No Linux systemd-compatible manager. |

## Nova package, diagnostics, network, security, storage, hardware, desktop

| Command | Category | Status | Tested | Notes |
|---|---|---|---|---|
| `nova-pkg` and `search`, `install`, `remove`, `update`, `upgrade`, `list`, `info`, `verify`, `clean` | Package | NOT IMPLEMENTED | No | Metadata/planning exists; no package CLI or installer. |
| `nova-info`, `nova-log`, `nova-diagnostics` | Diagnostics | NOT IMPLEMENTED | No | No command binaries. |
| `nova-reboot`, `nova-shutdown` | Power | NOT IMPLEMENTED | No | No user power command/API. |
| `recovery` | Recovery | NOT IMPLEMENTED | No | No interactive recovery mode. |
| `ip`, `ping`, `traceroute`, `nslookup`, `curl`, `wget`, `netstat`, `ss` | Network | NOT IMPLEMENTED | No | No active network stack or utilities. |
| `id`, `groups`, `chmod`, `chown`, `sudo`, `passwd`, `su` | Security | NOT IMPLEMENTED | No | No command, authentication, permission enforcement, or privilege transition. |
| `nova-lock` | Security/session | NOT IMPLEMENTED | No | No session-lock command/service. |
| `lsblk`, `blkid`, `mount`, `umount`, `fsck` | Storage | NOT IMPLEMENTED | No | No storage/filesystem. |
| `lspci`, `lsusb`, `lsmod` | Hardware | NOT IMPLEMENTED | No | Metadata/parser foundations do not provide interactive utilities or active discovery. |
| `nova-open` and desktop launch commands | Desktop | NOT IMPLEMENTED | No | No graphical session or process launcher. |
| Graphical Software Center | Desktop/package | PARTIAL | Yes (VGA screen size test only) | Static preview; does not install/launch. |
| `man` | Help | NOT IMPLEMENTED | No | No Nova shell/manual command. |
| POSIX shell, pipes, redirects, variables, history, completion | Shell | NOT IMPLEMENTED | No | No interactive shell. |

## Evidence

Implementation audit references:

- [`tools/nova-dev/src/main.rs`](../../tools/nova-dev/src/main.rs)
- [`tools/test.sh`](../../tools/test.sh)
- [`tools/build.sh`](../../tools/build.sh)
- [`tools/run_qemu.sh`](../../tools/run_qemu.sh)
- [`kernel/src/runtime/launcher.rs`](../../kernel/src/runtime/launcher.rs)
- [`sdk/libraries/nova-std/src/fs.rs`](../../sdk/libraries/nova-std/src/fs.rs)
- [`kernel/src/hal/time.rs`](../../kernel/src/hal/time.rs)
- [`kernel/src/pkg/installer.rs`](../../kernel/src/pkg/installer.rs)
- [Phase 14 audit](../phase-14-audit.md)
- [Phase 15 audit](../phase-15-audit.md)
