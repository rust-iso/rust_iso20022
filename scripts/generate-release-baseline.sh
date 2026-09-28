#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

output_dir="${1:-evidence/release-baseline}"
mkdir -p "$output_dir"

schema_count="$(jq -r '.schema_count' xsds/schema-set.json)"
schema_set_id="$(jq -r '.schema_set_id' xsds/schema-set.json)"
generator_version="$(jq -r '.generator.version' tools/codegen/generator-manifest.json)"
generator_digest="$(jq -r '.generator.source_sha256' tools/codegen/generator-manifest.json)"
core_msrv="$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name == "rust_iso20022") | .rust_version')"
mcp_msrv="$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name == "rust_iso20022_mcp") | .rust_version')"

jq -n \
  --argjson schema_count "$schema_count" \
  --arg schema_set_id "$schema_set_id" \
  --arg generator_version "$generator_version" \
  --arg generator_sha256 "$generator_digest" \
  --arg core_msrv "$core_msrv" \
  --arg mcp_msrv "$mcp_msrv" \
  '{
    format_version: 1,
    release_state: "unpublished-candidate",
    canonical_model: "generated struct",
    schema: {set_id: $schema_set_id, message_versions: $schema_count},
    generator: {version: $generator_version, source_sha256: $generator_sha256},
    msrv: {core: $core_msrv, mcp: $mcp_msrv},
    generated_validation_bindings: ["pacs.008.001.08"],
    builders: [
      "pacs.002.001.10", "pacs.008.001.08", "pacs.009.001.08",
      "pain.001.001.09", "pain.002.001.10", "camt.052.001.08",
      "camt.053.001.09", "camt.054.001.08"
    ],
    migrations: [
      "MT103 -> pacs.008.001.08",
      "MT202 -> pacs.009.001.08",
      "MT940 -> camt.053.001.09"
    ],
    executable_profile_rule_packs: [],
    profile_source_identities: ["CBPR+ SR2026", "SEPA SCT 2025 v1.1", "SEPA SCT Inst 2025 v1.1"]
  }' > "$output_dir/support.json"

cp docs/feature-matrix.md "$output_dir/feature-matrix.md"
cp docs/status.md "$output_dir/known-limitations.md"
cp xsds/schema-set.json "$output_dir/schema-set.json"
cp xsds/schema-manifest.json "$output_dir/schema-manifest.json"
cp tools/codegen/generator-manifest.json "$output_dir/generator-manifest.json"
cp profiles/cbpr_plus/source-review.json "$output_dir/cbpr-plus-source-review.json"
cp profiles/sepa/source-review.json "$output_dir/sepa-source-review.json"

(
  cd "$output_dir"
  find . -maxdepth 1 -type f ! -name SHA256SUMS -print0 |
    sort -z |
    xargs -0 shasum -a 256 > SHA256SUMS
)

echo "wrote release baseline to $output_dir"

