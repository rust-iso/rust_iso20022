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
- ⬜ **`Ccy` on choice-nested amounts not written** — yaserde flatten+enum limit
- ✅ **Business-message reader** — `read_business_message` returns header +
  detected `MxId` + metadata in one call
- ⬜ **Typed BAH envelope** — `read_business_message` covers reading; a fully
  *typed* `Envelope<head.001 BAH, Document>` (with the model) is still open
- ✅ **Coverage (检查xsds)** — expanded to **722 messages / 32 areas** by
  fetching `semt`, `sese`, `setr`, `tsin`, `tsmt`, `tsrv`, `trck` from
  iso20022.org's static schema path (`Fetcher::download_schema`)
- ⬜ Remaining empty areas: `seti`, `trea`, `cbrf`, `supl`, `xsys` have no current
  message definitions on iso20022.org
- ✅ **Per-area model features + full-build verification** — `model-<area>`
  features compile a single family in seconds; the full 722-module model was
  verified to compile error-free; `AnyMessage`/`parse_auto` boxed to avoid stack
  overflow on large messages
- ⬜ **Typed scalars** — values are `String` (lossless but untyped `Decimal`/`Date`)
- ✅ **WASM support** — `src/wasm.rs` exposes identification/catalogue/metadata to
  JS/npm (`--cfg direct_wasm`, `scripts/build-wasm.sh`); verified on wasm32
- ⬜ **Test gaps** — model-gated doctests are `ignore`d; per-area tests run, but a
  full round-trip corpus across all 722 messages is not exercised
- ⬜ **Schema provenance** — the original 502 came from a GitHub mirror; the 220
  new (securities/trade) messages are from authoritative iso20022.org. Re-sourcing
  the 502 from iso20022.org's static path would unify provenance
