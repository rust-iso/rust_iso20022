# Roadmap

The roadmap follows the Spec Kit work packages in
`specs/001-production-sdk/tasks.md`. Checked code alone is not a completed
capability; acceptance requires tests, documentation, evidence, and common
gates.

## Verified foundations

- Canonical generated structs with reproducible schema/codegen manifests.
- Message abstraction, schema-derived catalogue/metadata, and version diff.
- Structured L2 validation, financial helpers, and cross-field rules.
- Initial pacs/pain/camt builders and MT103/MT202/MT940 migrations.
- Thin CLI, local MCP, WASM slice, security/privacy limits, and benchmarks.
- Documentation and an unpublished release-candidate process.

## Active standards work

- CBPR+ SR2026: source identity/framework recorded; complete licensed Usage
  Guideline bytes, digests, field rules, and fixtures remain required.
- SEPA SCT/SCT Inst 2025: official release identities recorded; exact source
  files/digests and executable field-level rule packs remain required.
- Full L1 XSD validation remains gated on a verified secure backend.
- The initial coverage-guided fuzz run remains gated on cargo-fuzz/libFuzzer
  availability; deterministic regression corpora already run offline.

## Subsequent work

- Complete CBPR+, SEPA SCT, and SEPA SCT Inst rule packs before adding FedNow,
  Fedwire, T2, TIPS, CHAPS, or HVPS+.
- Extend generated validation bindings and builders only through measured,
  message-specific vertical slices.
- Add more migration mappings with explicit loss reports; never claim lossless
  conversion without field-by-field evidence.
- Publish only after all mandatory work packages and the release baseline pass.

See [docs/status.md](docs/status.md) for the exact support matrix and known
limitations. Sponsorship supports this open roadmap; it does not gate core
functionality.

