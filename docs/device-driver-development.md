# Device Driver Development

The current driver trait defines `name`, device matching, initialization, and
state. It is only an interface; the kernel does not enumerate hardware or bind
drivers yet.

Before adding an active driver:

- Validate all device-provided values and lengths.
- Require a verified mapping before dereferencing MMIO.
- Reserve port and memory resources before use.
- Keep interrupt handlers short and defer nontrivial work.
- Use owned, correctly aligned DMA buffers only after DMA mapping exists.
- Return explicit errors when the controller or required subsystem is absent.
- Add host tests for pure parsing and state logic; do not simulate successful
  hardware initialization with placeholder success values.
