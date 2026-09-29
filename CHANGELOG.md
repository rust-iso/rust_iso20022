# Changelog

All notable changes to this project are documented here. This project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.3] - 2026-09-29

### Added

- Added opt-in public-reference profile bundles for CBPR+ SR2026, SEPA SCT
  2025-v1.1, and SEPA SCT Inst 2025-v1.1. Each bundle exposes explicit source,
  release, provenance status, explainable rule IDs, and shared validation-engine
  integration without creating another canonical message model.
- Added focused public-reference profile guides, synthetic fixture boundaries,
  and evidence covering all 13 provisional rule IDs and field paths.
- Added reproducible local rustdoc generation and GitHub Pages deployment as a
  stable documentation fallback when docs.rs infrastructure fails before cargo
  starts.

### Fixed

- Made CI invoke stable or explicit MSRV toolchains so the Rust 1.85 core/MSRV
  job does not accidentally build the separately gated Rust 1.88 MCP crate.
- Pinned the MSRV-compatible `cargo-audit` release in supply-chain CI.

### Documentation

- Reduced docs.rs memory pressure by retaining the representative `head`,
  `pacs`, and `pain` feature set. Full generated models remain available in the
  crate and can be documented locally by selecting their model features.
- Pointed package documentation metadata at the GitHub Pages fallback while
  retaining docs.rs as a mirror.

### Compatibility

- This release is additive. Existing generated model paths, serialization,
  feature aliases, and the Rust 1.85 core MSRV are unchanged.
- Public-reference profiles remain explicitly `provisional`; this release does
  not claim complete SWIFT/EPC rulebook conformance, certification, or bank
  acceptance.

## [0.1.2] - 2026-09-28

Published to crates.io as `rust_iso20022` 0.1.2. CBPR+ and SEPA candidate
profile checks remain explicitly provisional until exact authoritative release
artifacts and digests are available.

### Added

- Added schema/codegen manifests, per-schema hashes, generated descriptors, a
  schema-derived message catalogue, and metadata-based version comparison.
- Added a unified message abstraction, bounded XML input handling, structured
  layered validation reports, explainable rule IDs, financial-field validators,
  and reusable cross-field rules.
- Added builders returning canonical generated structs for pacs.002/008/009,
  pain.001/002, and camt.052/053/054.
- Added exact-release profile/provenance frameworks. CBPR+ SR2026 and SEPA 2025
  source identities are recorded, but no complete executable L3 profile rule
  pack is claimed in this release candidate.
- Added opt-in provisional CBPR+ and SEPA/SCT/SCT Inst candidate rules for
  scalar, cross-field, remittance, currency, and amount-limit checks. These
  rules are not presented as network acceptance or certification.
- Added MT103, MT202, and MT940 migration adapters with explicit exact/derived/
  lossy/ambiguous/unsupported mapping reports.
- Added separate CLI, local stdio MCP, structured WASM, and benchmark workspace
  crates. These adapters delegate domain behavior to the core SDK.
- Added security/privacy limits, redaction, fuzz regression corpora, performance
  baselines, supply-chain policy, CycloneDX generation, and unpublished release
  candidate automation.

### Fixed
- Keep docs.rs builds within its 6.4 GB sandbox limit by documenting the
  representative `head`, `pacs`, and `pain` model families instead of all
  1,130 generated message modules at once. The full model remains available to
  users through the `model` feature.
- Brotli-compress the generated schema metadata corpus. This preserves all
  descriptors and generated paths while keeping the verified `.crate` archive
  below crates.io's 10 MiB upload limit.

### Documentation
- Reworked the README around common workflows, feature selection, and runnable
  examples; added a complete model-family guide and crates.io/docs/MSRV badges.
- Added Ko-fi project sponsorship through GitHub's funding configuration.

### Changed
- Moved the library crate to Rust Edition 2024 while retaining Rust 1.85 as the
  minimum supported version. Updated the generated simple-type adapter for
  Edition 2024 match ergonomics.
