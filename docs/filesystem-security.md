# Filesystem Security

No filesystem is mounted or implemented. `FileAccess` is a pure permission
policy helper that checks owner and other read/write/execute bits against a
caller-provided identity. It does not open files, mediate I/O, authenticate
callers, or enforce permissions in the kernel.

Real filesystem security needs trusted ownership metadata, a protected
filesystem implementation, a verified caller identity, and mandatory checks
on every access path.
