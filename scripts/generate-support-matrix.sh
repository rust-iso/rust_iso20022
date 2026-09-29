#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

output="${1:-docs/support-matrix.md}"
mkdir -p "$(dirname "$output")"
cargo +stable run --offline --quiet --example support_matrix > "$output"
echo "wrote schema-derived support matrix to $output"