- Kept the root crate MSRV at Rust 1.85; the separately unpublished MCP adapter
  requires Rust 1.88 because of its protocol SDK.
- Split capability features so generated model families, JSON, conversions,
  profiles, WASM, CLI, and MCP are not all pulled into the default build.
- Public additions are backward-compatible for the 0.1 series. Existing
  generated paths and wire formats remain the compatibility baseline. Any
  future schema-driven breaking change will be recorded separately with
  migration notes and a SemVer decision.

### Migration notes

- Existing generated types remain the sole canonical ISO 20022 model and remain
  directly accessible. Builders, validation, profiles, migration, CLI, MCP, and
  WASM APIs are additive layers.
- Applications should replace the umbrella `model` feature with the smallest
  required `model-<area>` features to reduce compile time and memory use.
- Validation success means only that implemented rules passed; it does not
  guarantee bank/network acceptance, certification, onboarding, or legal
  compliance.

## [0.1.1] - 2026-06-27

### Fixed
- docs.rs now renders the generated message model. The `generated` module is
  gated behind `__model`/`model-*` (off by default), so docs.rs — which builds
  with default features only — previously published an empty crate with no
  message modules. Added `[package.metadata.docs.rs]` to build the docs with
  the generated model and optional helper APIs.

## [0.1.0] - 2026-06-27

Initial release.

### Model & coverage
- Typed model for **1130 message versions** across **32 business areas**,
  generated from the official iso20022.org XSD schemas (current versions plus
  earlier ones). Each `Document` derives `yaserde` for XML and, with `serde`,
  for JSON.
- Per-area `model-<area>` features (e.g. `model-pacs`) so a single family
  compiles in seconds; `model` enables all areas.
- The `tools/codegen` generator (a separate, unpublished workspace crate) turns
  XSDs into the model + catalogue, with transforms that fix the upstream
  `xsd-parser` output: default-namespace parsing, `simpleContent` amount values,
  multi-`<choice>` disambiguation, and modelling every `<xsd:choice>` as a struct
  of `Option<…>` fields (so amount `Ccy` attributes serialize and no
  `__Unknown__` placeholder leaks).

### API
- **Core:** `MxId`, `BusinessArea` (37 areas), `from_xml` / `to_xml` /
  `to_xml_fragment`, `from_namespace`, `Error`.
- **Identity & dispatch:** the `MxMessage` trait on every `Document`, plus
  `detect`, `parse_as::<T>()`, and `generated::any::{AnyMessage, parse_auto}`.
- **Business Application Header:** `app_hdr::parse_business_header` (read) and
  `BusinessHeader::to_app_hdr_xml` (build), version-independent.
- **Metadata:** `metadata::extract` (message id, amount, currency, value date, …).
- **Envelopes:** `Envelope<D>` / `parse_envelope` (typed) and
  `read_business_message` (header + id + metadata).
- **Generic tree:** `MxNode::parse` — read any message by element name without
  the typed model.
- **JSON:** `to_json` / `from_json` using ISO 20022 element names (`serde`).
- **Typed scalars:** `convert::{to_decimal, to_date, to_datetime}` (`convert`).
- **Catalogue:** `catalogue` phf tables; runtime `fetch::Fetcher` (`catalogue`).
- **CLI:** the `iso20022` catalogue-lookup tool (`cli`).
- **WASM:** 21 JS bindings for the identification / catalogue / header /
  metadata / generic-tree layer (including `node_to_json` to dump a whole
  message tree), with a `console_error_panic_hook` for readable panics
  (`src/wasm.rs`).

### Packaging
- No git dependencies — publishable to crates.io. Apache-2.0 licensed, with
  `scripts/publish.sh` and `scripts/build-wasm.sh`.

### Known gaps
- 5 current messages (`camt.088.001.04`, `sese.020/021/022.001`,
  `sese.023.001.13`) are not downloadable from iso20022.org (see
  `docs/status.md`).
- `yaserde` is pinned at 0.7 (0.8+ changed the derive macro incompatibly with the
  generated code).
