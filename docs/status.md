# Status

Implementation status of `rust_iso20022`. ✅ = done, ⬜ = outstanding.

## Done

### Model & coverage
- ✅ Typed message model — **1130 message versions across 32 business areas**,
  generated from the official iso20022.org XSD schemas (in `xsds/`).
- ✅ Per-area `model-<area>` features (a single family compiles in seconds);
  `model` enables all areas. Full model verified to compile error-free.
- ✅ Codegen tool (`tools/codegen`, a separate unpublished crate driving
  `xsd-parser`) with transforms for default-namespace parsing, `simpleContent`
  amount values, multi-`<choice>` disambiguation, and modelling every
  `<xsd:choice>` as a struct of `Option<…>` fields.
- ✅ Per-message structural smoke test emitted per family (`cargo test
  --features model-<area>`).

### Core & API
- ✅ Hand-written core: `MxId`, `BusinessArea` (37 areas), `Error`,
  `from_xml`/`to_xml`/`to_xml_fragment`, `from_namespace`, `validate`.
- ✅ Per-`Document` identity via the `MxMessage` trait.
- ✅ Auto-detection — `detect(xml)`, `parse_as::<T>()`, `AnyMessage` +
  `parse_auto` (boxed to avoid stack overflow on large messages).
- ✅ Business Application Header — `app_hdr::parse_business_header` (read) and
  `BusinessHeader::to_app_hdr_xml` (build), version-independent, round-trips.
- ✅ Business metadata — `metadata::extract` (message id, amount, currency,
  value date, …).
- ✅ Typed envelope — `Envelope<D>` / `parse_envelope::<D>` and
  `read_business_message` (header + detected `MxId` + metadata in one call).
- ✅ Generic tree — `MxNode::parse` reads any message without the model.
- ✅ JSON — `to_json`/`from_json` using ISO 20022 element names (`serde` feature).
- ✅ Typed scalars — `convert` feature (`to_decimal`/`to_date`/`to_datetime`).
- ✅ Message `catalogue` (phf tables) + runtime XSD `fetch`er (`catalogue`).
- ✅ `iso20022` CLI (`cli` feature) — catalogue lookup.
- ✅ WASM — `src/wasm.rs` exposes the identification/catalogue/header/metadata/
  generic-tree layer to JS (`--cfg direct_wasm`, `scripts/build-wasm.sh`); see
  [wasm-api.md](wasm-api.md).

### Packaging
- ✅ No git dependencies — publishable to crates.io. The codegen tooling and the
  `xsds/` sources are excluded from the published package.
- ✅ `LICENSE` (Apache-2.0), `.gitignore`, `scripts/publish.sh`,
  `scripts/build-wasm.sh`.
- ✅ README, CHANGELOG, runnable examples, integration + doc tests.

## Outstanding

### Schema fetch gaps (2026-06-27)
5 current messages are not downloadable from iso20022.org — the static schema
path 404s and the `/message/{id}/download` endpoint is Akamai-blocked for
non-browser clients. Fetch manually (in a browser) and drop into `xsds/`, then
regenerate:
- ⬜ camt.088.001.04 (id 23566)
- ⬜ sese.020.001.09 (id 23573)
- ⬜ sese.021.001.08 (id 23574)
- ⬜ sese.022.001.08 (id 23575)
- ⬜ sese.023.001.13 (id 23576)

### Known constraint
- `yaserde` is pinned at 0.7 — the generated model targets its derive semantics;
  0.8+ changed the derive macro incompatibly (the 0.12 derive panics on the
  generated attributes), so moving up needs a full regenerate + re-validation.
