# CBPR+ profile status

## Identified release

The public [Swift Standards Releases](https://www.swift.com/standards/standards-releases)
timeline identifies the final November 2026 CBPR+ Usage Guidelines and CBPR+
UHB SR2026 final edition as published on 20 February 2026. Swift subsequently
revised the SR2026 schedule; the public page currently identifies 12 June 2027
as the live date.

This metadata is recorded in `profiles/cbpr_plus/source-review.json`. The
complete field-level Usage Guidelines are distributed through Swift channels
and were not available as raw public source bytes in the implementation
environment.

The consolidated public-reference inventory is maintained in
[`docs/profile-references.md`](../profile-references.md). It is used for rule
discovery and differential tests, not as a substitute for the Swift source.

## Support matrix

| Release | Message versions | Executable rules | Status |
|---|---|---:|---|
| CBPR+ SR2026 | none claimed | 0 | unavailable pending authoritative source bytes and rule verification |

## Provisional developer checks

`profiles::provisional::cbpr_plus_provisional_rules()` provides opt-in checks
for IBAN, BIC, and ISO 4217 currency syntax. These run through the shared
validation target and never enter the authoritative release registry. Their
version is explicitly `provisional`; they are useful for early integration,
not evidence of CBPR+ compliance.

The profile framework can represent an unavailable exact release, but it does
not silently substitute ISO L2 validators or rules inferred from another year.

## GitHub references

`phoughton/iso20022-cbpr-ur` demonstrates year-specific registries, reusable
rule combinators, enforced versus advisory outcomes, and synthetic fixtures.
Its own README says that it is AI-generated and must be verified against
official SWIFT specifications. `mx20022` and `MXMessage` provide additional
architecture and differential-test references. Their code licenses do not make
them authoritative CBPR+ sources.

No reference implementation's message model is imported. Any future rule must
operate on this project's generated structs, carry an exact source reference,
and have positive, negative, boundary, and unsupported-version evidence.

## Compliance meaning

When a rule pack becomes available, “valid” will mean valid according to its
implemented rules and exact release. It will not guarantee correspondent-bank
acceptance, Swift onboarding, certification, regulatory approval, or legal
compliance.
