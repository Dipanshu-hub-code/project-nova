# Package Security

## Current status

Package installation and repository fetching are unavailable. The project has
no package signature verifier, digest implementation, trusted key store,
network client, or archive extractor. Configuring an HTTPS repository does not
fetch data or establish trust.

The installer returns an explicit unavailable error and does not modify the
package database or filesystem. Do not enable unsigned installation as a
fallback.

## Requirements before enabling installation

- Verify repository metadata and package signatures against explicit trust
  anchors before installation.
- Verify package digests and validate every archive entry before writing.
- Reject traversal, absolute paths outside the install root, duplicate file
  ownership, and unsafe links.
- Keep private signing keys out of the repository and separate development
  trust from production trust.
- Define key rotation and revocation behavior.
