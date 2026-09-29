# SEPA SCT Inst public-reference profile

`sepa_sct_inst_public_reference()` provides opt-in developer checks for the
public SCT Inst 2025-v1.1 metadata baseline. It is deliberately separate from
SCT and currently covers ISO 13616 IBAN syntax, EUR settlement, and the
public-reference EUR 100,000 amount boundary candidate.

All rules are labelled `SEPA-SCT-INST-PROVISIONAL-*`. They are useful for
development and CI, but they are not a complete EPC implementation and do not
guarantee scheme acceptance. The exact EPC source artifacts and digests remain
required before authoritative release registration.
