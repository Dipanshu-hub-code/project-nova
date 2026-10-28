# Nova Software Center

The project now includes two non-installing previews:

- `preview/software-center.html` is a responsive host-side design preview with
  local search and category filtering. All app names and ratings are sample
  content, and install buttons are disabled.
- `preview/nova-desktop.html` is an interactive desktop-shell prototype with a
  launcher, dock, mock application windows, notification panel, and mock
  control center.
- The kernel draws an ASCII catalog screen into the BIOS VGA text buffer when
  it boots in QEMU. It is a static screen, not a graphical desktop, and the
  kernel has no keyboard or mouse driver for interaction.

The graphical software center is not yet a running Nova application. The
current BIOS boot path does not provide a mapped pixel framebuffer; the
framebuffer drawing primitives are not connected to a display driver. The
project also lacks package downloads, signature verification, archive
extraction, persistence, and application process execution. Thus the current
UI previews cannot install or launch apps.

**Phase 14 status: blocked by missing dependencies.** There is no repository
transport or usable installer transaction. The in-memory package metadata
foundation is not an authoritative persisted package database, and the sample
preview catalog must not be interpreted as installable software. Installation,
updates, removal, progress reporting, and transaction history are not
implemented.

To inspect the host preview, open `preview/software-center.html` in a browser.
To explore the wider desktop mockup, open `preview/nova-desktop.html`.
To view the kernel's text screen, run `./tools/build.sh` followed by
`./tools/run_qemu.sh` with a graphical QEMU display.

The desktop's visual direction uses common modern desktop patterns (a dock,
launcher, translucent surfaces, and compact system panels) with Nova's own
branding and CSS-drawn wallpaper. It does not use Deepin's name, logos, or
proprietary artwork.
