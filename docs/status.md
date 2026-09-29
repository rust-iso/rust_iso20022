# Implementation status

This page distinguishes implemented code from verified production coverage.
Passing an implemented rule never promises bank acceptance, certification,
network onboarding, regulatory approval, or legal compliance.

## Implemented and verified

- 1,130 generated message modules across 32 business areas, with per-area
  features and the generated structs retained as the sole canonical model.
- Reproducible schema/codegen manifests, source hashes, generated metadata,
  catalogue lookup, uniform message detection/parsing, and semantic version
  comparison.
- Bounded XML processing, privacy redaction, structured validation reports,
  financial-field validators, and reusable cross-field rules.
- Builders returning generated documents for pacs.002/008/009,
  pain.001/002, and camt.052/053/054.
- Thin CLI commands: detect, inspect, to-json, to-xml, validate, explain,
  catalog, versions, and compare.
- MT103 to pacs.008.001.08, MT202 to pacs.009.001.08, and MT940 to
  camt.053.001.09 conversions with complete mapping reports.
- Seven local stdio MCP tools with no file, URL, network, shell, or upload
  capability.
- WASM detection/catalogue plus a model-pacs parsing, serialization, and L2
  validation slice.
- Security attack corpus, deterministic fuzz-regression corpus, and runtime,
  memory, compile-time, dependency, binary, and WASM-size baselines.
- Opt-in public-reference bundles for CBPR+ SR2026, SEPA SCT 2025-v1.1, and
  SEPA SCT Inst 2025-v1.1. These are provisional developer checks, not
  authoritative scheme rule packs.

## Profile status

The exact-release profile framework is implemented. It rejects year-only
selectors and preserves historical release identity.

| Profile | Source identity | Authoritative L3 rules | Public-reference rules | Status |
|---|---|---:|---:|---|
| CBPR+ SR2026 | Public Swift timeline recorded | 0 | 7 | Complete Usage Guideline bytes/digests unavailable |
| SEPA SCT 2025 v1.1 + IG 2025 v1.0 | Official EPC pages recorded | 0 | 3 | Exact source-file digests pending |
| SEPA SCT Inst 2025 v1.1 + IG 2025 v1.0 | Official EPC pages recorded | 0 | 3 | Exact source-file digests pending |

See [profiles](profiles.md), [CBPR+ status](profiles/cbpr-plus.md), and
[SEPA status](profiles/sepa.md). Consultation documents EPC008-26 and EPC009-26
are not treated as effective releases.

## Known limitations

- Complete offline L1 XSD validation is not implemented; bounded XML syntax,
  namespace, root, and generated deserialization are available.
- The generated model targets YaSerde 0.7 behavior. An upgrade requires a full
  regenerate and compatibility review.
- L2 generated-message binding currently covers pacs.008.001.08; unsupported
  message versions return an explicit unavailable error.
- The scheduled cargo-fuzz configuration and regression replay exist, but the
  initial coverage-guided baseline remains pending because the sandbox could
  not download `cargo-fuzz`/`libfuzzer-sys`.
- No publish, Git tag, GitHub release, or crates.io release is performed by the
  current implementation work.
