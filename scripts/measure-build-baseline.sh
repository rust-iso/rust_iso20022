#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

output="${1:-evidence/performance/baseline.json}"
iterations="${ISO20022_BENCH_ITERATIONS:-1000}"
jobs="${CARGO_BUILD_JOBS:-1}"
selected_features="model-pacs"
default_target="$(mktemp -d /tmp/rust_iso20022-default-build.XXXXXX)"
selected_target="$(mktemp -d /tmp/rust_iso20022-selected-build.XXXXXX)"
artifact_target="$(mktemp -d /tmp/rust_iso20022-artifacts.XXXXXX)"
scratch="$(mktemp -d /tmp/rust_iso20022-measure.XXXXXX)"

cleanup() {
  rm -rf -- "$default_target" "$selected_target" "$artifact_target" "$scratch"
}
trap cleanup EXIT

mkdir -p "$(dirname "$output")"

measure_build() {
  local target_dir="$1"
  local feature_args="$2"
  local timing_file="$3"
  if [[ -n "$feature_args" ]]; then
    CARGO_TARGET_DIR="$target_dir" CARGO_PROFILE_DEV_DEBUG=0 \
      /usr/bin/time -p -o "$timing_file" cargo +stable build -p rust_iso20022 \
      --lib --no-default-features --features "$feature_args" --jobs "$jobs" >/dev/null
  else
    CARGO_TARGET_DIR="$target_dir" CARGO_PROFILE_DEV_DEBUG=0 \
      /usr/bin/time -p -o "$timing_file" cargo +stable build -p rust_iso20022 \
      --lib --no-default-features --jobs "$jobs" >/dev/null
  fi
  awk '$1 == "real" { print $2 }' "$timing_file"
}

default_seconds="$(measure_build "$default_target" "" "$scratch/default.time")"
selected_seconds="$(measure_build "$selected_target" "$selected_features" "$scratch/selected.time")"

default_dependencies="$(cargo +stable tree -p rust_iso20022 --no-default-features --edges normal --prefix none | sort -u | wc -l | tr -d ' ')"
selected_dependencies="$(cargo +stable tree -p rust_iso20022 --no-default-features --features "$selected_features" --edges normal --prefix none | sort -u | wc -l | tr -d ' ')"

CARGO_TARGET_DIR="$artifact_target" CARGO_PROFILE_RELEASE_DEBUG=0 \
  cargo +stable build -p rust_iso20022_cli --release --jobs "$jobs" >/dev/null
cli_path="$artifact_target/release/iso20022"
cli_bytes="$(stat -f '%z' "$cli_path" 2>/dev/null || stat -c '%s' "$cli_path")"

wasm_bytes=null
if rustup target list --installed | grep -qx 'wasm32-unknown-unknown'; then
  CARGO_TARGET_DIR="$artifact_target" CARGO_PROFILE_RELEASE_DEBUG=0 \
    cargo +stable build -p rust_iso20022_wasm --release \
    --target wasm32-unknown-unknown --no-default-features --jobs "$jobs" >/dev/null
  wasm_path="$artifact_target/wasm32-unknown-unknown/release/rust_iso20022_wasm.wasm"
  wasm_bytes="$(stat -f '%z' "$wasm_path" 2>/dev/null || stat -c '%s' "$wasm_path")"
fi

benchmark_target="${ISO20022_BENCH_TARGET_DIR:-/tmp/rust_iso20022-performance}"
CARGO_TARGET_DIR="$benchmark_target" CARGO_PROFILE_BENCH_DEBUG=0 \
  ISO20022_BENCH_ITERATIONS="$iterations" \
  cargo +stable bench -p rust_iso20022_benchmarks --bench sdk --jobs "$jobs" \
  >"$scratch/runtime.jsonl"
CARGO_TARGET_DIR="$benchmark_target" CARGO_PROFILE_RELEASE_DEBUG=0 \
  cargo +stable build -p rust_iso20022_benchmarks --release \
  --bin memory --bin allocations --jobs "$jobs" \
  >/dev/null
: >"$scratch/memory.jsonl"
: >"$scratch/allocations.jsonl"
for workload in pacs008_parse pacs008_serialize pacs008_detect pacs008_validate camt053_parse catalogue_lookup; do
  ISO20022_BENCH_ITERATIONS="${ISO20022_MEMORY_ITERATIONS:-100}" \
    "$benchmark_target/release/memory" "$workload" >>"$scratch/memory.jsonl"
  ISO20022_BENCH_ITERATIONS="${ISO20022_ALLOCATION_ITERATIONS:-100}" \
    "$benchmark_target/release/allocations" "$workload" >>"$scratch/allocations.jsonl"
