# Project Nova Command Index

This index covers the host development CLI and the requested Nova command
families. Nova currently has no interactive shell; entries marked
**NOT IMPLEMENTED** cannot be run inside Nova.

## Available host commands

| Command | Status | Reference |
|---|---|---|
| `nova-dev` | HOST ONLY | [Main command reference](../../NOVA_TERMINAL_COMMANDS.md#command-nova-dev) |
| `nova-dev new` | HOST ONLY | [Scaffold project](../../NOVA_TERMINAL_COMMANDS.md#command-nova-dev-new) |
| `nova-dev build` | HOST ONLY | [Build host project](../../NOVA_TERMINAL_COMMANDS.md#command-nova-dev-build) |
| `nova-dev run` | HOST ONLY | [Run host project](../../NOVA_TERMINAL_COMMANDS.md#command-nova-dev-run) |
| `nova-dev test` | HOST ONLY | [Test host project](../../NOVA_TERMINAL_COMMANDS.md#command-nova-dev-test) |
| `nova-dev clean` | HOST ONLY | [Clean host project](../../NOVA_TERMINAL_COMMANDS.md#command-nova-dev-clean) |
| `nova-dev info` | HOST ONLY | [SDK status](../../NOVA_TERMINAL_COMMANDS.md#command-nova-dev-info) |
| `nova-dev doctor` | HOST ONLY | [Host prerequisites](../../NOVA_TERMINAL_COMMANDS.md#command-nova-dev-doctor) |
| `nova-dev package` | NOT IMPLEMENTED | [Package status](../../NOVA_TERMINAL_COMMANDS.md#command-nova-dev-package) |
| `./tools/test.sh` | HOST ONLY | [Workspace tests](../../NOVA_TERMINAL_COMMANDS.md#command-tools-testsh) |
| `./tools/build.sh` | HOST ONLY | [Kernel image](../../NOVA_TERMINAL_COMMANDS.md#command-tools-buildsh) |
| `./tools/run_qemu.sh` | HOST ONLY | [QEMU boot](../../NOVA_TERMINAL_COMMANDS.md#command-tools-run_qemush) |

## Requested command categories

| Category | Commands currently available in Nova |
|---|---|
| Essential terminal (`help`, `clear`, `exit`, `history`, `echo`, `pwd`, `whoami`, `date`, `hostname`, `uname`) | None |
| Files and directories (`ls`, `cd`, `mkdir`, `rmdir`, `touch`, `cp`, `mv`, `rm`, `cat`, `less`, `head`, `tail`, `file`, `stat`, `find`, `tree`) | None |
| Text/search (`grep`, `sort`, `uniq`, `wc`, `cut`, `tr`, `diff`, `sed`, `awk`) | None |
| System information (`uptime`, `free`, `df`, `du`, `ps`, `top`, `vmstat`, `lscpu`, `lsmem`, `dmesg`, `journalctl`) | None |
| Process/services (`kill`, `jobs`, `bg`, `fg`, `systemctl`, `service`) | None |
| Packages (`nova-pkg` and subcommands) | None |
| Networking (`ip`, `ping`, `traceroute`, `nslookup`, `curl`, `wget`, `netstat`, `ss`) | None |
| Users/security (`id`, `groups`, `chmod`, `chown`, `sudo`, `passwd`, `su`) | None |
| Storage (`lsblk`, `blkid`, `mount`, `umount`, `fsck`) | None |
| Hardware (`lspci`, `lsusb`, `lsmod`) | None |
| Desktop (`nova-open`, file-manager/terminal/settings launch commands) | None |

The full status for every inventoried command appears in the
[implementation matrix](./COMMAND_IMPLEMENTATION_STATUS.md).
