# Application Execution Model

The intended application lifecycle is discovery, validation, process
creation, window creation, event delivery, execution, termination, and cleanup.
The current kernel implements only application metadata registration.

ELF x86_64 is a candidate format, not a supported executable format. There is
no executable loader, user-mode process, stable syscall entry, address-space
isolation, filesystem service, or window creation API. The GUI template is
therefore a host-buildable demonstration and cannot launch as a Nova app.
