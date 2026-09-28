<!--
Sync Impact Report
- Version change: template → 1.0.0
- Added principles:
  - I. One Canonical ISO 20022 Model
  - II. Reproducible Schema-to-Code Lineage
  - III. Compatibility and SemVer Discipline
  - IV. Additive SDK Layers and Thin Adapters
  - V. Structured, Explainable, Versioned Validation
  - VI. Privacy, Security, and Bounded Resource Use
  - VII. Evidence-Driven Quality
  - VIII. Feature, MSRV, and Performance Discipline
  - IX. Spec Kit Work Packages and Truthful Completion
- Added sections:
  - Architecture and Product Constraints
  - Development Workflow and Quality Gates
- Removed sections: none
- Deferred TODOs: none
-->
# rust_iso20022 Constitution

## Core Principles

### I. One Canonical ISO 20022 Model

The Rust structs generated from official ISO 20022 schemas MUST remain the only
canonical ISO 20022 data model. Builders MUST return generated structs.
Validation MUST operate on generated structs directly or through a common
abstraction over them. Profiles MUST NOT own a parallel message model, and
migrations MUST produce generated structs. Helper values, builder inputs,
validation contexts, and transient migration representations MAY exist only for
ergonomics, validation, conversion, or reporting; they MUST NOT become a second
long-lived schema representation. Generated paths and direct access to generated
types MUST remain available to SDK users.

Rationale: two canonical models inevitably drift and make schema fidelity,
serialization behavior, compatibility, and maintenance unverifiable.

### II. Reproducible Schema-to-Code Lineage

Generated output MUST derive only from declared schema inputs, a pinned generator
version or commit, and version-controlled generator configuration. The repository
MUST record schema manifests, cryptographic schema hashes, generator manifests,
and generation metadata sufficient to trace every generated type to its schema
and release. Re-running generation with identical inputs MUST produce identical
output. Generated files MUST NOT receive manual corrections; defects MUST be
fixed in schemas, generator transforms, or configuration and then regenerated.
The generator MUST have unit, snapshot, and regeneration tests covering
namespaces, documentation, enums, optional and repeated fields, and metadata.

Rationale: financial message types are trustworthy only when their origin and
generation process are independently reproducible.

### III. Compatibility and SemVer Discipline

Existing generated module paths, serialization behavior, detection, metadata,
feature flags, and supported WASM API form the compatibility baseline. Public API
changes MUST be compared against that baseline. Breaking changes require explicit
approval, a SemVer-major release, changelog justification, and migration notes.
Schema-driven structural changes MUST be recorded separately from hand-written
API changes. MSRV changes MUST be deliberate, tested in CI, and documented in the
changelog. Additive high-level APIs MUST NOT make low-level generated APIs less
capable.

Rationale: downstream financial integrations value predictable upgrades and
reproducible historical behavior more than convenience-driven churn.

### IV. Additive SDK Layers and Thin Adapters

The required dependency direction is official schema → codegen → generated
struct → high-level SDK capabilities. Message abstraction, metadata, catalogue,
builders, validation, profiles, and migration MUST build on generated structs.
CLI, MCP, and WASM MUST be thin adapters over the core SDK and MUST NOT reimplement
parsing, detection, validation, conversion, redaction, or catalogue logic. CLI and
MCP MUST remain separately feature-gated or separately packaged so they do not
inflate the default library. Adapter outputs MUST provide stable machine-readable
forms where required, including stable CLI exit codes.

Rationale: a single implementation path prevents inconsistent results across
Rust, command-line, MCP, and browser consumers.

### V. Structured, Explainable, Versioned Validation

