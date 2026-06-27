# rust_iso20022 — WebAssembly / JavaScript API

The crate ships a WebAssembly build that exposes the **identification,
catalogue, Business Application Header, metadata and generic-tree** layers to
JavaScript. The typed message model (1130 large `Document` types) is *not*
exposed — the data/identification layer is what is useful in the browser.

All structured results are returned as **JSON strings**: `JSON.parse` them on the
JS side. Rust `Option<String>` maps to `string | undefined`; `bool` to
`boolean`; arrays to JS arrays of strings.

## Building

```bash
scripts/build-wasm.sh           # wraps: RUSTFLAGS="--cfg direct_wasm" wasm-pack build --target web --release
```

This produces an ES-module package in `./pkg`. The bindings live in
`src/wasm.rs`, gated behind `cfg(all(direct_wasm, target_arch = "wasm32"))`, so
they never affect native builds or the published crate.

## Usage

```js
import init, {
  detect,
  read_business_message,
  node_find_attr,
} from "./pkg/rust_iso20022.js";

await init(); // loads the .wasm and installs the panic hook

const id = detect(xml);                 // "pacs.008.001.08"
const bm = JSON.parse(read_business_message(xml));
const ccy = node_find_attr(xml, "IntrBkSttlmAmt", "Ccy"); // "EUR"
```

`init()` runs an automatic start hook that installs `console_error_panic_hook`,
so any Rust panic surfaces a readable message + stack trace in `console.error`
instead of an opaque `unreachable`.

---

## API reference

### Identity

#### `detect(xml: string): string | undefined`
The message name detected from a document's namespace, e.g. `"pacs.008.001.08"`.
`undefined` if not a recognisable ISO 20022 message.

#### `from_namespace(namespace: string): string | undefined`
Resolve an XSD namespace (or bare name) to its canonical message name.

#### `mx_id(namespaceOrName: string): string | undefined`
Structured identification, as a JSON string:

```json
{
  "messageName": "pacs.008.001.08",
  "namespace": "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08",
  "businessArea": "pacs",
  "functionality": "008",
  "variant": "001",
  "version": "08"
}
```

### Business areas

#### `business_area_description(code: string): string | undefined`
Human-readable name of a business-area code, e.g. `"pacs"` →
`"Payments Clearing and Settlement"`.

#### `business_areas(): string[]`
Every business-area code.

### Catalogue

#### `catalogue_contains(messageName: string): boolean`
Whether the catalogue knows the given message name.

#### `namespace_of(messageName: string): string | undefined`
The XSD namespace for a message name.

#### `business_area_of(messageName: string): string | undefined`
The business-area code of a message name, e.g. `"pacs"`.

#### `has_model(messageName: string): boolean`
Whether a typed Rust model is generated for the message.

#### `catalogue_entry(messageName: string): string | undefined`
A catalogue entry as a JSON string:

```json
{
  "messageName": "pacs.008.001.08",
  "namespace": "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08",
  "businessArea": "pacs",
  "hasModel": true
}
```

#### `catalogue_all(): string[]`
Every message name in the catalogue.

### Headers & metadata

#### `parse_app_hdr(xml: string): string | undefined`
The Business Application Header fields as a JSON string, or `undefined` if the
message has no `AppHdr`:

```json
{
  "from": "BANKBEBB",
  "to": "BANKDEFF",
  "bizMsgIdr": "MSG-2026-0001",
  "msgDefIdr": "pacs.008.001.08",
  "creDt": "2026-06-27T10:30:00Z"
}
```

Any field absent in the source is `null`.

#### `build_app_hdr(from?, to?, bizMsgIdr?, msgDefIdr?, creDt?): string`
Build a `head.001` `<AppHdr>` XML document from the given fields (each optional;
pass `undefined` to omit). Returns the XML string.

#### `extract_metadata(xml: string): string`
Business metadata extracted from any message, as a JSON string:

```json
{
  "messageId": "ABC-2026-0001",
  "creationDateTime": "2026-06-27T10:30:00Z",
  "numberOfTransactions": "1",
  "amount": "1234.56",
  "currency": "EUR",
  "valueDate": "2026-06-30"
}
```

Fields not present in the message are `null`.

#### `read_business_message(xml: string): string`
A full business message — detected type + header + metadata — in one JSON string:

```json
{
  "messageName": "pacs.008.001.08",
  "header": { "from": "BANKBEBB", "to": "BANKDEFF", "bizMsgIdr": "…", "msgDefIdr": "…", "creDt": "…" },
  "metadata": { "messageId": "…", "creationDateTime": "…", "numberOfTransactions": "…", "amount": "…", "currency": "…", "valueDate": "…" }
}
```

`messageName` and `header` are `null` when not detectable / absent.

### Generic tree

Read any message's fields without the typed model.

#### `node_text(xml: string, path: string): string | undefined`
Text at a `/`-separated path of local element names, e.g.
`node_text(xml, "FIToFICstmrCdtTrf/GrpHdr/MsgId")`.

#### `node_find(xml: string, local: string): string | undefined`
Text of the first descendant element with the given local name.

#### `node_attr(xml: string, path: string, attr: string): string | undefined`
Value of an attribute on the element at a `/`-separated path, e.g.
`node_attr(xml, "FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt", "Ccy")`.

#### `node_find_attr(xml: string, local: string, attr: string): string | undefined`
Value of an attribute on the first descendant with the given local name, e.g.
`node_find_attr(xml, "IntrBkSttlmAmt", "Ccy")` → `"EUR"`.

#### `node_find_all(xml: string, local: string): string[]`
The texts of every descendant element with the given local name (e.g. all
`EndToEndId`s in a batch).

#### `node_to_json(xml: string): string | undefined`
The whole parsed message tree as a JSON string, recursively:

```json
{
  "name": "Document",
  "value": null,
  "attributes": {},
  "children": [
    {
      "name": "FIToFICstmrCdtTrf",
      "value": null,
      "attributes": {},
      "children": [ /* … */ ]
    }
  ]
}
```

Leaf elements carry their text in `value`; element attributes (e.g. `Ccy`) are
in `attributes`. `undefined` if the XML has no root element.

---

## Function summary

| Function | Returns |
|---|---|
| `detect(xml)` | `string?` |
| `from_namespace(ns)` | `string?` |
| `mx_id(nsOrName)` | JSON `string?` |
| `business_area_description(code)` | `string?` |
| `business_areas()` | `string[]` |
| `catalogue_contains(name)` | `boolean` |
| `namespace_of(name)` | `string?` |
| `business_area_of(name)` | `string?` |
| `has_model(name)` | `boolean` |
| `catalogue_entry(name)` | JSON `string?` |
| `catalogue_all()` | `string[]` |
| `parse_app_hdr(xml)` | JSON `string?` |
| `build_app_hdr(from?, to?, …)` | XML `string` |
| `extract_metadata(xml)` | JSON `string` |
| `read_business_message(xml)` | JSON `string` |
| `node_text(xml, path)` | `string?` |
| `node_find(xml, local)` | `string?` |
| `node_attr(xml, path, attr)` | `string?` |
| `node_find_attr(xml, local, attr)` | `string?` |
| `node_find_all(xml, local)` | `string[]` |
| `node_to_json(xml)` | JSON `string?` |
