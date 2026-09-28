#!/usr/bin/env bash
#
# Run the expensive generated-model release matrix with bounded concurrency.
# This is intentionally separate from per-WP gates: each WP runs only its
# relevant feature suite, while release verification covers every key family.
set -euo pipefail

cd "$(dirname "$0")/.."

export CARGO_PROFILE_DEV_DEBUG="${CARGO_PROFILE_DEV_DEBUG:-0}"
export CARGO_PROFILE_TEST_DEBUG="${CARGO_PROFILE_TEST_DEBUG:-0}"

run() {
    echo ">> $*"
    "$@"
}

run_family() {
    local features="$1"
    run cargo +stable test -p rust_iso20022 --no-default-features \
        --features "$features" --jobs 1
    run cargo +stable test -p rust_iso20022 --doc --no-default-features \
        --features "$features" --jobs 1
}

run_family model-head,serde
run_family model-pacs,serde,convert
run_family model-pain,serde
run_family model-camt,serde

echo ">> Generated-model release matrix passed"
