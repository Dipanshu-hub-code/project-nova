# Clipboard Service

**Status: Blocked by missing dependencies.** There is no Nova IPC or
cross-application clipboard service. The UI text-field state is private to its
owner and does not provide copy/paste between applications.

The acceptance criterion is not met. A clipboard implementation requires
authenticated ownership, bounded text transfer, lifecycle cleanup, and
access control over a real application communication channel.
