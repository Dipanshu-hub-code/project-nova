# Phase 13 Audit

The project now contains a developer-facing API/SDK foundation, but it does not
provide a runnable Nova user-space environment.

| Area | Status |
|---|---|
| Versioned syscall number definitions | Defined as experimental ABI v1 |
| Syscall entry and dispatch | Missing; validation-only dispatcher returns `NotSupported` |
| User-mode execution and process isolation | Missing |
| `nova-std` | Host-buildable facade; system calls return `NotSupported` |
| `nova-ui` | Host-buildable widget metadata; window display returns `NotSupported` |
| `nova-dev` | Host-project scaffold/build/run/test/clean/info/doctor commands |
| `.npkg` packaging and signing | Not implemented; package command reports unavailable |
| CLI and GUI application templates | Compile as host programs; they are not Nova executables |
| Graphical desktop in kernel | Not implemented; see the browser desktop preview |

No public syscall reads user pointers or issues a `syscall` instruction. This is
intentional: the kernel has no user address-space validation, privilege
transition, executable loader, or window service to safely support those
operations.
