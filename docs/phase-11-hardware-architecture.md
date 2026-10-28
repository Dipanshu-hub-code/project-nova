# Phase 11 Hardware Architecture

## Implemented foundations

- `kernel/src/hal/` contains explicit status types and validated physical
  ranges. It does not map MMIO or install interrupt handlers.
- `kernel/src/devices/registry.rs` stores up to 32 device records in
  caller-owned memory.
- `kernel/src/devices/resource.rs` describes MMIO ranges, inclusive I/O port
  ranges, and IRQ identifiers and detects overlaps.
- `kernel/src/devices/pci.rs` parses a 256-byte PCI configuration header and
  offers a callback-based, allocation-free bus scanner. Callers must supply
  configuration reads; no hardware access implementation is included.
- `kernel/src/drivers/driver.rs` defines a minimal lifecycle interface. No
  storage, network, audio, display, or input driver is bound.

## Safety and integration boundaries

Physical range validation is not a mapping permission. Before any real driver
is added, the kernel needs an owned physical-memory mapping API, PCI
configuration-space access, resource reservation, and interrupt dispatch.
Device-provided data must be validated, and unsupported devices must remain
reported as unsupported rather than being represented by success-shaped
stubs.

Storage, networking, audio, ACPI power methods, and firmware loading remain
unavailable. The static software-center preview does not exercise any of these
interfaces.
