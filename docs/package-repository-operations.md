# Repository Operations

`RepositorySystem` stores up to eight named repository URLs in caller-owned
memory. It accepts `https://` URLs and `file://` URLs for configuration. It
does not parse an index, access the filesystem, download packages, or validate
TLS certificates or signatures. The scheme check is not a security or
authenticity check.

The planned index format is JSON or TOML and package archives use `.npkg`.
Network-backed `refresh`, package search, and CLI commands such as
`nova-pkg repositories` are not implemented. Local repositories are only a
future development option until filesystem access is available.
