# Public-reference profiles

The SDK exposes usable, opt-in profile bundles based on public standards
metadata and independently written rules:

- `cbpr_plus_public_reference()` (`CBPR+ SR2026`)
- `sepa_sct_public_reference()` (`SEPA SCT 2025-v1.1`)
- `sepa_sct_inst_public_reference()` (`SEPA SCT Inst 2025-v1.1`)

Every bundle uses the normal validation engine and generated-message projection.
It carries source URLs and an explicit `PublicReference` status, and its rules
are versioned `provisional`. This makes the bundles useful for development,
CI, and early integration while preventing an accidental claim of SWIFT/EPC
certification or bank acceptance.

```rust
use rust_iso20022::profiles::public_reference::cbpr_plus_public_reference;
use rust_iso20022::validation::{ValidationContext, ValidationTarget};

let profile = cbpr_plus_public_reference();
let report = profile.into_registry().validate(
    &ValidationTarget::from_pairs(&[("$financial.currency", "EUR")]),
    &ValidationContext::default(),
);
assert!(report.valid());
```

These bundles intentionally do not enter the authoritative release registry.
Promotion requires exact source artifacts, SHA-256 digests, rule references,
and independently verified fixtures. The generated ISO 20022 structs remain
the only canonical message model.
