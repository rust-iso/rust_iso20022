# Implementation Plan: Production-Grade ISO 20022 SDK

**Branch**: `001-production-sdk` | **Date**: 2026-09-27 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/001-production-sdk/spec.md`

## Summary

Evolve the existing generated-model/codegen library into a production-grade SDK
without replacing its generated structs. First freeze compatibility and make
schema generation reproducible; then derive metadata, dispatch, and catalogue
from the same source. Build bounded parsing, structured validation, typed helpers
and builders, versioned profiles, version comparison, migration, and thin
CLI/MCP/WASM adapters on that foundation. Every work package closes with tests,
documentation, recorded evidence, and the common quality gates.

## Technical Context

**Language/Version**: Rust 2024 Edition; declared MSRV Rust 1.85; development stable Rust 1.96; nightly only for cargo-fuzz  
**Primary Dependencies**: yaserde/yaserde_derive 0.7 compatibility layer, xml-rs 0.8, serde/serde_json, phf; pinned xsd-parser for codegen; dependency additions selected per WP after MSRV/license/security review  
**Storage**: Version-controlled XSDs, manifests, generated Rust/metadata, fixtures, snapshots, benchmark/fuzz evidence; no runtime database or network dependency  
**Testing**: cargo test, doctests, compile fixtures, golden/snapshot and regeneration tests, cargo-semver-checks/public API comparison, wasm target tests, cargo-fuzz, Criterion-style benchmarks, CLI/MCP contract smoke tests  
**Target Platform**: Rust library on supported stable/MSRV platforms; native CLI/MCP binaries; wasm32 subset documented by feature matrix  
**Project Type**: Rust workspace containing a library façade/core SDK, generated model, code generator, and separately packaged adapters/tools  
**Performance Goals**: Establish reproducible baselines for pacs.008 parse/serialize/detect/validate, camt.053 parse, and catalogue lookup before setting regression budgets; no optimization claim without like-for-like measurements  
**Constraints**: Generated structs remain sole canonical model; minimal default features; offline deterministic operation; bounded untrusted input; no raw financial payload logging; no breaking API without explicit SemVer process; no publish during implementation  
**Scale/Scope**: 1,130 schema/message versions across 32 generated areas; phase-one builders for 8 message families, 3 profile families, 3 MT migrations, CLI/MCP/WASM surfaces, security/fuzz/performance/release evidence

## Constitution Check

### Pre-Research Gate

| Principle | Plan compliance | Result |
|---|---|---|
| I. One canonical model | Builders, validation, profiles, migration, and adapters all consume or produce generated structs; schema metadata is descriptive only. | PASS |
| II. Reproducible lineage | Pinned generator, schema/generator manifests, hashes, transactional generation, and three-run regeneration tests are planned before metadata consumers. | PASS |
| III. Compatibility/SemVer | WP-001 freezes published and working-tree APIs, wire fixtures, features, detection, metadata, and WASM exports before architectural changes. | PASS |
| IV. Additive layers/thin adapters | Root core owns behavior; separate CLI/MCP/WASM/migration packages call core contracts. | PASS |
| V. Explainable validation | Three layers, stable rule descriptors, versioned profile keys, deterministic reports, and explain lookup are first-class contracts. | PASS |
| VI. Privacy/security | One bounded XML gate and one redaction policy serve all entry points; MCP lacks network/shell/file capabilities. | PASS |
| VII. Evidence quality | Matrix fixtures, mapping classifications, fuzz targets, and measurement context are required acceptance evidence. | PASS |
| VIII. Feature/MSRV/performance | Minimal defaults and current aliases are preserved; feature/target/MSRV matrix and baselines are explicit. | PASS |
| IX. Spec Kit/DoD | Tasks will be concrete WPs with all mandatory sections and common verification gates. | PASS |

### Post-Design Gate

The Phase 1 data model and contracts preserve all nine principles. No second
message model is introduced; `SchemaDescriptor` and migration IR are explicitly
non-canonical and bounded in lifetime. Four research gates remain (`R-XSD-RUST`,
`R-PARSER`, `R-LEGAL-CBPR`, `R-LEGAL-SEPA`), but each gates only the affected
claim. They do not justify placeholders or suspend other clear work. **PASS**.

## Architecture and Dependency Direction

```text
versioned XSD bundle + generator config
                 │
                 ▼
       pinned deterministic codegen
          ┌──────┴────────┐
          ▼               ▼
 generated structs   schema metadata
  (canonical model)       │
          │          descriptors/catalogue/diff
          ├───────────────┤
          ▼               ▼
 message abstraction, validation, profiles, builders
          │
          ├──────── migration (returns generated structs + report)
          │
          └──────── CLI / MCP / WASM thin adapters