Validation MUST distinguish L1 syntax/schema checks, L2 ISO 20022 semantic rules,
and L3 market or network profile rules. Every run MUST produce a structured
`ValidationReport` containing validity, errors, and warnings. Every issue MUST
contain a stable code, severity, field path, message, and rule ID. Profile issues
MUST additionally identify profile, profile release, and source or reference.
Important rules MUST be explainable by rule ID and traceable to affected message,
field path, reason, and source. Profiles and their supported messages MUST be
explicitly release-versioned; historical releases MUST remain reproducible.
Cross-field behavior MUST use reusable rule abstractions rather than scattered
message-specific conditionals. Validation MUST NOT collapse results to a boolean
or untyped string and MUST NOT panic.

Rationale: financial institutions need auditability and must know exactly which
implemented rule produced a result; “valid” never implies bank acceptance,
regulatory certification, network onboarding, or legal compliance.

### VI. Privacy, Security, and Bounded Resource Use

Raw financial messages and sensitive values such as account numbers, IBANs,
names, addresses, references, and payment details MUST NOT be logged by default,
including debug logging. Shared redaction facilities MUST be used by all adapters.
Full-message logging requires an explicit unsafe opt-in. Parsers, detectors,
validators, and migration parsers MUST enforce documented size, depth, collection,
and allocation limits appropriate to untrusted XML. Entity expansion, malformed
namespaces and Unicode, excessive nesting, and oversized collections MUST fail
without panic. MCP MUST default to local processing with no network access, shell
execution, or message upload.

Rationale: payment data is sensitive and adversarial input must not turn parser
convenience into disclosure or denial-of-service risk.

### VII. Evidence-Driven Quality

Every behavior MUST be demonstrated by tests that prove domain outcomes, not
merely that a function returns success. Financial field validators require valid,
invalid, boundary, and appropriate fuzz coverage. Profiles and migrations require
versioned fixtures and expected rule IDs, field paths, mapping classifications,
warnings, and unmapped fields. Migration MUST classify every mapping as Exact,
Derived, Lossy, Ambiguous, or Unsupported and MUST never silently discard data.
Security-sensitive parsers require fuzz targets with panic-free and bounded-use
acceptance criteria. Performance work MUST start from recorded benchmarks for the
defined parsing, serialization, detection, validation, and catalogue workloads.

Rationale: claims about standards, safety, compatibility, migration fidelity, or
performance are acceptable only when backed by reproducible evidence.

### VIII. Feature, MSRV, and Performance Discipline

Default features MUST remain minimal. XML, JSON, validation, profiles, individual
profile families, WASM, CLI, and MCP capabilities MUST be separable according to
their dependency and platform needs. Unsupported feature/target combinations MUST
fail early through configuration or be explicitly documented and tested. CI MUST
cover the declared MSRV and supported stable toolchain. Baselines MUST record
default compile time, dependency count, binary size, and representative runtime
benchmarks. Optimization MUST be justified by measurement; convenience MUST NOT
pull all profiles or adapters into the default build.

Rationale: the generated model is already large, so compile cost and target
compatibility are product requirements rather than incidental concerns.

### IX. Spec Kit Work Packages and Truthful Completion

All product work MUST follow Specification → Implementation Plan → Work Packages
→ Implementation → Verification → Evidence → Release Baseline. Every WP MUST
state Goal, Scope, Non-goals, Dependencies, Implementation, Tests, Evidence, and
objective Acceptance Criteria. A WP is Done only when implementation, tests,
documentation, applicable runnable examples, formatting, linting, workspace tests,
diff checks, and recorded acceptance evidence all pass. Interfaces, modules,
TODOs, mocked adapters, always-valid rules, happy-path-only behavior, or silent
field skipping are not completion. Stage reports MUST NOT be treated as delivery
while safe, authorized, clear, in-scope work remains.

Rationale: explicit, testable packages prevent a broad SDK roadmap from devolving
into unverifiable scaffolding or false completion.

## Architecture and Product Constraints

- Phase-one profiles are CBPR+, SEPA SCT, and SEPA SCT Inst, each bound to an
  explicit release and backed by supported-message metadata, rules, fixtures,
  references, and evidence. Later profile families MUST use the same abstraction.
