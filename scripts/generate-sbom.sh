#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

output_dir="${1:-evidence/release-baseline/sbom}"
mkdir -p "$output_dir"

if ! command -v cargo-cyclonedx >/dev/null 2>&1; then
  echo "error: cargo-cyclonedx 0.5.9 is required" >&2
  echo "install with: cargo +stable install cargo-cyclonedx --version 0.5.9 --locked" >&2
  exit 2
fi

version="$(cargo +stable cyclonedx --version)"
case "$version" in
  *" 0.5.9") ;;
  *) echo "error: expected cargo-cyclonedx 0.5.9, found: $version" >&2; exit 2 ;;
esac

export SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-$(git log -1 --format=%ct)}"
output="$output_dir/rust_iso20022.cdx.json"
scratch="$(mktemp -d)"
output_stem="iso20022-sbom-$(basename "$scratch")"
cleanup() {
  if [[ -s "$scratch/packages.tsv" ]]; then
    while IFS=$'\t' read -r name manifest; do
      rm -f -- "$(dirname "$manifest")/$output_stem.json"
    done < "$scratch/packages.tsv"
  fi
  rm -rf -- "$scratch"
}
trap cleanup EXIT

# The tool writes beside each workspace manifest and accepts a filename only.
cargo +stable metadata --locked --all-features --format-version 1 > "$scratch/metadata.json"
jq -r '.workspace_members as $members | .packages[] |
  select(.id as $id | $members | index($id)) |
  [.name, .manifest_path] | @tsv' "$scratch/metadata.json" > "$scratch/packages.tsv"
cp Cargo.lock "$scratch/Cargo.lock"

cargo +stable cyclonedx \
  --manifest-path Cargo.toml \
  --format json \
  --spec-version 1.5 \
  --no-build-deps \
  --all-features \
  --target all \
  --override-filename "$output_stem"

cmp Cargo.lock "$scratch/Cargo.lock"
while IFS=$'\t' read -r name manifest; do
  generated="$(dirname "$manifest")/$output_stem.json"
  jq -e --arg name "$name" '.bomFormat == "CycloneDX" and
    .specVersion == "1.5" and .metadata.component.name == $name' "$generated" >/dev/null
  mv -- "$generated" "$output_dir/$name.cdx.json"
  (cd "$output_dir" && shasum -a 256 "$name.cdx.json" > "$name.cdx.json.sha256")
done < "$scratch/packages.tsv"
test -s "$output"
printf '%s\n' "$version" > "$output_dir/tool-version.txt"
echo "wrote $output"
