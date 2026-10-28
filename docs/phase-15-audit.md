# Phase 15 Repository Audit

## Architecture and verified baseline

Project Nova remains an experimental x86_64 `no_std` kernel. The bootloader
hands off a BIOS boot image; `kernel_main` initializes metadata/services,
prints a VGA/serial status screen, and halts with `hlt`. The Phase 15 baseline,
before optimization, passed `./tools/test.sh`, the bare-metal target check, and
`./tools/build.sh`.

| Area | Current implementation and classification |
|---|---|
| Boot and diagnostics | Implemented and build-verified. Startup prints memory-region count and explicit unavailable-service states; no kernel timer exists to measure boot phases. |
| Memory management | Missing. `hal::memory` validates MMIO ranges only; there is no page allocator, heap allocator, virtual memory manager, or memory accounting. |
| Scheduling and processes | Missing. No scheduler, task/thread model, context switching, process execution, or context-switch counter exists. |
| Timer and interrupts | Missing. Uptime reports `TimerUnavailable`; interrupt registration reports controller unavailable. |
| System calls | Experimental definitions/validation only; dispatcher returns `NotSupported`. |
| Filesystem and storage | Missing. No usable filesystem, storage driver, or filesystem latency path exists. |
| Graphics | Software framebuffer and rectangle primitives are implemented and host-testable. Hardware display setup and graphical boot are missing. |
| Window manager/compositor | Bounded software metadata/composition exists; not connected to a live display, event loop, or hardware input. |
| Input | Event types exist, but keyboard/mouse drivers and event queue are unavailable. |
| Application lifecycle | Manifest registry metadata exists; launch/termination are unavailable. |
| Drivers/hardware | Resource metadata and host-testable PCI parsing exist; active device drivers are unavailable. |
| Logging and crash handling | VGA/serial console and panic output exist. Persistent logging, structured kernel diagnostics, and recovery/restart policy are unavailable. |
| Build and tests | Nightly Rust workspace; `tools/test.sh`, bare-metal target check, bootimage build, and QEMU text-mode script are available. |
| Benchmarks | No existing benchmark suite or runtime metric API existed. A reproducible ignored host microbenchmark was added for the software framebuffer fill path. |

## Bottlenecks and implementation scope

The only measurable, exercised rendering path suitable for a safe optimization
is `Framebuffer::fill_rect`. It previously called the fully checked
`write_pixel` for every pixel, repeating coordinate checks, checked offset
arithmetic, bounds validation, and channel selection.

The implementation now validates the rectangle and row byte spans once,
selects the pixel format once, and performs only the required volatile byte
writes per pixel. The mapped-memory contract and volatile accesses are
preserved. Host tests verify channel order, stride, clipping, bounds, and that
pixels and row padding outside the target rectangle remain unchanged.

No scheduler, allocator, idle-state, input pipeline, filesystem, application
runtime, service startup graph, active display driver, or power-management
implementation exists to optimize. No optimization is claimed for those
subsystems. The framebuffer is not used by the current boot path, so this
change does not claim improved interactive desktop performance.

## Results and limitations

The same ignored debug-profile host microbenchmark was run before and after the
change in WSL2/Ubuntu 24.04 on the same workspace:

| Sample | Median time for 25 full 640x480 fills | Throughput |
|---|---:|---:|
| Before | 1,258,645 us | 24 MB/s |
| After, run 1 | 230,762 us | 133 MB/s |
| After, run 2 | 260,953 us | 117 MB/s |

The observed host-only improvement was approximately 4.9x–5.5x relative to the
single baseline series. Samples vary, and this result measures writes into a
host memory buffer through the software framebuffer API—not a physical
framebuffer, QEMU rendering, end-user latency, or GPU performance. The initial
baseline is one five-sample median, so the ratio is indicative rather than a
formal performance guarantee.

Detailed test commands and QEMU verification are recorded in
[`phase-15-performance.md`](./phase-15-performance.md).

## Recommended implementation order

1. Establish a real boot-time timer source and bounded diagnostic counters.
2. Implement and test physical-page allocation and memory accounting before
   adding heap or process metrics.
3. Implement safe interrupt/timer infrastructure, then a minimal scheduler
   with isolated host-model tests where practical.
4. Add a verified display/input path before evaluating compositor damage
   tracking or input latency.
5. Add storage and filesystem primitives before measuring file/application
   workloads.
6. Only then add system-wide `nova-top`/`nova-perf` interfaces. Until a real
   shell and metric APIs exist, the host microbenchmark remains explicitly
   scoped to one software-rendering operation.
