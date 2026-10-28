# Phase 14 Testing Record

## Baseline before Phase 14 edits

- `./tools/test.sh`: passed before edits (32 kernel library tests, 2
  `nova-std` tests, 2 `nova-ui` tests, 1 `nova-dev` test, and workspace
  checking).
- `cargo check -p nova-kernel --target x86_64-unknown-none`: passed.
- `./tools/build.sh`: passed and produced the kernel boot image.
- QEMU graphical/application integration: not run; the current boot path is
  text-mode and has no native application runtime.

## After Phase 14 changes

- `cargo fmt --all -- --check`: passed.
- `cargo test -p nova-ui --lib`: passed, 8 tests.
- `./tools/test.sh`: passed, 43 tests total (32 kernel, 2 `nova-std`,
  8 `nova-ui`, 1 `nova-dev`), workspace check passed.
- `cargo check -p nova-kernel --target x86_64-unknown-none`: passed.
- `./tools/build.sh`: passed and produced the boot image.
- QEMU text-mode boot: verified using `./tools/run_qemu.sh -display none`;
  expected kernel diagnostics and the explicit unavailable-service screen were
  printed. QEMU was stopped after 8 seconds with the expected timeout status.
  Graphical/application integration was not verified; this text-mode boot is
  not evidence of a graphical desktop or native applications.

The `nova-ui` library tests cover:

- Shared Light/Dark palette distinction and design-token values.
- Focus order, wrapping, removal, empty state, and duplicate registration.
- Button mouse-click eligibility, keyboard activation, disabled state, and
  focus behavior.
- UTF-8 text insertion, cursor movement, deletion, label validation, and
  bounded input behavior.
- Layout padding calculations and dimension underflow.
- Existing explicit `NotSupported` behavior for window display and UI init.

Run `cargo test -p nova-ui --lib` from the repository root. Full-workspace and
bare-metal regression checks are recorded in the completion response after
the Phase 14 changes.

## Blocked test plans

File operations, process execution/termination, editor file persistence,
system metrics, settings persistence, package transactions, networking,
clipboard transfer, native keyboard navigation, notifications, and crash
cleanup cannot currently be integration-tested because their corresponding
runtime services do not exist. No destructive operation or simulated success
is used as a substitute.
