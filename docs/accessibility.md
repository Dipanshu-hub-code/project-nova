# Accessibility

**Status: Partially implemented in host-side SDK logic; native desktop
accessibility is blocked.** `nova-ui` exposes bounded, deterministic focus
registration and forward/reverse focus traversal, keyboard activation state
for buttons, and accessible text-field labels. These behaviors have unit
tests.

There is no rendered focus indicator, OS keyboard event dispatch, font
scaling, high-contrast setting, reduced-motion service, screen-reader API, or
running desktop. Do not claim screen-reader support or complete keyboard
usability. The acceptance criterion for native apps is not met.
