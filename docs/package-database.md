# Package Database

The current `PackageDatabase` is a bounded, caller-owned in-memory data
structure. It tracks package metadata and installed/removed state only; it does
not track files, timestamps, repository origin, signature status, or
transaction history. It is discarded when its owner is discarded or the
system restarts.

The planned persistent location is `/var/lib/nova/packages.db`, subject to a
future filesystem implementation. Persistence should use atomic replacement
and a transaction journal for crash recovery. Neither persistence nor
filesystem-backed install/remove is implemented.
