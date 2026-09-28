#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

output_dir="${1:-evidence/release-baseline/sbom}"
mkdir -p "$output_dir"

if ! command -v cargo-cyclonedx >/dev/null 2>&1; then
  echo "error: cargo-cyclonedx 0.5.9 is required" >&2
  echo "install with: cargo install cargo-cyclonedx --version 0.5.9 --locked" >&2
  exit 2
fi

version="$(cargo cyclonedx --version)"
case "$version" in
  *0.5.9*) ;;
  *) echo "error: expected cargo-cyclonedx 0.5.9, found: $version" >&2; exit 2 ;;
esac

export SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-$(git log -1 --format=%ct)}"
output="$output_dir/rust_iso20022.cdx.json"
output_stem="$output_dir/rust_iso20022.cdx"

cargo cyclonedx \
  --manifest-path Cargo.toml \
  --format json \
  --spec-version 1.5 \
  --no-build-deps \
  --override-filename "$output_stem"

jq -e '.bomFormat == "CycloneDX" and .specVersion == "1.5"' "$output" >/dev/null
shasum -a 256 "$output" > "$output.sha256"
printf '%s\n' "$version" > "$output_dir/tool-version.txt"
echo "wrote $output"
