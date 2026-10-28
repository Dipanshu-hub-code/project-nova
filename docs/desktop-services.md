# Desktop Services

- **Application manager:** owns a bounded manifest registry. Launch requests
  return `ProcessExecutionUnavailable`.
- **Session:** cannot start until display, input, and process execution
  backends exist.
- **Notifications:** bounded, in-memory FIFO with validated UTF-8 text lengths.
  Caller identity and application authorization are not implemented.
- **Settings:** in-memory light/dark theme value; filesystem persistence is
  unavailable.
- **Logging:** typed log records are available, but no serial, file, or
  ring-buffer sink is connected.

These are kernel foundations, not isolated user-facing desktop services.
