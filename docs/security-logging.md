# Security Logging

`SecurityLog` is a fixed-capacity, caller-owned FIFO containing enumerated
events only. It accepts no free-form strings or secrets and reports queue-full
instead of silently overwriting entries.

It is not a protected, persistent, tamper-resistant, or globally shared audit
log. No authentication or privileged operation is currently available to
produce trusted security audit records.
