# Terminal

**Status: Blocked by missing dependencies.** Nova has no graphical terminal
window, command interpreter, process execution API, or process termination
service. SDK process calls return `NotSupported`.

The project does not claim support for `help`, `ls`, `cat`, POSIX shell syntax,
or other shell commands as Nova commands. A host terminal used for building
the project is not a Project Nova application.

**Acceptance criterion:** not met. Implement only after a documented command
interpreter, non-blocking process API, terminal I/O, and supported termination
behavior exist.