- Initial builders cover pacs.008, pacs.009, pacs.002, pain.001, pain.002,
  camt.052, camt.053, and camt.054. Each builder MUST return its generated type,
  use typed helper values and typed errors, and validate construction invariants.
- Initial migration covers MT103 → pacs.008, MT202 → pacs.009, and MT940 →
  camt.053 with field-level mapping reports and fixtures. No conversion may claim
  losslessness without evidence.
- The catalogue MUST be generated from the same schema/codegen metadata as the
  models and support identifier, family, version, latest, namespace, and root
  lookups without a separately maintained hardcoded message list.
- Version comparison MUST operate on schema metadata and report field, type,
  cardinality, and enum changes; XML text diff is insufficient.
- Public library failures MUST use domain-specific typed errors where the domain
  can be modeled. `anyhow` MAY be used only at outer adapter boundaries.
- Documentation MUST cover generated and high-level APIs, validation, profiles,
  CLI, migration, WASM, MCP, limitations, and the compliance disclaimer. Every
  public module requires module-level documentation and core APIs require runnable
  examples.
- Open-source core capabilities MUST remain open. Sponsorship supports standards
  updates, profile releases, migration, security, fuzzing, documentation, and
  maintenance; it MUST NOT gate core behavior behind private forks.

## Development Workflow and Quality Gates

1. Repository baseline work MUST inventory current public APIs, features,
   generated paths, serialization fixtures, build behavior, supported targets,
   schema inputs, dependencies, and known limitations before architectural work.
2. Specifications MUST express testable requirements, compliance disclaimers,
   and compatibility constraints without prematurely dictating implementation.
3. Plans MUST preserve dependency direction and identify research decisions,
   data contracts, feature boundaries, security limits, and release risks.
4. Work packages MUST be independently reviewable and ordered by dependency.
   Broad labels such as “improve validation” are prohibited.
5. Implementation MUST remain within the active WP. Newly discovered work MUST
   be appended to the task set through convergence rather than hidden in scope.
6. Verification evidence MUST record exact commands, fixture identities, results,
   benchmark context, supported versions, and any unresolved limitation.
7. Every WP acceptance gate includes, where applicable:
   `cargo fmt --check`, `cargo clippy --workspace`, `cargo test --workspace`,
   focused feature/target checks, and `git diff --check`.
8. The release baseline additionally requires workspace build/test/clippy/format,
   CLI smoke tests, WASM checks, profile and migration fixtures, fuzz and benchmark
   baselines, supported-message/profile/migration records, MSRV, feature matrix,
   known limitations, changelog, checksums, and supply-chain checks.
9. Release engineering MUST cover SemVer, GitHub Release, crates.io, docs.rs, CLI
   binaries, checksums, and practical SBOM evaluation. Dependency audit and
   license policy checks MUST be maintainable; tooling complexity requires
   explicit justification.

## Governance

This constitution supersedes informal project practices and applies to every
specification, plan, WP, review, adapter, generated artifact, and release.
Amendments require a documented rationale, impact analysis, explicit approval,
and migration or remediation plan when existing work becomes non-compliant.

Constitution versions follow SemVer: MAJOR for incompatible principle removal or
redefinition, MINOR for new principles or materially expanded obligations, and
PATCH for non-semantic clarification. Each amendment updates the Sync Impact
Report and the Last Amended date.

Every plan and WP MUST include a constitution check before implementation and a
post-implementation compliance check. Deviations MUST be recorded with owner,
scope, rationale, evidence, expiry or review date, and remediation plan; silent
exceptions are prohibited. Release approval requires all mandatory gates to pass
or a documented external blocker that names what remains and why it cannot be
completed safely within the authorized scope.

**Version**: 1.0.0 | **Ratified**: 2026-09-27 | **Last Amended**: 2026-09-27
