# Phase 10 Testing

Run the kernel library's host-side unit tests with:

```sh
./tools/test.sh
```

They exercise manifest parsing and path validation, bounded database behavior,
dependency resolution, repository configuration, and the explicit installer
failure boundary.

QEMU package installation tests are not currently applicable: the kernel has
no package filesystem, network client, signature verifier, or application
launcher.
