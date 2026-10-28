#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ -f "$HOME/.cargo/env" ]]; then
    . "$HOME/.cargo/env"
fi
IMAGE="$REPO_ROOT/target/x86_64-unknown-none/debug/bootimage-nova-kernel.bin"

if [[ ! -f "$IMAGE" ]]; then
    echo "[error] No bootable image found. Run tools/build.sh first." >&2
    exit 1
fi

exec qemu-system-x86_64 \
    -machine pc \
    -m 512M \
    -drive "format=raw,file=$IMAGE" \
    -vga std \
    -serial stdio \
    -boot order=c \
    -no-reboot \
    "$@"
