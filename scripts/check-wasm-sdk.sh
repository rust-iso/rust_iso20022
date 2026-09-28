#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

command -v wasm-pack >/dev/null 2>&1 || {
    echo "error: wasm-pack is required" >&2
    exit 1
}

wasm_tmp_dir="$(mktemp -d)"
cleanup() {
    if [[ -n "${wasm_tmp_dir:-}" && -d "$wasm_tmp_dir" ]]; then
        rm -rf "$wasm_tmp_dir"
    fi
}
trap cleanup EXIT

wasm-pack build crates/wasm \
    --target nodejs \
    --release \
    --out-dir "$wasm_tmp_dir/pkg" \
    --out-name rust_iso20022_wasm \
    --features model-pacs

node crates/wasm/tests/security.mjs "$wasm_tmp_dir/pkg/rust_iso20022_wasm.js"

wasm-pack build crates/wasm \
    --target web \
    --release \
    --out-dir "$wasm_tmp_dir/web" \
    --out-name rust_iso20022_wasm \
    --features model-pacs

test -s "$wasm_tmp_dir/web/rust_iso20022_wasm.js"
test -s "$wasm_tmp_dir/web/rust_iso20022_wasm_bg.wasm"

node crates/wasm/tests/browser.mjs "$wasm_tmp_dir/web"

wasm_bytes="$(wc -c < "$wasm_tmp_dir/web/rust_iso20022_wasm_bg.wasm" | tr -d ' ')"
echo "verified node and browser runtimes; optimized wasm bytes: $wasm_bytes"
