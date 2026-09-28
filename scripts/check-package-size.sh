#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

limit_bytes=$((10 * 1024 * 1024))
archive="${1:-}"

if [[ -z "$archive" ]]; then
  cargo +stable package --locked --allow-dirty --offline --no-verify
  archive="$(find target/package -maxdepth 1 -name 'rust_iso20022-*.crate' -type f | sort | tail -1)"
fi

test -n "$archive" && test -f "$archive" || {
  echo "error: package archive not found" >&2
  exit 2
}

bytes="$(stat -f '%z' "$archive" 2>/dev/null || stat -c '%s' "$archive")"
if (( bytes >= limit_bytes )); then
  echo "error: package is $bytes bytes; crates.io limit is $limit_bytes bytes" >&2
  exit 1
fi

echo "package size: $bytes bytes (limit: $limit_bytes)"

