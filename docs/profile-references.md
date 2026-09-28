# Profile implementation references

This inventory records public projects that can help discover rule shapes,
fixtures, and adapter boundaries. None replaces an authoritative Swift, EPC,
or network release. Their message models are not imported; generated structs
remain this repository's only canonical ISO 20022 model.

## Reference matrix

| Project | Primary value | Reusable ideas | Boundary |
|---|---|---|---|
| [`phoughton/iso20022-cbpr-ur`](https://github.com/phoughton/iso20022-cbpr-ur) | CBPR+ rule inventory | Year/message registry, rule IDs, XPath inventory, conditional/cross-field combinators, enforced/advisory split, synthetic fixtures | README says AI-assisted and requires official SWIFT verification |
| [`sebastienrousseau/pain001`](https://github.com/sebastienrousseau/pain001) | SEPA client-payment validation | SCT/SCT Inst composition, per-row diagnostics, IBAN/BIC/currency checks, amount limits, explainable errors | Python row model and scheme claims must be projected onto generated fields and EPC editions |
| [`sebastienrousseau/pacs008`](https://github.com/sebastienrousseau/pacs008) | Multi-network `pacs.008` | Profile adapter shape, schema → profile → audit pipeline, redaction and security patterns | Public network claims do not supply this project's release manifest and digests |
| [`socrates8300/mx20022`](https://github.com/socrates8300/mx20022) | Rust crate boundaries | Generated model / parser / validation / migration split and feature strategy | Public pages do not provide exact scheme source digests |
| [`GoPlasmatic/MXMessage`](https://github.com/GoPlasmatic/MXMessage) | Rust CBPR+ examples | Field/business error taxonomy, SR2025 scenarios, generated model boundary | Public README claims support without the required authoritative source package |
| [`phax/phive`](https://github.com/phax/phive) | Generic validation engine | Validation-pyramid ordering and versioned schema/business-rule adapters | General XML infrastructure; no CBPR+ or EPC rules |
| [`prowide/prowide-iso20022`](https://github.com/prowide/prowide-iso20022) | Broad generated model | Large generated-model organization and family modularity | Scheme validation is in complementary commercial products |

## Consolidated candidate-rule backlog

| Rule family | References | Project destination |
|---|---|---|
| IBAN/BIC/LEI/currency/country | CBPR UR, pain001, pacs008 | `src/helpers`, `src/validation`, provisional profiles |
| BAH ↔ Document identity | CBPR UR, Swift handbook examples | generated metadata binding + profile rule |
| Header message-id consistency | CBPR UR, pacs008 | generated binding + profile rule |
| Conditional required/mutual exclusion | CBPR UR, pain001 | `src/validation/cross_field.rs` |
| Agent/account relationships | CBPR UR, generated bindings | `src/validation/bindings.rs` |
| Currency/amount/total relationships | CBPR UR, pain001, pacs008 | reusable cross-field rules |
| Address constraints | CBPR UR, pain001, pacs008 | generated-field projection with profile version |
| Remittance length/character sets | CBPR UR, pain001 | reusable length/charset rules |
| SCT currency and instant ceiling | pain001 | SEPA release-bound rules only |
| Duplicate-payment detection | pain001 | optional batch validation, not a message model |
| Schema/Schematron ordering | phive | validation adapter if Schematron is supported |

## Merge policy

1. Extract a candidate descriptor and field path from a public reference.
2. Re-express it over generated-struct projections and the shared `Rule`
   abstraction; never copy a second message DTO.
3. Add valid, invalid, boundary, and unsupported-version fixtures, plus a
   differential comparison where the reference is executable.
4. Keep it `provisional` until exact scheme release, source access, digest,
   effective range, and distribution decision are recorded. Only then may it
   enter the authoritative `ProfileRegistry`.

This preserves implementation progress without confusing a GitHub project,
public summary, or inferred rule with an official network acceptance test.
