# SEPA SCT public-reference profile

`sepa_sct_public_reference()` provides opt-in developer checks for the public
SCT 2025-v1.1 metadata baseline. It currently covers ISO 13616 IBAN syntax,
ISO 4217 currency syntax, and the independently documented EUR settlement
candidate rule.

The bundle is labelled `public-reference` and all rule IDs are versioned
`SEPA-SCT-PROVISIONAL-*`. It is suitable for local validation and CI feedback;
it is not a complete EPC rulebook/implementation-guideline implementation and
does not guarantee scheme acceptance.

Authoritative source identity, exact downloaded bytes, and SHA-256 digests are
tracked separately in `profiles/sepa/source-review.json`. Until those bytes
are pinned, this profile is not registered as a production release.
