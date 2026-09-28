# Builders

Builders are additive construction helpers over the canonical generated model.
They are consumed by `build` and return the exact generated `Document`; they do
not define a high-level ISO 20022 message hierarchy.

The first pacs and pain slices support:

| Builder | Exact output type |
|---|---|
| `Pacs008Builder` | `generated::pacs::pacs_008_001_08::Document` |
| `Pacs009Builder` | `generated::pacs::pacs_009_001_08::Document` |
| `Pacs002Builder` | `generated::pacs::pacs_002_001_10::Document` |
| `Pain001Builder` | `generated::pain::pain_001_001_09::Document` |
| `Pain002Builder` | `generated::pain::pain_002_001_10::Document` |
| `Camt052Builder` | `generated::camt::camt_052_001_09::Document` |
| `Camt053Builder` | `generated::camt::camt_053_001_09::Document` |
| `Camt054Builder` | `generated::camt::camt_054_001_09::Document` |

Enable `model-pacs`, validate scalar inputs with the `helpers` module, and keep
using the generated fields after construction:

```rust
use rust_iso20022::builders::Pacs008Builder;
use rust_iso20022::generated::pacs::pacs_008_001_08;
use rust_iso20022::helpers::Money;

let document: pacs_008_001_08::Document = Pacs008Builder::new()
    .message_id("MSG-1")
    .transaction_id("TX-1")
    .settlement(Money::parse("EUR", "125.50")?)
    .build()?;

assert_eq!(document.fi_to_fi_cstmr_cdt_trf.grp_hdr.msg_id.0, "MSG-1");
# Ok::<(), Box<dyn std::error::Error>>(())
```

`BuilderError` is typed and contains only a stable code, logical input field,
and category. It never echoes account data or other source values. Missing
fields and scalar boundaries are rejected before a generated value is returned.
The pacs.008 builder also runs the shared direct-generated cross-field rules.
The pain.001 builder accepts a validated `Money` helper but writes the amount
directly into `CreditTransferTransaction34`; the helper is consumed and never
becomes a parallel stored payment model. The pain.002 builder produces the
generated status report and retains its generated original-group structure.

The camt builders accept `CamtReportId`, a scoped Max35 helper used for report,
statement, and notification identifiers. It provides input validation and
conversion only: `build` consumes it into the selected generated identifier
field, and account reports/statements/notifications continue to be represented
solely by their generated types. The first camt slice declares version
`.001.09`, matching the available schema set rather than inventing the stale
`.001.08` example identifier found in older project prose.

These builders intentionally cover one declared version per first-phase
message. They do not apply CBPR+ or SEPA policy and do not claim bank or network
acceptance. Profile validation remains an explicit subsequent step bound to a
specific profile release.

Run the complete example with:

```bash
cargo run --example build_pacs --features model-pacs
cargo run --example build_pain --features model-pain
cargo run --example build_camt --features model-camt
```
