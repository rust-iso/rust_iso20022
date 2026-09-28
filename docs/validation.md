# Validation

`rust_iso20022` separates validation into three explicit layers:

1. L1 — bounded XML syntax and schema validation. XML safety checks are
   available; full XSD validation remains explicitly unavailable until the
   backend gate in `docs/decisions/xsd-backend.md` passes.
2. L2 — ISO 20022 semantic rules, including the financial scalar rules below.
3. L3 — rules tied to an explicit market or network profile release.

Every run produces a `ValidationReport`. A report is valid only when it has no
errors and every requested layer was available. Issues carry a stable rule ID,
severity, logical field path, safe explanation, layer, and source metadata.
Each executable rule's immutable descriptor also records all affected message
patterns and logical field paths so `explain` is traceable before a failure is
observed.

## Validated financial values

The `helpers` module provides typed, value-free failures for:

- IBAN national length/structure and MOD-97 checksum;
- BIC/BICFI syntax and ISO country component;
- LEI syntax and MOD-97 checksum;
- active ISO 4217 currency and ISO 3166-1 alpha-2 country codes;
- non-negative amounts, currency minor-unit precision, and an 18-digit bound;
- calendar dates and date-times with an explicit UTC designator or offset;
- bounded account, clearing-scheme, and clearing-member identifiers.

These helpers are additive scalar values, not a second message model. They
convert into `String` or amount parts that builders place into the existing
generated leaf newtypes. Builders and validators always end at generated
message structs.

```rust
use rust_iso20022::helpers::{Iban, Money};

let iban = Iban::parse("DE89370400440532013000")?;
assert_eq!(iban.as_str(), "DE89370400440532013000");
assert!(!iban.account_existence_verified());

let money = Money::parse("EUR", "125.50")?;
assert_eq!(money.currency().as_str(), "EUR");
assert_eq!(money.amount(), "125.50");
# Ok::<(), rust_iso20022::helpers::ValueError>(())
```

## Explainable L2 registry

`validation::financial_rules()` returns the same executable rule objects used
for `RuleRegistry::validate`, `RuleRegistry::explain`, and
`RuleRegistry::explain_str`. The string lookup exists for process adapters and
returns the same static descriptor; it is not a second explanation index.
Adapters should map generated fields to `ValidationTarget` paths; they must not
copy a message into a parallel DTO.

```rust
use rust_iso20022::validation::{
    financial_rules, RuleRegistry, ValidationContext, ValidationTarget,
};

let registry = RuleRegistry::new(financial_rules())?;
let fields = [("$financial.iban", "DE88370400440532013000")];
let report = registry.validate(
    &ValidationTarget::from_pairs(&fields),
    &ValidationContext::default(),
);
assert_eq!(report.errors()[0].rule_id.as_str(), "ISO20022-L2-IBAN-CHECKSUM");

let explanation = registry
    .explain_details_str("ISO20022-L2-IBAN-CHECKSUM")
    .unwrap();
assert!(explanation.descriptor.source.unwrap().contains("SWIFT"));
assert_eq!(explanation.field_paths, vec!["$financial.iban"]);
# Ok::<(), rust_iso20022::validation::DuplicateRuleId>(())
```

`explain` describes rules implemented by this SDK. A source/reference string is
a traceability pointer, not a claim of certification or permission to
redistribute a standards document. Profile descriptors must remain bound to an
exact release and are unavailable when the corresponding reviewed rule pack is
not installed.

## Reference snapshots

The offline data is pinned in `reference-data/manifest.json`:

| Dataset | Pinned release | Records |
|---|---|---:|
| IBAN structure | SWIFT IBAN Registry release 102, June 2026 | 89 |
| Country codes | `rust_iso3166` 0.2.0 dataset | 249 |
| Active currencies | SIX ISO 4217 List One, 2026-09-17 | 178 |

Run `cargo run -p iso20022-reference-gen -- --check` to verify every snapshot,
manifest digest, and generated Rust table without network access.

## Scope and compliance

Checksum or syntax success does not prove account existence, BIC directory
membership, LEI issuance/status, bank acceptance, regulatory certification,
network onboarding approval, or legal compliance. It means only that the value
passed the implemented, versioned rules described by its rule metadata.
