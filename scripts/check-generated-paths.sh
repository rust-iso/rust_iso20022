#!/usr/bin/env bash
#
# Compile one representative generated path for every business-area feature.
# Areas are checked separately because linking all 1,130 models together exceeds
# typical CI and docs.rs memory limits.
set -euo pipefail

cd "$(dirname "$0")/.."

generated_areas=(
    acmt admi auth caaa caad caam cafc cafm cafr cain camt canm casp casr
    catm catp colr fxtr head pacs pain reda remt secl seev semt sese setr
    trck tsin tsmt tsrv
)

if (( $# > 0 )); then
    generated_areas=("$@")
fi

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PWD/target/compat-generated-paths}"
manifest="tests/compatibility/compile/generated_paths/Cargo.toml"

for area in "${generated_areas[@]}"; do
    echo ">> checking public generated path for model-$area"
    cargo +stable check --locked \
        --manifest-path "$manifest" \
        --no-default-features \
        --features "model-$area"
done

echo ">> verified ${#generated_areas[@]} generated business-area paths"
