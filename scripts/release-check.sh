#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
export CARGO_PROFILE_DEV_DEBUG="${CARGO_PROFILE_DEV_DEBUG:-0}"
export CARGO_PROFILE_TEST_DEBUG="${CARGO_PROFILE_TEST_DEBUG:-0}"

run() {
  echo ">> $*"
  "$@"
}

plan() {
  cat <<'EOF'
format
diff
workspace-build
workspace-test
workspace-clippy
msrv
generated-model-matrix
generated-regeneration
docs-rs
cli-smoke
mcp-smoke
wasm-smoke
profile-fixtures
migration-fixtures
fuzz-regressions
fuzz-baseline
benchmark-baseline
audit
licenses
package
package-size
sbom
release-evidence
checksums
EOF
}

require_plan_row() {
  local rows="$1"
  local required="$2"
  grep -qx "$required" "$rows" || {
    echo "error: release plan is missing mandatory row: $required" >&2
    return 1
  }
}

require_clean_generation() {
  "$@" || {
    echo "error: generated output differs from a clean regeneration" >&2
    return 1
  }
}

require_supply_chain() {
  "$@" || {
    echo "error: audit or license policy failed" >&2
    return 1
  }
}

require_adapter_smoke() {
  "$@" || {
    echo "error: adapter smoke test failed" >&2
    return 1
  }
}

require_evidence() {
  local directory="$1"
  test -s "$directory/support.json" || {
    echo "error: release support evidence is missing" >&2
    return 1
  }
  test -s "$directory/SHA256SUMS" || {
    echo "error: release checksums are missing" >&2
    return 1
  }
  (cd "$directory" && shasum -a 256 -c SHA256SUMS >/dev/null) || {
    echo "error: release checksum verification failed" >&2
    return 1
  }
}

self_test() {
  local scratch
  scratch="$(mktemp -d)"
  trap 'rm -rf -- "$scratch"' RETURN

  plan > "$scratch/plan"
  for row in workspace-test generated-regeneration audit licenses cli-smoke package-size checksums; do
    require_plan_row "$scratch/plan" "$row"
  done

  grep -vx generated-model-matrix "$scratch/plan" > "$scratch/skipped"
  if require_plan_row "$scratch/skipped" generated-model-matrix 2>/dev/null; then
    echo "error: skipped matrix row was not detected" >&2
    return 1
  fi
  if require_clean_generation false 2>/dev/null; then
    echo "error: dirty generated output was not detected" >&2
    return 1
  fi
  if require_supply_chain false 2>/dev/null; then
    echo "error: audit/license failure was not detected" >&2
    return 1
  fi
  if require_adapter_smoke false 2>/dev/null; then
    echo "error: failed adapter smoke was not detected" >&2
    return 1
  fi
  mkdir -p "$scratch/evidence"
  if require_evidence "$scratch/evidence" 2>/dev/null; then
    echo "error: missing checksum/evidence was not detected" >&2
    return 1
  fi

  printf '{"release_state":"unpublished-candidate"}\n' > "$scratch/evidence/support.json"
  (cd "$scratch/evidence" && shasum -a 256 support.json > SHA256SUMS)
  require_evidence "$scratch/evidence"
  echo ">> release-check failure injection tests passed"
}

core_checks() {
  run cargo +stable fmt --all --check
  run git diff --check
  run cargo +stable build --locked --workspace --no-default-features --jobs 1
  run cargo +stable test --locked --workspace --no-default-features --jobs 1
  run cargo +stable clippy --locked --workspace --all-targets --no-default-features --jobs 1 -- -D warnings
  run cargo +1.85 check --locked --workspace --exclude rust_iso20022_mcp --no-default-features --jobs 1
}

case "${1:-}" in
  --plan)
    plan
    exit 0
    ;;
  --self-test)
    self_test
    exit 0
    ;;
  --ci-core)
    core_checks
    run env RUSTDOCFLAGS="--cfg docsrs -D rustdoc::broken_intra_doc_links" \
      cargo +stable doc --locked --no-deps --features model-head,model-pacs,model-pain,serde,convert --jobs 1
    run cargo +stable test --locked -p rust_iso20022_cli --jobs 1
    run cargo +stable test --locked -p rust_iso20022_mcp --jobs 1
    exit 0
    ;;
  "") ;;
  *) echo "usage: scripts/release-check.sh [--plan|--self-test|--ci-core]" >&2; exit 2 ;;
esac

core_checks
run scripts/check-model-matrix.sh
require_clean_generation cargo +stable run --locked -p rust_iso20022_codegen -- --check
run env RUSTDOCFLAGS="--cfg docsrs -D rustdoc::broken_intra_doc_links" \
  cargo +stable doc --locked --no-deps --features model-head,model-pacs,model-pain,serde,convert --jobs 1

require_adapter_smoke cargo +stable test --locked -p rust_iso20022_cli --jobs 1
require_adapter_smoke cargo +stable test --locked -p rust_iso20022_mcp --jobs 1
require_adapter_smoke scripts/check-wasm-sdk.sh
run cargo +stable test --locked -p rust_iso20022 --features profiles \
  --test profile_framework --test profile_provenance --test sepa_framework --jobs 1
run cargo +stable test --locked -p rust_iso20022_migration \
  --features "mt103 mt202 mt940" --jobs 1
run scripts/run-fuzz-regressions.sh
run scripts/run-fuzz-baseline.sh
run scripts/measure-build-baseline.sh evidence/performance/release-candidate.json

require_supply_chain cargo +stable audit
require_supply_chain cargo +stable deny --locked --all-features check

run cargo +stable package --list --allow-dirty
run cargo +stable package --locked --allow-dirty
run scripts/check-package-size.sh target/package/rust_iso20022-*.crate
run scripts/generate-release-baseline.sh evidence/release-baseline
run scripts/generate-sbom.sh evidence/release-baseline/sbom
require_evidence evidence/release-baseline

echo ">> unpublished release candidate passed every mandatory gate"
