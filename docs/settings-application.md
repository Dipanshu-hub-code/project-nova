# Settings Application

**Status: Blocked by missing dependencies.** The kernel has a small in-memory
Light/Dark theme setting. It is not persisted, not exposed to Nova user
applications, and not wired to an active desktop. Other requested display,
dock, input, user, network, audio, update, and permission settings are not
available as operating-system services.

`nova-ui` contains light/dark design tokens, but selecting a token does not
change the kernel desktop theme. The settings acceptance criterion is not met.
Do not present unsupported controls as working.
