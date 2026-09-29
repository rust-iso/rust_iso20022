#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

metadata="$(cargo +stable metadata --no-deps --format-version 1)"
expected='{
  "payments": ["model-head", "model-acmt", "model-admi", "model-auth", "model-camt", "model-pacs", "model-pain", "model-reda", "model-remt"],
  "securities": ["model-secl", "model-seev", "model-semt", "model-sese", "model-setr"],
  "trade": ["model-colr", "model-tsin", "model-tsmt", "model-tsrv"],
  "cards": ["model-caaa", "model-caad", "model-caam", "model-cafc", "model-cafm", "model-cafr", "model-casp", "model-casr", "model-catm", "model-catp"],
  "fx": ["model-fxtr"]
}'

for alias in payments securities trade cards fx; do
  actual="$(jq -c --arg alias "$alias" '.packages[] | select(.name == "rust_iso20022") | .features[$alias] // []' <<<"$metadata")"
  wanted="$(jq -c --arg alias "$alias" '.[$alias]' <<<"$expected")"
  if [[ "$actual" != "$wanted" ]]; then
    echo "feature alias drift: $alias" >&2
    echo "expected: $wanted" >&2
    echo "actual:   $actual" >&2
    exit 1
  fi
done

echo "feature aliases match the documented domain composition"
