# Implementation Start

- Feature directory: `specs/001-production-sdk`
- Workflow: Specification → Implementation Plan → Work Packages →
  Implementation → Verification → Evidence → Release Baseline
- Started: 2026-09-27 (Asia/Taipei)
- Development toolchain observed before implementation: Rust 1.96
- Declared crate MSRV: Rust 1.85
- Canonical-model invariant: generated schema structs remain the sole canonical
  ISO 20022 message model.
- Publication constraint: this implementation effort must not publish, push,
  tag, or create a release without separate user authorization.

## Pre-existing Working-Tree Scope

The implementation began in an intentionally dirty working tree. Existing
changes include the docs.rs/package metadata fix, README/documentation work,
Ko-fi funding metadata, Rust 2024 conversion, Spec Kit initialization, and all
feature artifacts under `specs/001-production-sdk/`. These changes are
user-directed and must not be discarded or overwritten.

## Active External Gates

- `R-XSD-RUST`: full L1 XSD backend evidence.
- `R-PARSER`: one bounded XML gate and encoding/entry-point evidence.
- `R-LEGAL-CBPR`: authoritative CBPR+ source/access/distribution evidence.
- `R-LEGAL-SEPA`: SEPA-derived rule-data licensing evidence.

The gates block only their affected claims. Safe, authorized, independently
verifiable work continues.

