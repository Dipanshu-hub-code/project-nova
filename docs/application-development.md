# Application Development

Application manifests can currently be parsed and registered, but application
binaries cannot be launched: the kernel has no verified executable loader,
user-mode transition, or process-isolation model.

Example manifest:

```toml
id = "terminal"
name = "Terminal"
version = "0.1.0"
exec = "/apps/terminal.bin"
icon = "/icons/terminal.png"
caps = ["fs.read", "graphics.window"]
```

Parsing checks required fields, field sizes, duplicate fields, supported
capabilities, and unsafe path components. The manifest format is a deliberately
small key/value subset, not a complete TOML implementation. Manifest paths are
metadata only; no filesystem lookup is performed.
