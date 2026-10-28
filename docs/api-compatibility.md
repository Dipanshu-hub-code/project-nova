# API and ABI Compatibility

The syscall number definitions are experimental ABI v1. They have no
compatibility guarantee because there is no syscall entry point, process
loader, or released user-space runtime.

SDK crates are version `0.1.0`. Future stable API releases must document
version negotiation, calling conventions, argument validation, and error
mapping before applications can depend on them.
