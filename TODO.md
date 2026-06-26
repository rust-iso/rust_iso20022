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
- ⬜ **Unset choices serialize as `<__Unknown__>`** — needs minOccurs-aware
  `Option` generation in codegen (Option-wrapping alone breaks round-trip)
- ⬜ **`Ccy` on choice-nested amounts not written** — yaserde flatten+enum limit
- ✅ **Business-message reader** — `read_business_message` returns header +
  detected `MxId` + metadata in one call
- ⬜ **Typed BAH envelope** — `read_business_message` covers reading; a fully
  *typed* `Envelope<head.001 BAH, Document>` (with the model) is still open
- ⬜ **Coverage (检查xsds)** — 12 business areas have no model (`semt`, `sese`,
  `seti`, `setr`, `tsin`, `tsmt`, `tsrv`, `trea`, `cbrf`, `supl`, `trck`,
  `xsys`); the current mirror lacks their XSDs (~502 of the official ~793)
- ⬜ **Typed scalars** — values are `String` (lossless but untyped `Decimal`/`Date`)
- ✅ **WASM support** — `src/wasm.rs` exposes identification/catalogue/metadata to
  JS/npm (`--cfg direct_wasm`, `scripts/build-wasm.sh`); verified on wasm32
- ⬜ **Test gaps** — model-gated doctests are `ignore`d; full `--features model`
  build/round-trip corpus not exercised (exceeds the dev sandbox compile ceiling)
- ⬜ **Schema provenance** — sourced from a GitHub mirror, not authoritative
  iso20022.org; diff against official XSDs
