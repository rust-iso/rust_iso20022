#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

export CARGO_PROFILE_DEV_DEBUG="${CARGO_PROFILE_DEV_DEBUG:-0}"
export CARGO_PROFILE_TEST_DEBUG="${CARGO_PROFILE_TEST_DEBUG:-0}"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"

cargo +stable test --locked -p rust_iso20022 --test fuzz_corpus --no-default-features
cargo +stable test --locked -p rust_iso20022_migration --test fuzz_corpus \
  --features "mt103 mt202 mt940"
