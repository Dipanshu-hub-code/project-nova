# User Accounts

The current `AccountDatabase` stores up to 32 account metadata records in
caller-owned memory. UIDs are allocated monotonically from 1000, usernames are
restricted to lowercase ASCII letters, digits, hyphens, and underscores, and
home paths are generated as `/home/<username>`.

The records are volatile and are not tied to filesystem ownership or an
authenticated session. Passwords and password hashes are intentionally not
stored. Persistence, account deletion, and authentication are not implemented.
