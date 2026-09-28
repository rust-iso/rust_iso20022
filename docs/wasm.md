# WebAssembly support

The repository provides two WebAssembly surfaces:

- The original root-crate package preserves all 22 exports recorded in
  `tests/compatibility/wasm-exports.json`. Its structured JSON is now produced
  by serde-backed compatibility shims rather than manual string construction.
- The separate `rust_iso20022_wasm` workspace crate provides bounded,
  structured JS values for the verified SDK subset. It is not a second message
  model: XML/JSON parsing and serialization always use the core
  `ParsedMessage`, whose value is an existing generated struct.

Neither crate is part of the root crate's default feature set. No package is
published by the repository's verification commands.

## Verified structured API

The separate package exports:

| Function | Verified behavior |
|---|---|
| `detect_message(xml)` | Bounded namespace/root detection |
| `catalogue_message(messageId)` | Generated descriptor and schema hash |
| `parse_message(xml)` | XML to the selected canonical generated struct, returned as a JS object |
| `serialize_message(messageId, document)` | JS object to the exact generated struct and XML |
| `validate_message(xml, layer)` | L2 core binding and structured report |

Results are ordinary JS objects produced with `serde-wasm-bindgen`. Failures
are thrown as objects with stable `code`, generic `message`, and an optional
`required_feature`. Input content is not included in errors.

```js
import init, {
  detect_message,
  parse_message,
  serialize_message,
  validate_message,
} from "./rust_iso20022_wasm.js";

await init();
const detected = detect_message(xml);
const parsed = parse_message(xml);
const report = validate_message(xml, "l2");
const roundTrip = serialize_message(parsed.message_id, parsed.document);
```

## Supported target and feature matrix

| Surface | Target | Generated models | Validation | Status |
|---|---|---|---|---|
| Root legacy package | `wasm32-unknown-unknown` | None exposed | None exposed | Supported; 22-export compatibility baseline |
| Structured package + `model-pacs` | `wasm32-unknown-unknown` | pacs family | L2 for installed generated bindings | Supported and tested in Node and Chrome |
| Structured package, no features | `wasm32-unknown-unknown` | None | None | Detection and catalogue only |
| Structured package | Native host | Same core calls | Same L2 binding | Supported for parity/security tests; not a JS package |
| L1 XSD validation | WASM | — | L1 | Unavailable, returns `validation_unavailable` |
| L3 market profiles | WASM | — | L3 | Unavailable, returns `validation_unavailable` |
| Non-pacs generated family | Structured package | Not compiled | — | Preflight error `unsupported_feature` with the exact `required_feature` |

The structured crate currently compiles the complete pacs family because the
core feature is family-granular. The measured optimized browser artifact is
**33,284,824 bytes**. This is a verified baseline and a known limitation, not a
small-bundle claim. Applications needing detection and catalogue only should
continue to use the smaller legacy/core WASM surface until finer-grained
generated-model features are implemented and measured.

## Limits and privacy

XML detection runs with `ParseLimits::DEFAULT` before generated
deserialization or validation. JSON serialization input is capped at the same
byte limit before generated deserialization. The Node boundary suite exercises
a message larger than 16 MiB and verifies a structured `invalid_input` result,
no panic, and no sensitive-value echo. XML entity declarations, excessive
depth, malformed namespaces, invalid Unicode declarations, and oversized
collections inherit the core bounded-parser policy.

The bindings do not log financial messages. The installed panic hook suppresses
panic payloads, locations, and backtraces because they may contain
caller-controlled values. Core errors and adapter errors remain redacted.

## Build and verification

Build the original package:

```console
scripts/build-wasm.sh
scripts/check-wasm-compat.sh --build
```

Build and execute the structured package in Node and a real headless Chrome
runtime:

```console
scripts/check-wasm-sdk.sh # builds the structured crate with model-pacs
```

The Chrome harness serves the generated browser ES module and `.wasm` from a
temporary loopback HTTP server, then performs detection and catalogue parity in
the page. It does not require ChromeDriver. Native object-parity and security
tests run with:

```console
cargo test -p rust_iso20022_wasm --tests
cargo test --test wasm_native_contract --features serde
```

## Compliance boundary

A successful result means the message passed the implemented and selected SDK
rules. It does not guarantee acceptance by a bank or network, regulatory
certification, onboarding approval, or legal compliance.
