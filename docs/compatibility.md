# Compatibility Policy and Baseline

This document defines the compatibility baseline for the additive production SDK
work. It distinguishes the published `0.1.1` crate from the current unreleased
working tree (`0.1.2`). It is not a promise that every pre-1.0 release can never
change; it is the evidence and approval process required before a change is made.

## Protected Surface

Unless an explicitly approved breaking-change work package says otherwise, the
following remain compatible:

- root reexports and public core types/functions;
- all 32 `model-<area>` feature names and aggregate `model`;
- existing generated module/type paths;
- XML and enabled JSON serialization behavior;
- message detection, `MxId`, metadata, envelope, catalogue, and AppHdr behavior;
- current `serde`, `convert`, `catalogue`, and `cli` feature meanings;
- the documented WASM functions and npm-visible names.

Generated structs remain the sole canonical ISO 20022 model. Additive wrappers,
builders, validators, profiles, migrations, and adapters must preserve direct
access to the generated value.

## Published and Working Baselines

The automated baseline under `evidence/compatibility/` contains:

- a simplified default public API snapshot and a
  `model-pacs,serde,convert` snapshot;
- a `cargo-public-api` diff against crates.io `0.1.1`;
- `cargo-semver-checks` results against crates.io `0.1.1`;
- published package metadata and the complete current feature graph;
- downstream compile fixtures for the root API and one public generated path
  from every business-area feature;
- exact core/Serde behavior goldens and an XML byte-length/FNV-1a wire
  fingerprint;
- the npm/WASM export inventory and runtime smoke cases.

Regenerate the machine evidence with:

```console
scripts/check-public-api.sh
scripts/check-generated-paths.sh
scripts/check-wasm-compat.sh --build
cargo test --test compatibility_baseline --features serde,model-pacs
```

The generated-area check is intentionally per area. Enabling all 1,130 generated
modules in one crate is not a valid substitute: it has a much larger memory peak
and fails to demonstrate that each individual feature is self-contained.

## High-Risk Existing Public Types

Several existing types cannot be casually extended in an additive release:

- `Error` and `BusinessArea` are public exhaustive enums. Adding a variant breaks
  downstream exhaustive matches.
- `MxId` and `CatalogueEntry` expose public fields. Adding a required field
  breaks downstream struct literals.
- `MxMessage` is publicly implementable and has required associated constants.
  Adding a required item breaks external implementations.
- `AnyMessage` variants are feature-dependent and public. Dispatch evolution
  must be generated and compatibility-tested.

New required semantics therefore belong in new typed errors, descriptors,
reports, wrappers, or extension traits. Existing compatibility types may delegate
to the new core only when observable behavior remains equivalent.

## Schema-Driven Versus Hand-Written Change

Every release note separates:

1. schema-originated changes caused by a recorded XSD set;
2. generator/configuration changes affecting generated Rust or metadata; and
3. hand-written API or behavior changes.

A generated diff without schema/generator/configuration provenance is not
acceptable. Generated files are never manually patched; the generator is fixed
and the tree is regenerated.

## Feature and Toolchain Baseline

- Edition: Rust 2024.
- Declared MSRV: Rust 1.85.
- License: Apache-2.0. A dual-license change requires copyright-holder
  authorization and is not implied by this roadmap.
- Default features remain empty.
- XML/yaserde is currently part of the existing ABI and is not declared optional.
- Model families are selected through the 32 public `model-<area>` features.
- `model-head` is header-only; message auto-dispatch is compiled only when a
  generated `Document` family is selected.
- CLI, MCP, profiles, migrations, fuzzing, benchmarks, and browser-specific
  packaging must not all enter the default feature set.

An MSRV increase requires CI evidence, a changelog entry, release impact, and
migration guidance. Edition alone is not a reason to raise MSRV.

## Breaking-Change Approval

A proposed breaking change must not be merged or released until its WP records:

- the exact public item, feature, wire fixture, or adapter contract affected;
- why an additive alternative is insufficient;
- downstream impact and migration instructions;
- whether the change is schema-originated or hand-written;
- `cargo-public-api`, `cargo-semver-checks`, compile-fixture, and behavior-golden
  results;
- the required SemVer release impact and explicit maintainer approval.

No exception may be hidden by updating a golden without explaining the change.
The project is pre-1.0, but financial integration stability still requires this
review discipline.

## Known Baseline Limitations

- The pre-existing generated tree is not rustfmt-clean under Rust 1.96.
  Formatting must be corrected through deterministic code generation and
  regeneration, not manual generated-file edits.
- `yaserde`/`yaserde_derive` remain pinned to the 0.7 behavior used by the
  generated model. An upgrade needs full regeneration and wire revalidation.
- The current XML detector/scanner has not yet passed the planned bounded-parser
  security work; baseline compatibility does not certify it as safe for
  untrusted input.
- Passing implemented validation rules never guarantees bank acceptance,
  regulatory certification, network onboarding, or legal compliance.
