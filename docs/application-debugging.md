# Application Debugging

Host Rust templates can be built and tested with Cargo through `nova-dev`.
Kernel boot diagnostics are emitted to the VGA text console and COM1 serial
port. The current project has no Nova process logs, application exit-status
service, user-mode debugger, or interactive syscall tracing.

`nova-dev doctor` is an environment check, not a kernel debugger.
