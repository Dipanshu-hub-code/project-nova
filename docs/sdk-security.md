# SDK Security

The SDK does not issue privileged instructions or expose raw hardware access.
Its system functions return `NotSupported` because the kernel currently has no
user-mode syscall boundary.

The permission policy helpers elsewhere in the project are not enforced by a
filesystem. Applications must not treat a host-built template as isolated,
authenticated, or safe to run with Nova privileges.
