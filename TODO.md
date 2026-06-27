# TODO

Status of `rust_iso20022`. ✅ = done, ⬜ = outstanding.

## Foundation
- ✅ Hand-written core: `MxId`, `BusinessArea` (37 areas), `Error`, `from_xml`/`to_xml`/`to_xml_fragment`, `from_namespace`
- ✅ `validate` module (local `Validate` trait)
- ✅ Codegen tool driving `xsd-parser` (now `tools/codegen`, separate crate)
- ✅ Generated message model — **502/502** messages across 25 business areas
- ✅ Message `catalogue` (phf tables: id → namespace/area/has_model)
- ✅ Runtime XSD `fetch`er (`catalogue` feature)
- ✅ `iso20022` CLI (`cli` feature) — catalogue lookup
- ✅ `LICENSE` (Apache-2.0), `.gitignore`, `scripts/publish.sh`
- ✅ README, CHANGELOG, doc-tested examples, integration tests

## prowide-iso20022 parity (差异)
- ✅ Typed model + XML parse/serialize (`from_xml`/`to_xml`)
- ✅ `AbstractMX` identity — `MxMessage` trait on every `Document`
- ✅ Auto-detection — `detect(xml)`, `parse_as::<T>()`, `AnyMessage` + `parse_auto`
- ✅ Business Application Header reader — `app_hdr::parse_business_header`
- ✅ JSON — `to_json`/`from_json`, serde on message types (`serde` feature)
- ✅ Fixed the 7 multi-`<choice>` messages (codegen disambiguation)

## Bugs fixed (潜在bug / serde)
- ✅ Default-namespace parsing ("bad namespace") — messages now parse
- ✅ Monetary amount value recovered (`simpleContent`)
- ✅ serde wired through core, catalogue and generated message types
- ✅ Publishable to crates.io — removed all git deps (`xsd-types` dropped,
  `xsd-macro-utils` → local `simple_type!` macro, `xsd-parser` → `tools/codegen`)

## Repo hygiene
- ✅ `.gitignore`
- ✅ `thrdpty/` excluded from the published package and git-ignored
- ✅ Git repo initialised (`main`, initial commit, clean tree)

## Outstanding
- ✅ **JSON uses ISO tag names** — yaserde renames/flatten mirrored onto serde, so
  JSON keys are `MsgId`, `IBAN`, … like prowide
- ✅ **Document metadata extraction** — `metadata::extract` reads
  message-id/amount/currency/value-date/etc. (prowide `MxSwiftMessage`)
- ✅ **AppHdr build** — `BusinessHeader::to_app_hdr_xml` writes a `head.001`
  `<AppHdr>` (round-trips through the reader)
- ✅ **`<__Unknown__>` no longer leaked** — `to_xml` strips the synthetic
  `<__Unknown__>` placeholder elements that unset choices produce (round-trip
  preserved). A minOccurs-aware fix that also omits the now-empty parent element
  is still a future refinement.
- ✅ **`Ccy` on choice-amounts — root-fixed.** codegen models **all**
  `<xsd:choice>`s (top-level *and* inline/nested) as a struct of `Option<…>`
  fields (JAXB/prowide style), so the amount struct serializes as a normal
  element with its `Ccy` attribute and the `__Unknown__` placeholder is gone for
  choices. pain.001 `InstdAmt Ccy` round-trips; verified across the inline-choice
  families (reda/semt/secl/caam/camt/pain).
- ✅ **Business-message reader** — `read_business_message` returns header +
  detected `MxId` + metadata in one call
- ✅ **Typed envelope** — `Envelope<D>` + `parse_envelope::<D>` lift the
  `<Document>` out of an AppHdr+Document message and parse it typed, with the
  header alongside
- ✅ **Coverage (检查xsds)** — expanded to **722 messages / 32 areas** by
  fetching `semt`, `sese`, `setr`, `tsin`, `tsmt`, `tsrv`, `trck` from
  iso20022.org's static schema path (`Fetcher::download_schema`)
- ✅ Remaining empty areas (`seti`, `trea`, `cbrf`, `supl`, `xsys`) — **N/A**:
  these have no current message definitions on iso20022.org (nothing to generate);
  they remain in `BusinessArea` for identification
- ✅ **Per-area model features + full-build verification** — `model-<area>`
  features compile a single family in seconds; the full 722-module model was
  verified to compile error-free; `AnyMessage`/`parse_auto` boxed to avoid stack
  overflow on large messages
- ✅ **Typed scalars** — `convert` feature: `to_decimal`/`to_date`/`to_datetime`
  give typed access (rust_decimal/chrono) to the lossless `String` values, matching
  prowide's typed getters without a risky full retype
- ✅ **WASM support** — `src/wasm.rs` exposes identification/catalogue/metadata to
  JS/npm (`--cfg direct_wasm`, `scripts/build-wasm.sh`); verified on wasm32
- ✅ **Test corpus** — codegen emits a per-message structural smoke test (every
  `Document` constructs + serializes, on a large-stack thread) in each family;
  run via `cargo test --features model-<area>`
- ✅ **Schema provenance unified** — re-sourced 721/722 schemas from

## Schema fetch gaps (2026-06-27)
5 current messages are not downloadable from iso20022.org — the static schema
path 404s and the `/message/{id}/download` endpoint is Akamai-blocked for
non-browser clients. Fetch manually (in a browser) and drop into `xsds/`:
- ⬜ camt.088.001.04 (id 23566)
- ⬜ sese.020.001.09 (id 23573)
- ⬜ sese.021.001.08 (id 23574)
- ⬜ sese.022.001.08 (id 23575)
- ⬜ sese.023.001.13 (id 23576)
