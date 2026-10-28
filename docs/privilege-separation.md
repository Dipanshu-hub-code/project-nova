# Privilege Separation

Kernel/user mode separation and process address-space isolation are not
implemented in this repository. There is no user-mode process loader or syscall
interface.

The internal `PrivilegeContext` is a pure policy test value. It is not created
from an authenticated login session and does not enforce hardware privilege
boundaries. No application should be treated as isolated or trusted based on
this type.