done

jq -s '.' "$scratch/runtime.jsonl" >"$scratch/runtime.json"
jq -s '.' "$scratch/memory.jsonl" >"$scratch/memory.json"
jq -s '.' "$scratch/allocations.jsonl" >"$scratch/allocations.json"

commit="$(git rev-parse HEAD 2>/dev/null || printf unknown)"
tree_state=clean
if [[ -n "$(git status --porcelain)" ]]; then
  tree_state=dirty
fi
rustc_version="$(rustc +stable --version)"
cargo_version="$(cargo +stable --version)"
target_triple="$(rustc +stable -vV | awk '/^host:/ { print $2 }')"
os="$(uname -srvmo 2>/dev/null || uname -a)"
cpu="$(sysctl -n machdep.cpu.brand_string 2>/dev/null || system_profiler SPHardwareDataType 2>/dev/null | awk -F': ' '/Chip:/ { print $2; exit }' || awk -F: '/model name/ { sub(/^ /, "", $2); print $2; exit }' /proc/cpuinfo 2>/dev/null || printf unknown)"
logical_cpus="$(getconf _NPROCESSORS_ONLN 2>/dev/null || sysctl -n hw.logicalcpu 2>/dev/null || printf unknown)"
pacs_sha="$(shasum -a 256 fixtures/iso/valid/pacs.008.001.08-cross-field.xml | awk '{print $1}')"
camt_sha="$(shasum -a 256 fixtures/migration/mt940/expected.xml | awk '{print $1}')"
measured_at="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"

jq -n \
  --arg schema "rust_iso20022.performance-baseline.v1" \
  --arg measured_at "$measured_at" \
  --arg commit "$commit" \
  --arg tree_state "$tree_state" \
  --arg rustc "$rustc_version" \
  --arg cargo "$cargo_version" \
  --arg target "$target_triple" \
  --arg os "$os" \
  --arg cpu "$cpu" \
  --arg logical_cpus "$logical_cpus" \
  --arg pacs_sha "$pacs_sha" \
  --arg camt_sha "$camt_sha" \
  --argjson iterations "$iterations" \
  --argjson default_seconds "$default_seconds" \
  --argjson selected_seconds "$selected_seconds" \
  --arg selected_features "$selected_features" \
  --argjson default_dependencies "$default_dependencies" \
  --argjson selected_dependencies "$selected_dependencies" \
  --argjson cli_bytes "$cli_bytes" \
  --argjson wasm_bytes "$wasm_bytes" \
  --slurpfile runtime "$scratch/runtime.json" \
  --slurpfile memory "$scratch/memory.json" \
  --slurpfile allocations "$scratch/allocations.json" \
  '{
    schema: $schema,
    measured_at: $measured_at,
    runner: {
      commit: $commit,
      tree_state: $tree_state,
      rustc: $rustc,
      cargo: $cargo,
      target: $target,
      os: $os,
      cpu: $cpu,
      logical_cpus: $logical_cpus,
      jobs: 1,
      debug_info: 0
    },
    fixtures: {
      pacs008_sha256: $pacs_sha,
      camt053_sha256: $camt_sha
    },
    runtime: {
      iterations: $iterations,
      observations: $runtime[0]
    },
    memory: {
      observations: $memory[0]
    },
    allocations: {
      observations: $allocations[0]
    },
    build: {
      cold_default_seconds: $default_seconds,
      cold_selected_seconds: $selected_seconds,
      selected_features: $selected_features,
      normal_dependency_nodes_default: $default_dependencies,
      normal_dependency_nodes_selected: $selected_dependencies,
      cli_release_bytes: $cli_bytes,
      wasm_release_bytes: $wasm_bytes,
      wasm_features: "default (detection/catalogue only)"
    },
    limitations: [
      "single-run baseline; repeat three times before regression conclusions",
      "peak RSS is process-wide high-water memory, not per-operation allocation",
      "working tree state is recorded and must match for comparisons"
    ]
  }' >"$scratch/baseline.json"

mv "$scratch/baseline.json" "$output"
echo "wrote $output"
