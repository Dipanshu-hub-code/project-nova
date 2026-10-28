# Desktop Integration and Recovery

**Status: Blocked by missing dependencies.** The current boot path does not
run a graphical shell or application processes. There is no native window
lifecycle, authenticated application identity, IPC, or session manager to
deliver clipboard data, application notifications, or crash cleanup.

The kernel has a bounded notification queue, but user apps cannot submit
verified notifications through it. The browser preview's notifications and
window controls are simulated. No claim is made for native crash recovery,
resource cleanup, or unsaved-document protection.

The acceptance criterion is not met. Implement after a real process manager,
window lifecycle, and authenticated service channel exist.
