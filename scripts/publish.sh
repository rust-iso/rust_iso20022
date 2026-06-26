#!/usr/bin/env bash
#
# Release rust_iso20022 to crates.io.
#
# Steps:
#   1. sanity checks (required tools, clean working tree)
#   2. regenerate the model + catalogue from xsds/ (codegen)
#   3. run the test suite (default features + the typed model)
#   4. publish to crates.io
#
# Pass --dry-run to exercise cargo in dry-run mode without publishing.
#
# Usage:
#   scripts/publish.sh [--dry-run]
#
set -euo pipefail

cd "$(dirname "$0")/.."

DRY_RUN=0
if [ "${1:-}" = "--dry-run" ]; then
    DRY_RUN=1
fi

VERSION=$(grep -m1 '^version' Cargo.toml | sed -E 's/.*"([^"]+)".*/\1/')
echo ">> Releasing rust_iso20022 v${VERSION} (dry-run=${DRY_RUN})"

# 1. sanity checks
for tool in cargo git; do
    command -v "$tool" >/dev/null 2>&1 || { echo "error: '$tool' not found in PATH" >&2; exit 1; }
done

if [ -n "$(git status --porcelain 2>/dev/null || true)" ]; then
    echo "error: working tree is dirty; commit or stash changes before publishing." >&2
    exit 1
fi

# 2. regenerate the model + catalogue (requires xsds/)
if [ -d xsds ]; then
    echo ">> Regenerating model + catalogue from xsds/"
    cargo run -p rust_iso20022_codegen -- --input xsds --output src/generated
fi

# 3. tests: always-available core + the typed model
echo ">> Testing core + catalogue"
cargo test
echo ">> Testing the generated model (this is slow: ~495 modules)"
cargo test --features model

# 4. crates.io
if [ "$DRY_RUN" = 1 ]; then
    cargo publish --dry-run
else
    cargo publish
fi

echo ">> Done: rust_iso20022 v${VERSION}"
