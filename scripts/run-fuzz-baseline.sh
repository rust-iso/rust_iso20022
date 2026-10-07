#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

toolchain="${FUZZ_TOOLCHAIN:-nightly-2026-09-26}"
seconds="${FUZZ_SECONDS:-60}"
timeout="${FUZZ_TIMEOUT_SECONDS:-5}"
rss_mb="${FUZZ_RSS_LIMIT_MB:-2048}"
max_len="${FUZZ_MAX_INPUT_BYTES:-65536}"
codegen_units="${FUZZ_CODEGEN_UNITS:-16}"
mode="${1:-run}"
targets=(xml detect namespace financial mt_parser mt103 mt202 mt940)
artifact_dir="$repo_root/fuzz/artifacts"
mkdir -p "$artifact_dir"

# cargo-fuzz otherwise forces one LLVM unit for the entire generated-model crate.
# Split optimization work while retaining sanitizers, assertions and all targets.
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
export CARGO_INCREMENTAL=0
if [[ ! "$codegen_units" =~ ^[1-9][0-9]*$ ]]; then
  echo "FUZZ_CODEGEN_UNITS must be a positive integer" >&2
  exit 2
fi

if ! fuzz_version="$(cargo "+$toolchain" fuzz --version 2>/dev/null)" || [[ "$fuzz_version" != "cargo-fuzz 0.13.2" ]]; then
  echo "cargo-fuzz 0.13.2 is required; install it with: cargo +$toolchain install cargo-fuzz --version 0.13.2 --locked" >&2
  exit 2
fi

export CARGO_NET_OFFLINE="${CARGO_NET_OFFLINE:-false}"
cargo "+$toolchain" metadata --locked --manifest-path fuzz/Cargo.toml --format-version 1 >/dev/null
# cargo-fuzz has no --locked option; use the prefetched, validated lockfile offline.
export CARGO_NET_OFFLINE=true

if [[ "$mode" == "--minimize" ]]; then
  for target in "${targets[@]}"; do
    cargo "+$toolchain" fuzz cmin "$target" "fuzz/corpus/$target" --codegen-units "$codegen_units" -- \
      -max_len="$max_len"
  done
  exit 0
fi

if [[ "$mode" != "run" ]]; then
  echo "usage: scripts/run-fuzz-baseline.sh [run|--minimize]" >&2
  exit 2
fi

for target in "${targets[@]}"; do
  log="$artifact_dir/$target.log"
  cargo "+$toolchain" fuzz run "$target" "fuzz/corpus/$target" --codegen-units "$codegen_units" -- \
    -max_total_time="$seconds" \
    -timeout="$timeout" \
    -rss_limit_mb="$rss_mb" \
    -max_len="$max_len" \
    -artifact_prefix="$artifact_dir/$target-" 2>&1 | tee "$log"
done
