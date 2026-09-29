# Public-reference profile implementation evidence

## Scope

This evidence covers the code-completable portion of T089–T107 without
claiming access to restricted CBPR+ Usage Guidelines or reproducing EPC source
documents. The implementation is intentionally opt-in and marked
`PublicReference`/`provisional`.

## Delivered API

- `profiles::public_reference::cbpr_plus_public_reference()`
- `profiles::public_reference::sepa_sct_public_reference()`
- `profiles::public_reference::sepa_sct_inst_public_reference()`

Each bundle exposes source references, scheme/release labels, an explicit
non-authoritative status, and the existing explainable `RuleRegistry`. No
second ISO 20022 message model was introduced.

## Rule coverage

| Bundle | Rule IDs | Coverage |
|---|---|---|
| CBPR+ SR2026 public reference | 7 | IBAN, BIC, currency, header identity, AnyBIC exclusivity, structured remittance length, commodity currency candidate |
| SEPA SCT 2025-v1.1 public reference | 3 | IBAN, ISO 4217 currency, EUR settlement candidate |
| SEPA SCT Inst 2025-v1.1 public reference | 3 | IBAN, EUR settlement, EUR 100,000 amount boundary candidate |

Tests assert every rule ID, field path, profile version, scheme separation,
and provisional status. Synthetic fixture README files explicitly prohibit
interpreting them as copied scheme documents.

## Verification

```text
cargo +stable test --offline --features profiles \
  --test profile_provisional --test profile_framework \
  --test profile_provenance --test sepa_framework \
  --test public_reference --jobs 1
PASS: 17 tests

cargo +stable clippy --offline --test public_reference \
  --features profiles --jobs 1 -- -D warnings
PASS

cargo +stable fmt --all -- --check
PASS

git diff --check
PASS
```

## Remaining authoritative gates

These changes do not close T088, T093, or promote T098–T107 to official
profile support. Exact source bytes, SHA-256 digests, rights/distribution
decisions, and release-bound rule verification remain required for that step.
