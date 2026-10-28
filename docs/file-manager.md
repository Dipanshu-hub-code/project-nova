# File Manager

**Status: Blocked by a missing dependency.** There is no usable Nova
filesystem, storage driver, permission-enforced file API, application launcher,
or native window/input path. SDK filesystem calls return `NotSupported`.

No file listing, create/copy/move/rename/delete, metadata, mounted-volume,
search, or open-file operation is implemented as a Nova application. The
browser desktop preview does not access or represent Nova files. Adding a
mock directory listing would misrepresent the system, so it is intentionally
not included.

**Acceptance criterion:** not met. Implement after a permission-checked
filesystem and application-opening service exist. Tests for real Nova
operations are blocked until then.
