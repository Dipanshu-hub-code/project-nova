# Phase 15 Performance and Validation Record

## Instrumentation

An ignored host-side microbenchmark in the kernel library tests exercises the
software `Framebuffer::fill_rect` implementation. It fills a 640x480 RGB
buffer 25 times per sample, records five durations using `std::time::Instant`,
sorts them, and reports the median and decimal MB/s. It runs only when invoked
explicitly, so it adds no overhead to normal kernel operation.

Run from the repository root:

```sh
cargo test -p nova-kernel --lib benchmark_framebuffer_fill_rect -- --ignored --nocapture
```

This is a test harness, not a Nova user command: Nova has no shell or process
runtime. CPU usage, uptime, memory allocation, scheduler latency, interrupt
counts, application startup, filesystem latency, and graphical presentation
latency remain unavailable and are not represented by invented values.

## Optimization record

| Field | Result |
|---|---|
| Subsystem / component | Software graphics / `Framebuffer::fill_rect` |
| Problem | Repeated per-pixel method checks and checked offset arithmetic in a bounded fill loop |
| Baseline | 1,258,645 us median for 25 full 640x480 fills; 24 MB/s in host debug-profile test |
| Implementation | Validate rectangle/row spans once; select RGB/BGR channel order once; retain volatile pixel writes |
| Result | Two post-change medians: 230,762 us (133 MB/s) and 260,953 us (117 MB/s) |
| Correctness tests | Framebuffer channel ordering, stride, bounds, renderer clipping, untouched surrounding pixels and row padding |
| Trade-offs | Simplifies per-pixel path; still performs four volatile byte writes per pixel. Host measurements do not predict real display hardware. |
| Status | Implemented and host-tested; hardware/display integration unverified because no display driver is active. |

## Validation

Completed during Phase 15:

- Baseline `./tools/test.sh`: passed before changes (32 kernel library tests,
  2 `nova-std`, 8 `nova-ui`, 1 `nova-dev`; workspace check).
- Baseline `cargo check -p nova-kernel --target x86_64-unknown-none`: passed.
- Baseline `./tools/build.sh`: passed.
- Post-change framebuffer tests: passed (5 tests; benchmark excluded unless
  selected explicitly).
- Post-change host benchmark: completed twice; results are above.
- Post-change `./tools/test.sh`: passed (33 kernel tests, 1 benchmark ignored,
  2 `nova-std`, 8 `nova-ui`, 1 `nova-dev`; workspace checks passed).
- Post-change `cargo fmt --all -- --check`: passed.
- Post-change `cargo check -p nova-kernel --target x86_64-unknown-none`:
  passed.
- Post-change `./tools/build.sh`: passed and produced the boot image.
- QEMU text-mode boot: verified the expected status and preview limitation
  screen; QEMU was intentionally stopped after 8 seconds. Graphical rendering,
  hardware, and physical-device behavior are not implied by host tests or the
  boot-image build.

## Current validation boundaries

QEMU can verify the kernel's VGA text startup/status output only. The kernel
prints that framebuffer, keyboard, mouse, process launch, persistence,
installation, authentication, and crypto services are unavailable, then
halts. There is no live desktop against which to measure input latency, window
movement, presentation latency, redraw count, application lifecycle, or
resource leaks.
