# Text Editor

**Status: Blocked by missing dependencies.** The Nova filesystem and native
window/input APIs are unavailable, so no native editor can open, save, or
preserve real Nova documents.

The shared `nova-ui::TextField` is a tested single-line input state primitive,
not a multi-line editor. It validates UTF-8 boundaries and caps input length;
it does not implement document buffers, line endings, undo/redo, save
transactions, or unsaved-change prompts.

**Acceptance criterion:** not met. A native editor must wait for file APIs and
must preserve original content on save failure and warn before discarding
unsaved data.
