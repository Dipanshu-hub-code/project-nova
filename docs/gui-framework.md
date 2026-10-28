# GUI Framework (`nova-ui`)

`nova-ui` provides host-side window metadata and reusable presentation/state
primitives:

- Light and dark palette tokens, spacing, and corner-radius values.
- A bounded focus manager for deterministic forward/reverse keyboard traversal.
- Buttons with primary/secondary/destructive/quiet variants, enabled/focused
  state, and keyboard activation checks.
- A bounded, UTF-8-safe single-line text field with an accessible label.
- Checked size, inset, and stack-layout values.

These components are unit-tested host-side state; they are not a renderer.
The crate does not paint pixels, dispatch keyboard/mouse events, create a
kernel window, or run as a Nova application. Calling `Window::show` and
`nova_ui::init` returns `NotSupported`. The SDK theme tokens are not connected
to the kernel's in-memory settings model or a running desktop.

The interactive desktop mockup in `preview/nova-desktop.html` is a separate
browser prototype and does not use this Rust library.