```

No dependency points from generated code toward a profile, adapter, builder, or
migration crate. Adapter crates may depend on core; core never depends on them.

## Project Structure

### Documentation for this feature

```text
specs/001-production-sdk/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── library-api.md
│   ├── manifests.md
│   ├── validation-profiles.md
│   ├── adapters.md
│   └── migration-versioning-security.md
├── checklists/
│   └── requirements.md
└── tasks.md
```

### Intended repository layout

```text
src/
├── generated/              # sole canonical ISO 20022 model
├── core/                   # compatibility façade and bounded message API
├── metadata/               # generated descriptors and lookup indices
├── validation/             # report, registry, L1/L2 rule engine
├── profiles/               # versioned rule sets over generated types
├── builders/               # typed construction returning generated Documents
├── helpers/                # Money/Currency/Iban/Bic/etc.; no message DTOs
├── compare/                # schema-metadata version comparison
├── privacy/                # redaction policy shared by every adapter
└── lib.rs

tools/codegen/               # pinned deterministic generator
crates/
├── migration/               # bounded MT parsing/conversion + reports
├── cli/                     # thin native command adapter
├── mcp/                     # local stdio MCP adapter
├── wasm/                    # thin wasm-bindgen adapter and compatibility shims
└── benchmarks/              # reproducible runtime/size/compile baselines

xsds/                        # schema set, manifest, source files
fixtures/
├── iso/{valid,invalid}/
├── cbpr_plus/{valid,invalid}/
├── sepa/{sct,sct_inst}/{valid,invalid}/
└── migration/{mt103,mt202,mt940}/

tests/
├── compatibility/
├── contracts/
├── integration/
├── security/
└── matrix/

