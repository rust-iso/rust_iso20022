# SEPA profile source framework

The SDK identifies SEPA releases from separate rulebook and implementation-
guideline documents. It never treats a year such as “SEPA 2025” as a complete
release identity and never aliases SCT Inst to SCT.

## Publicly identified effective baseline

| Scheme | Rulebook | Guidelines | Effective range |
|---|---|---|---|
| SCT | EPC125-05, 2025 v1.1 | EPC132-08 C2PSP 2025 v1.0; EPC115-06 Inter-PSP 2025 v1.0 | 2025-10-05 through 2027-11-21 |
| SCT Inst | EPC004-16, 2025 v1.1 | EPC121-16 C2PSP 2025 v1.0; EPC122-16 Inter-PSP 2025 v1.0 | 2025-10-05 through 2027-11-21 |

The authoritative index pages are maintained by the European Payments Council:

- [SCT rulebook and implementation guidelines](https://www.europeanpaymentscouncil.eu/what-we-do/epc-payment-schemes/sepa-credit-transfer-sct/sepa-credit-transfer-rulebook-and)
- [SCT Inst rulebook and implementation guidelines](https://www.europeanpaymentscouncil.eu/what-we-do/epc-payment-schemes/sepa-instant-credit-transfer/sepa-instant-credit-transfer-rulebook)

The implementation guidelines use the 2019 ISO 20022 message generation. The
2025 rulebook v1.1 changed the unstructured-address cutoff to 15 November 2026;
the EPC states that it made no other business or operational rule change from
v1.0.

SEPA implementation references and the merged candidate-rule backlog are
tracked in [`docs/profile-references.md`](../profile-references.md). Public
implementations help with fixture and API design; they do not replace EPC
rulebook and guideline evidence.

## Consultation boundary

EPC008-26 (SCT) and EPC009-26 (SCT Inst) are 2026 change-request consultation
documents. They describe possible changes for the next release and are rejected
by the SDK as effective profile identities. They must not silently modify the
2025 release.

## Current implementation status

`profiles::sepa` implements source roles, publication status, exact scheme
identity, effective-release checks, SCT/SCT Inst separation, and an explicit
production-readiness gate. The public metadata is recorded in
`profiles/sepa/source-review.json`.

`profiles::provisional::sepa_sct_provisional_rules()` additionally exposes
opt-in IBAN and ISO 4217 syntax checks for application development. It is
versioned as `provisional`, is not registered as EPC 2025, and does not claim
SEPA scheme acceptance.

The execution environment could verify the official document identities and
dates but could not download the original PDF/ZIP bytes. Their SHA-256 fields
therefore remain null. Until exact source bytes are pinned:

- no SEPA L3 rule set is registered as available;
- the SDK does not claim complete SCT or SCT Inst validation;
- WP-018 remains incomplete at T093;
- WP-019 and WP-020 remain gated.

GitHub projects such as `hupe1980/sepa` are architecture, algorithm, and
differential-test references only. Their message models are not imported, and
the generated structs in this repository remain the sole canonical ISO 20022
model.

## Compliance meaning

Future reports mean only “valid according to the implemented rules and exact
identified release.” They do not guarantee acceptance by a bank or clearing
mechanism, regulatory certification, network onboarding approval, or legal
compliance.
