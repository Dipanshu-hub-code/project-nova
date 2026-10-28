# Application Packaging

The existing `.npkg` manifest format is not yet connected to an application
build or package-creation tool. `nova-dev package` intentionally returns an
error. Package archives, signing, installation, and launching are unavailable.

Do not treat a host binary or an unsigned archive as an installable Nova
application.
