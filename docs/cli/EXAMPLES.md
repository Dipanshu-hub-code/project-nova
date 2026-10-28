# Project Nova CLI Examples and Workflows

## Important distinction

Nova currently has no interactive terminal, shell, or filesystem. The
following workflows label host commands explicitly. A host command does not
operate inside Nova, and a proposed Nova utility must not be interpreted as
available. See the [command status matrix](./COMMAND_IMPLEMENTATION_STATUS.md).

## Workflow 1: Create a project directory

**Nova status:** `mkdir`, `cd`, and `pwd` are not implemented in Nova.

**Host-only alternative:** In a host Bash terminal, ordinary host shell
commands can create and enter a development directory:

```sh
mkdir my-project
cd my-project
pwd
```

These commands operate only on the host filesystem. From the project root,
`nova-dev new my-project` is another host-only option; it creates a complete
host Rust scaffold and refuses to overwrite an existing destination.

## Workflow 2: Create and inspect files

**Nova status:** No Nova file create/list/read/copy/move/delete commands exist;
the Nova filesystem API is unavailable. Do not run these operations as Nova
examples.

**Host-only alternative:** Use the host OS's file manager or shell in a
disposable host test directory. This does not read or modify a Nova disk image.
No destructive host command is run by this guide.

## Workflow 3: Search for files

**Nova status:** No Nova `find`, `grep`, index, or filesystem API exists.

**Alternative:** Inspect host project files with the host's search tools. Host
search results are not Nova filesystem results.

## Workflow 4: Install software

**Nova status:** Not available. There is no `nova-pkg`, package repository
transport, signature verification, installer transaction, or functional
Software Center. No install or remove command is provided.

## Workflow 5: Inspect system resources

**Nova status:** No `top`, `free`, `df`, `ps`, or system metrics command. The
kernel does not expose CPU utilization, allocator statistics, process lists,
or storage usage. The boot screen's memory-region count is not free/used
memory.

**Host-only benchmark:** The project has a host test for one software
framebuffer operation, not a general system monitor:

```sh
cargo test -p nova-kernel --lib benchmark_framebuffer_fill_rect -- --ignored --nocapture
```

It measures host memory-buffer fills and does not measure Nova runtime
resources or physical graphics.

## Workflow 6: Diagnose an application failure

**Nova status:** Nova applications cannot be launched, and there is no
application/service log command or diagnostic report generator.

**Host-only development checks:** From the repository root, run
`./tools/test.sh` for workspace tests/checks or `nova-dev doctor` for basic
host prerequisite checks. `doctor` prints missing tools but currently exits
successfully even when some are missing.

## Workflow 7: Inspect network connectivity

**Nova status:** No network interfaces, network stack, DNS, `ip`, or `ping`
command is implemented. No Nova connectivity test is possible.

**Host-only alternative:** Host network tools inspect the development
machine's network only and cannot validate Nova networking.

## Workflow 8: Build and test Project Nova

Run in Ubuntu/WSL2 from the repository root:

```sh
./tools/test.sh
cargo check -p nova-kernel --target x86_64-unknown-none
./tools/build.sh
./tools/run_qemu.sh
```

`tools/test.sh` runs workspace library tests, `nova-dev` tests, and workspace
checks. `build.sh` creates the kernel boot image. `run_qemu.sh` starts the
current kernel in QEMU. QEMU currently shows VGA text diagnostics and halts;
it does not launch a graphical desktop or native application suite.

For a generated host project, use:

```sh
nova-dev new nova-demo
cd nova-demo
nova-dev test
nova-dev run
```

The generated program runs on the host, not Nova.

## Workflow 9: Shut down or reboot safely

There is no Nova shutdown/reboot command or interactive session. QEMU can be
closed from the host; this stops the emulator, not a Nova-managed shutdown
sequence. No command that claims graceful shutdown is available.

## Workflow 10: Recover from common problems

| Problem | Accurate next step |
|---|---|
| A command is not found in Nova | Nova has no shell. Check this command reference; do not assume Linux utilities exist. |
| A native application will not launch | Native process launch is unavailable; this is a project limitation, not an application command failure. |
| Package install fails | Package installation is not implemented. The Software Center is preview-only. |
| Network is unavailable | Nova has no active networking backend or command. Host networking is separate. |
| Build fails | On the host, run `nova-dev doctor`, inspect the first Cargo error, then retry the documented test/build command after correcting host prerequisites. |
| QEMU image is missing | On the host, run `./tools/build.sh` before `./tools/run_qemu.sh`. |
| QEMU appears stuck | Current kernel is designed to halt after boot diagnostics. Observe VGA/serial output; this is not evidence of a shell or GUI lockup. |
| A host `nova-dev` subcommand fails | Confirm the current directory has `Cargo.toml` for build/run/test/clean; `doctor` checks basic host tools but does not install them. |

## Availability summary

- **Available only on the host:** `nova-dev` scaffolding/Cargo forwarding,
  repository test/build/QEMU scripts, and direct Cargo commands.
- **Partially implemented:** package metadata/planning and an offline VGA/
  browser Software Center preview; neither is an installable CLI.
- **Not implemented in Nova:** all interactive shell, file, process, package,
  network, storage, account, diagnostic, and desktop commands listed in the
  reference.
