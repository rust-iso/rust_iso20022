# Changelog

All notable changes to this project are documented here. This project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - unreleased

### Fixed
- docs.rs now renders the generated message model. The `generated` module is
  gated behind `__model`/`model-*` (off by default), so docs.rs — which builds
  with default features only — previously published an empty crate with no
  message modules. Added `[package.metadata.docs.rs]` to build the docs with
  the full `model` plus `serde`/`convert`/`cli`.

## [0.1.0] - unreleased

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
