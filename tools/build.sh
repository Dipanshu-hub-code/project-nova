#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ -f "$HOME/.cargo/env" ]]; then
    # Load rustup's PATH setup for non-login shells such as VS Code tasks.
    . "$HOME/.cargo/env"
fi
cd "$REPO_ROOT"
cargo bootimage -p nova-kernel --target x86_64-unknown-none
