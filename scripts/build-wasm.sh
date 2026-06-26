#!/usr/bin/env bash
#
# Build the WebAssembly / npm package with wasm-pack.
#
# The `--cfg direct_wasm` flag activates the `#[wasm_bindgen]` bindings in
# `src/wasm.rs` (gated behind `all(direct_wasm, target_arch = "wasm32")`),
# producing the JS bindings and the .wasm artifact in ./pkg. Only the
# identification / catalogue / metadata layer is exposed (not the message model).
#
# Usage:
#   scripts/build-wasm.sh [extra wasm-pack args]
#
set -euo pipefail

cd "$(dirname "$0")/.."

if ! command -v wasm-pack >/dev/null 2>&1; then
    echo "error: wasm-pack not found. Install it with: cargo install wasm-pack" >&2
    exit 1
fi

RUSTFLAGS="--cfg direct_wasm" wasm-pack build --target web --release "$@"
