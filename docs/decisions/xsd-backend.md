# XSD validation backend qualification (R-XSD-RUST)

Status: **no backend approved; full L1 XSD validation remains unavailable**.

Date: 2026-09-28. Product MSRV: Rust 1.85.1. The bounded well-formedness gate
is implemented independently and must remain in front of any future backend.

## Acceptance gates

A product backend must demonstrate all of the following against the complete
manifested schema set, not a toy schema:

1. Correct XSD coverage for all 1,130 schemas and representative valid/invalid
   fixtures, with no silently skipped construct.
2. In-memory, offline operation with external network/file resolution disabled.
3. Finite input, depth, event, element, attribute, text, decoded-data, and
   collection limits before validation allocates a DOM.
4. Panic-free typed failures for malformed XML, malformed schemas, and internal
   validator failures.
5. Rust 1.85.1 native support and a declared WASM outcome.
6. A maintainable license and dependency/supply-chain posture.

## Corpus inventory

Run:

```text
cargo +1.85.1 run --locked --manifest-path tools/xsd-spike/Cargo.toml -- xsds
cargo +1.85.1 test --locked --manifest-path tools/xsd-spike/Cargo.toml
```

The executable probe inventories constructs a candidate must support. At this
baseline it finds 1,130 schemas, no imports/includes, no identity constraints,
XSD 1.1 assertions, unions, or lists, and widespread `xs:any` declarations.
The absence of selected constructs narrows qualification but is not proof that
a backend correctly implements the remaining XSD 1.0 type/facet/content model.

## Candidates

| Candidate | Coverage | Offline / limits | MSRV | WASM | Decision |
|---|---|---|---|---|---|
| `libxml` / libxml2 | libxml2 exposes XSD validation, but the Rust wrapper still contains documented panic paths for schema-context creation and internal validation failures | `ParserOptions::no_net` exists; DOM validation still requires the SDK preflight limits | current 0.3.21 declares Rust 1.88; older 0.3.9 has no declared MSRV | native C dependency; not a supported browser/WASI baseline | reject for current baseline |
| pure-Rust `xmlschema` | current documentation says XSD 1.0 coverage is incomplete and specifically warns of unsupported constructs in early releases | no C dependency; still needs the SDK preflight and an audited external-resolution policy | 0.0.4 declares Rust 1.86; current releases exceed the 1.85 baseline; 0.0.1 declared 1.67.1 but is not an acceptable proxy for current correctness | potentially portable, not yet evidenced on this corpus | reject pending maturity and corpus evidence |
| generated `Validate` facets | preserves current generated scalar facet checks | offline and bounded after core preflight | passes 1.85.1 | existing WASM surface | retain as compatibility/L2 input, but it is not full XSD document validation |

Primary API evidence used for this decision:

- libxml `SchemaValidationContext` validates a DOM and returns structured error
  vectors for schema violations.
- libxml `ParserOptions` has `no_net`, while `huge` explicitly relaxes hardcoded
  parser limits and must remain disabled.
- libxml's schema wrapper source contains `panic!` branches for context creation
  and internal validation errors, which violates the SDK panic-free contract.
- the pure-Rust `xmlschema` project explicitly labels its coverage incomplete.

## Release claim

WP-006 may expose L1 as `Unavailable` with this decision reference. It must not
say “XSD valid” or “L1 passed.” Backend qualification remains open until a
candidate passes every gate above, including MSRV and WASM policy. No runtime
schema download is permitted.
