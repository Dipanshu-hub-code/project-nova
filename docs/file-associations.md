# File Associations

**Status: Blocked by missing dependencies.** Nova has no usable filesystem,
installed-application runtime, file-type registry, or application-launch
service. Selecting a file cannot safely launch an application, and there is no
default-application setting.

The acceptance criterion is not met. Executable files must remain distinct
from ordinary documents and must never run merely on selection. Implement
associations only with path/permission validation and the application manager.
