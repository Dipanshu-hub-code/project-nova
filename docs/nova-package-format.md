# Nova Package Format

The intended archive extension is `.npkg`, with a `manifest.toml` at its root
and application payload under `usr/`. Archive reading and extraction are not
implemented.

## Supported manifest subset

The current parser accepts only these sections and keys. Strings must be
unescaped basic quoted strings. Dependency requirements support package IDs
with an optional `>=` numeric dotted-version minimum.

```toml
[package]
id = "com.example.terminal"
name = "Terminal"
version = "1.0.0"
architecture = "x86_64"

[exec]
path = "/usr/bin/terminal"

[desktop]
icon = "/usr/share/icons/terminal.png"

[dependencies]
requires = ["nova-libs >= 1.0.0"]
```

`[desktop]` and `[dependencies]` are optional. The required fields are the
package ID, name, version, architecture, and executable path. The parser
currently rejects keys outside this subset (including descriptive metadata
and file lists).

Executable and icon paths must be canonical paths beneath `/usr/`; empty,
`.` and `..` path components and backslashes are rejected. The separate
`validate_archive_path` helper accepts only relative archive paths and rejects
absolute paths, traversal, empty components, backslashes, and drive prefixes.
Only `x86_64` is supported.
