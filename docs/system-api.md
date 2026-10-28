# System API

`kernel/src/abi/` defines experimental API version 1 syscall numbers and
explicit error codes. `kernel/src/syscall/` validates the API version and
syscall number but returns `NotSupported` for every currently defined call.
It never interprets syscall arguments as pointers.

The `nova-std` facade exposes process, filesystem, time, and logging function
shapes, all of which return `NotSupported` because the kernel has no syscall
entry point or corresponding services. No stable syscall ABI or binary
compatibility guarantee is claimed.
