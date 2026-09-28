#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

toolchain="${FUZZ_TOOLCHAIN:-nightly-2026-09-26}"
seconds="${FUZZ_SECONDS:-60}"
timeout="${FUZZ_TIMEOUT_SECONDS:-5}"
rss_mb="${FUZZ_RSS_LIMIT_MB:-2048}"
max_len="${FUZZ_MAX_INPUT_BYTES:-65536}"
mode="${1:-run}"
targets=(xml detect namespace financial mt_parser mt103 mt202 mt940)
artifact_dir="$repo_root/fuzz/artifacts"
mkdir -p "$artifact_dir"

if ! cargo "+$toolchain" fuzz --version >/dev/null 2>&1; then
  echo "cargo-fuzz 0.13.2 is required; install it with: cargo install cargo-fuzz --version 0.13.2 --locked" >&2
  exit 2
fi

if [[ "$mode" == "--minimize" ]]; then
  for target in "${targets[@]}"; do
    cargo "+$toolchain" fuzz cmin "$target" "fuzz/corpus/$target" -- \
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
  cargo "+$toolchain" fuzz run "$target" "fuzz/corpus/$target" -- \
    -max_total_time="$seconds" \
    -timeout="$timeout" \
    -rss_limit_mb="$rss_mb" \
    -max_len="$max_len" \
    -artifact_prefix="$artifact_dir/$target-" 2>&1 | tee "$log"
done
