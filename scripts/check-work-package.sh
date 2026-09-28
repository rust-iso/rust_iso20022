#!/usr/bin/env bash
#
# Run the common Spec Kit work-package gates.
#
# Usage:
#   scripts/check-work-package.sh
#
# This intentionally selects representative feature-gated suites instead of
# relying on a default-only run that can report zero tests for generated models.
set -euo pipefail

cd "$(dirname "$0")/.."

# Generated model crates are extremely large. Debug symbols add substantial
# memory and link cost without improving these CI-style pass/fail gates.
export CARGO_PROFILE_DEV_DEBUG="${CARGO_PROFILE_DEV_DEBUG:-0}"
export CARGO_PROFILE_TEST_DEBUG="${CARGO_PROFILE_TEST_DEBUG:-0}"

run() {
    echo ">> $*"
    "$@"
}

run cargo +stable fmt --check
run cargo +stable clippy --workspace --all-targets --jobs 1 -- -D warnings

# Run the adapter/workspace contract once. Feature-specific tests belong to the
# active WP; the expensive cross-family generated-model matrix is a release
# gate in `scripts/check-model-matrix.sh`, not a tax on every work package.
run cargo +stable test --workspace --no-default-features --jobs 1

# The core/CLI/codegen/WASM packages retain Rust 1.85. The separate MCP
# adapter follows the official rmcp SDK's Rust 1.88 requirement.
run cargo +1.85 check --workspace --exclude rust_iso20022_mcp --no-default-features --jobs 1

run git diff --check

if git diff --unified=0 -- '*.rs' |
    grep '^+' |
    grep -Ev '^\+\+\+' |
    grep -E '(todo!|unimplemented!|panic!|TODO|PLACEHOLDER)' >/dev/null; then
    echo "error: active Rust diff contains a placeholder or explicit panic path" >&2
    git diff --unified=0 -- '*.rs' |
        grep '^+' |
        grep -Ev '^\+\+\+' |
        grep -E '(todo!|unimplemented!|panic!|TODO|PLACEHOLDER)' >&2
    exit 1
fi

echo ">> Common work-package gates passed"
