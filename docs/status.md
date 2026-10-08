# Implementation status

This page distinguishes implemented code from verified production coverage.
Passing an implemented rule never promises bank acceptance, certification,
network onboarding, regulatory approval, or legal compliance.

## Published release and verification

As of 2026-10-08, the core SDK is published as **0.1.6** on
[crates.io](https://crates.io/crates/rust_iso20022/0.1.6) and has a
[v0.1.6 GitHub release](https://github.com/rust-iso/rust_iso20022/releases/tag/v0.1.6).
The release was prepared on 2026-10-07. Its public SDK API, generated models,
wire formats, feature flags, and Rust 1.85 MSRV are unchanged from 0.1.5;
see the [changelog](../CHANGELOG.md). The optional `convert` dependency floor
and CI/release tooling were corrected.

CLI, migration, WASM, and MCP remain **unpublished workspace crates** built
from this repository. The MCP adapter requires Rust 1.88; the core SDK's
MSRV remains 1.85.

The initial coverage-guided fuzz baseline completed in
[GitHub Actions run 37600066777](https://github.com/rust-iso/rust_iso20022/actions/runs/37600066777):
all eight targets (`xml`, `detect`, `namespace`, `financial`, `mt_parser`,
`mt103`, `mt202`, `mt940`) completed the configured 60-second runs, with no
reported crash, timeout, OOM, or sanitizer leak. This is a bounded baseline,
not proof that all inputs or defects have been covered. Scheduled fuzzing and
persistent regression replay continue through the
[fuzz workflow](../.github/workflows/fuzz.yml).
Per-target execution counts and downloaded-log digests are recorded in the
[fuzz evidence summary](../evidence/ci/37600066777.md).

[Release verification run 37627052320](https://github.com/rust-iso/rust_iso20022/actions/runs/37627052320)
passed all six jobs and verified 28 artifact checksums. The selected source
revision resolved to the full 0.1.6 release commit
`760787ff5b5014d045bc22a974a952e7af1bf057`; its crate archive matched the
published 0.1.6 checksum. These are recorded completed runs, not a live status
claim about subsequent workflow executions. Earlier implementation-stage
evidence and release-baseline snapshots retain their historical context.
See the [release evidence summary](../evidence/ci/37627052320.md) for retained
artifact verification and source comparison.

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
- Coverage-guided fuzzing has a finite per-target budget; it does not establish
  exhaustive parser, migration, or profile-rule correctness.
