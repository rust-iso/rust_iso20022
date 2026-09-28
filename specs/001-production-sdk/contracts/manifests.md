# Contract: Schema, Generator, and Evidence Manifests

All manifests use UTF-8 JSON with a declared `format_version`, canonical key
spelling, stable array order, relative POSIX-style paths, and lowercase SHA-256.
Unknown provenance is represented by an explicit status, not an invented value.

## Schema Set

```json
{
  "format_version": 1,
  "schema_set_id": "iso20022-repository-<declared-release-or-unverified>",
  "provenance_status": "verified|partially_verified|unverified",
  "aggregate_sha256": "<64 hex>",
  "schemas": ["<schema-record-id in canonical order>"]
}
```

## Schema Record

```json
{
  "id": "pacs.008.001.10",
  "relative_path": "xsds/pacs.008.001.10.xsd",
  "sha256": "<64 hex>",
  "bytes": 12345,
  "publisher": "ISO 20022",
  "source_url": null,
  "source_release": null,
  "publication_timestamp": null,
  "provenance_status": "unverified",
  "namespace": "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.10",
  "root_element": "Document",
  "root_type": "Document",
  "generated_module": "generated::pacs::pacs_008_001_10",
  "generation_status": "generated"
}
```

## Generator Manifest

```json
{
  "format_version": 1,
  "generator_version": "<crate semver>",
  "generator_source_sha256": "<64 hex>",
  "xsd_parser_revision": "<full immutable revision>",
  "cargo_lock_sha256": "<64 hex>",
  "config_sha256": "<64 hex>",
  "rust_toolchain": "<exact channel/version>",
  "metadata_format_version": 1,
  "outputs": [
    {"relative_path": "src/generated/...", "sha256": "<64 hex>", "bytes": 123}
  ],
  "aggregate_sha256": "<64 hex>"
}
```

The generator fails on missing/unlisted input, stale/unlisted output, duplicate
identity, invalid hash, empty required namespace/root, or any partial conversion.

## Profile Source Manifest

Records profile scheme/release, guideline edition, effective range, document ID,
publisher, URL, section/reference, digest, access classification, and the recorded
implementation/distribution decision. It does not embed source content.

## Reference-Data Manifest

Records data kind (IBAN lengths, currency, country, LEI), publisher, release,
source URL, digest, license note/review, transform version, and generated output
digest. It distinguishes format/checksum capability from registry existence.

## Evidence Record

Each WP evidence document records WP ID, commit/tree state, date, toolchain,
platform, commands, feature/target selections, fixture/source digests, exit
status, summarized result, known limitations, and acceptance-criterion mapping.
Benchmark records additionally include CPU/OS and measurement tool; fuzz records
include engine/toolchain, target, duration/executions, corpus digest, and
timeout/OOM/crash count.

