# Phase 12 Testing

Run the kernel library's host-side tests:

```sh
./tools/test.sh
```

Tests cover bounded account metadata, username validation, pure permission
policies, bounded security-event FIFO behavior, and fail-closed authentication,
cryptography, encryption, integrity, and recovery APIs.

These are logic tests only. They do not establish filesystem security,
credential security, process isolation, cryptographic correctness, or boot
integrity. QEMU does not provide a Phase 12 login or recovery workflow.
