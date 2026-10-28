# Nova SDK

The SDK workspace currently contains:

- `nova-std`: host-buildable API facade with explicit errors for unavailable
  Nova services.
- `nova-ui`: validated `Window`, `Button`, and `Label` metadata, plus shared
  theme tokens, focus traversal, button state, a bounded single-line text
  field, and layout values. These are host-side logic primitives; rendering,
  window display, and Nova input dispatch are unavailable.
- CLI and GUI sample projects under `sdk/templates/`.
- `nova-dev`: host Rust project scaffolding, Cargo command forwarding, SDK
  information, and environment checks.

The SDK does not yet target a working Nova user-space ABI. Crate version
`0.1.0` is pre-release and no binary compatibility guarantee is made.
