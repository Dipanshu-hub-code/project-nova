# Desktop Architecture — Phase 8 foundation

## What is implemented

- 32-bit RGB/BGR framebuffer access with checked layout, bounds, and stride.
- Clipped rectangle rendering, color definitions, and a fixed-capacity window
  stack that does not require a kernel heap.
- Basic desktop wallpaper, panel, dock, and window composition primitives.
- Shared keyboard/mouse event types and explicit driver availability status.
- Application API placeholders for terminal, editor, file manager, and settings.

## Current hardware limitations

The BIOS bootloader used by this project does not provide a framebuffer to the
kernel. No VBE/UEFI GOP or virtual-GPU driver is configured, so desktop drawing
functions are not called at boot and no graphical desktop is displayed.
Framebuffer construction is unsafe by design: a future display driver must
provide a valid mapped 32-bit buffer and metadata.

Keyboard and mouse drivers, text/font rendering, application processes, and
userspace are not implemented. Their APIs must not be interpreted as working
hardware or launchable applications.

## Boot behavior

The kernel reports the framebuffer and input driver limitations over VGA text
and COM1 serial, then halts. Once a display driver is available, a caller can
construct a framebuffer, create a `WindowManager`, and call `Desktop::render`.

## Phase 14 shared UI SDK

The host-buildable `nova-ui` crate contains tested theme, focus, control-state,
and layout primitives for future applications. They are not connected to this
kernel renderer or to device input, and do not change the graphical desktop.
See the [Phase 14 architecture](./phase-14-architecture.md).