fuzz/                        # separate nightly cargo-fuzz workspace
benches/ or crates/benchmarks/
evidence/                    # command results, manifests, matrices, baselines
```

**Structure Decision**: The existing root package remains the stable library and
generated-model owner. New workspace crates exist only where platform or release
boundaries matter. Direct generated-model capabilities remain root modules so
there is one implementation path and no cyclic dependency.

## Feature Strategy

- Preserve `default = []`, every `model-*` feature, aggregate `model`, `serde`,
  `convert`, `catalogue`, and legacy `cli` behavior during the compatibility
  window.
- Introduce `json` as the public JSON capability while retaining `serde` as an
  alias for at least one release cycle; add `validation`, `builder-pacs`,
  `builder-pain`, `builder-camt`, `builders`, `profiles`, `cbpr-plus`, `sepa-sct`,
  `sepa-sct-inst`, `sepa`, `version-compare`, and `schema-fetch` only as needed.
- Keep CLI, MCP, migration, fuzz, benchmark, and WASM packaging outside default
  core. Do not make XML optional until the existing unconditional yaserde ABI is
  deliberately migrated and compatibility-tested.
- Every supported and unsupported feature/target pair appears in the feature
  matrix and CI. Unsupported pairs fail clearly or are documented before build.

## Delivery Sequence

1. **Baseline and provenance**: WP-001; WP-002A–E. Freeze compatibility, pin all
   codegen inputs, add manifests, make generation strict/transactional, and prove
   byte-identical regeneration.
2. **Metadata and message core**: WP-003–005 and WP-021. Emit schema descriptors,
   add generated-value wrappers, catalogue indices, field lookup, and semantic
   version comparison.
3. **Safe parsing and validation core**: WP-006–008 plus parser/licensing spikes.
   Establish bounded XML, typed reports, helper validators, reusable cross-field
   rules, redaction, and explanation registry.
4. **Ergonomic construction**: WP-009–011 expanded to all eight required phase-one
   builders, each returning generated `Document` values.
5. **Thin native tooling**: WP-012–015 delivers detect/inspect/validate/explain,
   then catalogue/version/serialization commands and stable JSON/exit contracts.
6. **Profiles**: WP-016–020. Implement append-only release framework first;
   accept profile claims only after source-rights gates and fixture evidence.
7. **Migration and adapters**: WP-022–027. Add bounded MT parsing, three mappings,
   MCP, and verified WASM subset without duplicating core behavior.
8. **Hardening and release**: WP-028–033. Complete attack corpus, fuzzing,
   benchmark/compile/size baselines, documentation, release/supply-chain
   automation, and sponsorship metadata; run the full release baseline.

## Verification Strategy

- Each WP records exact commands, selected features/targets, fixture IDs,
  results, and limitations under `evidence/`.
- Common gate: `cargo fmt --check`, applicable clippy with warnings denied,
  focused tests, `cargo test --workspace` with required features, doctests,
  `git diff --check`, and a scan for active-WP placeholders/panic paths.
- Compatibility gate compares API snapshots, downstream compile fixtures,
  serialized goldens, detection/metadata snapshots, feature aliases, and WASM
  exports.
- Codegen gate performs clean generation three times, compares bytes, validates
  manifest/output bidirectionality, detects tampering/stale files, and confirms a
  tracked-tree regeneration diff of zero.
- Validation/profile/migration gates use the declared multidimensional fixture
  matrix with expected rule IDs, paths, profile releases, mapping classes,
  warnings, and unmapped fields.
- Security gate exercises size/depth/event/collection limits, prohibited XML
  constructs, malformed Unicode/namespaces, canary redaction, fuzz regressions,
  and panic-free behavior.
- Release gate runs all commands named in the specification, CLI/MCP/WASM smoke
  contracts, fuzz and benchmark baselines, dependency/license/audit checks,
  docs.rs-equivalent build, checksums, and SBOM evaluation.

## External Gates and Honest Scope

| Gate | Owner/output | Work that may continue | Claim blocked while open |
|---|---|---|---|
| R-XSD-RUST | XSD backend spike and decision record | report types, L2/L3 engine, bounded parser | complete L1 XSD validation |
| R-PARSER | shared gate prototype, encoding/entry-point matrix | metadata, helpers, builders | hardened XML acceptance |
| R-LEGAL-CBPR | authoritative source/access/license evidence | versioned profile framework | complete CBPR+ production rules |
| R-LEGAL-SEPA | derived-data license review | profile framework and private test design | published complete SEPA rule database |

If a gate cannot be resolved with repository authority, its affected WP remains
open with evidence and exact remaining work. No synthetic substitute may be
reported as standards coverage.

## Complexity Tracking

| Intentional complexity | Why needed | Simpler alternative rejected because |
|---|---|---|
| Separate migration, CLI, MCP, WASM, fuzz, and benchmark packages/workspaces | Platform, dependency, release, and MSRV boundaries must not inflate the core/default build | One package/default feature graph would compile unrelated adapters and nightly tooling for every user |
| Schema descriptor IR | Catalogue, field lookup, stable validation paths, and semantic diff need one schema-derived source | Hand-maintained lists drift; a second runtime message model violates Principle I |
| Compatibility façade beside v2 typed APIs | Existing exhaustive types/traits cannot safely absorb required fields | Directly mutating them is a breaking change before an approved SemVer-major release |
