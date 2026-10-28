# Phase 12 Security Architecture

The current security module is a fail-closed foundation, not an implemented
security subsystem.

## Current module responsibilities

- `accounts`: bounded in-memory user metadata without credentials.
- `auth`: explicit unavailable result; it accepts no password and performs no
  authentication.
- `privilege` and `fs_perms`: internal, pure policy values that are not
  connected to a syscall or filesystem boundary.
- `crypto`, `encryption`, and `integrity`: report unavailable and never return
  a successful cryptographic or signature operation.
- `logging`: bounded caller-owned event queue with fixed event types, not a
  protected or persistent audit log.
- `recovery`: returns an unavailable result.

## Required foundations

Before any real access control can be claimed, Nova needs kernel/user
privilege separation, isolated address spaces, validated syscalls, a trusted
filesystem with ownership enforcement, persistent protected account storage,
a maintained cryptographic library, a secure entropy source, and provisioned
trust anchors. Never replace missing cryptography with a custom algorithm or
an always-success stub.
