#!/usr/bin/env bash
#
# Verify the legacy WASM export contract. Pass --build to compile a fresh
# node-target package in a temporary directory and execute smoke cases.
set -euo pipefail

cd "$(dirname "$0")/.."

node scripts/check-wasm-compat.mjs declarations pkg/rust_iso20022.d.ts

if [[ "${1:-}" != "--build" ]]; then
    echo ">> declaration inventory passed; use --build for runtime smoke"
    exit 0
fi

command -v wasm-pack >/dev/null 2>&1 || {
    echo "error: wasm-pack is required for --build" >&2
    exit 1
}

compat_tmp_dir="$(mktemp -d)"
cleanup() {
    if [[ -n "${compat_tmp_dir:-}" && -d "$compat_tmp_dir" ]]; then
        rm -rf "$compat_tmp_dir"
    fi
}
trap cleanup EXIT

RUSTFLAGS="--cfg direct_wasm" wasm-pack build \
    --target nodejs \
    --release \
    --features=serde \
    --out-dir "$compat_tmp_dir/pkg" \
    --out-name rust_iso20022

node scripts/check-wasm-compat.mjs \
    module \
    "$compat_tmp_dir/pkg/rust_iso20022.js"
