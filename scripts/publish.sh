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

# The model is tested one business area at a time. Compiling all ~1130 modules
# into a single test binary needs far more memory than most machines have (the
# linker step is OOM-killed); per-area builds give identical coverage and finish.
MODEL_AREAS="acmt admi auth caaa caad caam cafc cafm cafr cain camt canm casp \
casr catm catp colr fxtr head pacs pain reda remt secl seev semt sese setr \
trck tsin tsmt tsrv"
echo ">> Testing the generated model per area (slow: 32 areas, ~1130 modules total)"
for area in $MODEL_AREAS; do
    echo ">> model-$area"
    cargo test --features "model-$area"
done

# 4. crates.io
if [ "$DRY_RUN" = 1 ]; then
    cargo publish --dry-run
else
    cargo publish
fi

echo ">> Done: rust_iso20022 v${VERSION}"
