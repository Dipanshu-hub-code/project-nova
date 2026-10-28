#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ -f "$HOME/.cargo/env" ]]; then
    . "$HOME/.cargo/env"
fi
cd "$REPO_ROOT"
cargo test --workspace --lib
cargo test -p nova-dev
cargo check --workspace
