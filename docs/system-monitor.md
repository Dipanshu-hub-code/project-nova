# System Monitor

**Status: Blocked by missing dependencies.** There is no process enumeration
service or supported system-metrics API. The kernel does not expose CPU,
memory, storage, network, or per-application measurements to user applications.

No fabricated measurements or process listings are shown. The acceptance
criterion is not met. Implement after metrics APIs exist, with explicit
unavailable states and authorization-limited process controls.
