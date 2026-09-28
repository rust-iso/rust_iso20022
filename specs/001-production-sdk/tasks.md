# Work Packages: Production-Grade ISO 20022 SDK

**Input**: [spec.md](./spec.md), [plan.md](./plan.md),
[research.md](./research.md), [data-model.md](./data-model.md), and
[contracts/](./contracts/)

**Execution rule**: Within every WP, write the named tests first and observe the
relevant failure before implementation. A WP is Done only after its focused
acceptance checks, documentation/examples, common quality gates, and evidence
record all pass. Interface-only, placeholder, mocked, always-valid, or
happy-path-only work is not Done.

**Common quality gates**: `cargo fmt --check`; applicable
`cargo clippy --workspace --all-targets -- -D warnings`; applicable focused and
workspace tests with explicit features; doctests; `git diff --check`; no new
unjustified panic path or active-WP TODO/placeholder.

## Phase 1: Setup

**Purpose**: Make Spec Kit execution and evidence locations explicit without
changing the product model.

- [X] T001 Create the planned `evidence/`, `fixtures/iso/{valid,invalid}/`, `tests/compatibility/`, `tests/contracts/`, `tests/security/`, and `tests/matrix/` layout with purpose READMEs in each top-level directory
- [X] T002 [P] Add the WP evidence schema and command/result template to `evidence/README.md` and `evidence/template.md`
- [X] T003 [P] Add the test-matrix source document with Message × Version × Serialization × Validation Layer × Profile × Valid/Invalid dimensions to `tests/matrix/matrix.toml`
- [X] T004 Record the initial implementation state, active feature directory, toolchain, dirty-tree ownership, and no-publish constraint in `evidence/implementation-start.md`

---

## Phase 2: Foundational Quality Controls

**Purpose**: Establish shared automation that every independently testable story
uses. These controls do not redefine existing public APIs.

- [X] T005 Add a script that runs common quality gates without hiding skipped feature-gated tests to `scripts/check-work-package.sh`
- [X] T006 [P] Add feature/target/MSRV matrix definitions, including current compatibility feature names and unsupported-pair expectations, to `.github/workflows/ci.yml`
- [X] T007 [P] Add fixture sidecar schema for expected rule IDs, paths, profile keys, mapping classes, and source digests to `fixtures/README.md`
- [X] T008 Verify setup/foundational commands and record results, exact versions, and known gaps in `evidence/WP-000-foundation.md`

**Checkpoint**: Shared evidence and matrix controls exist; story WPs can now
produce comparable proof.

---

## Phase 3: User Story 1 — Upgrade Without Breaking Existing Integrations (P1)

**Goal**: Freeze and continuously verify the compatibility surface before
architectural work.

**Independent Test**: Unchanged downstream clients compile against published
0.1.1 and the working tree, representative XML/JSON/detection/metadata results
match, and legacy WASM exports/features remain present.

### WP-001 — Repository and Compatibility Baseline

**Goal**: Create a reviewable baseline for all existing public and wire behavior.

**Scope**: Public Rust API, generated paths, features, serialized fixtures,
detection/metadata, WASM exports, build/test behavior, dependency/compile/size
snapshots, and current known limitations.

**Non-goals**: Changing APIs, regenerating models, upgrading schemas, or approving
breaking changes.

**Dependencies**: T001–T008.

**Implementation**:

- [X] T009 [P] [US1] Add downstream compile fixtures for root reexports, every representative generated business-area path, `MxMessage`, `MxId`, `BusinessArea`, and `CatalogueEntry` to `tests/compatibility/compile/`
- [X] T010 [P] [US1] Add representative XML/JSON serialization, detection, metadata, and feature-selection goldens to `tests/compatibility/fixtures/` and `tests/compatibility_baseline.rs`
- [X] T011 [P] [US1] Add a legacy WASM export/npm smoke inventory to `tests/compatibility/wasm-exports.json` and `scripts/check-wasm-compat.sh`
- [X] T012 [US1] Add repeatable public-API and SemVer comparison commands for published 0.1.1 versus the working tree to `scripts/check-public-api.sh` and store the baseline in `evidence/compatibility/`
- [X] T013 [US1] Document exhaustive-type/trait hazards, generated-path coverage, current features, serialization contract, MSRV 1.85, Rust 2024 edition, Apache-2.0 licensing, and approved exception process in `docs/compatibility.md`
- [X] T014 [US1] Run WP-001 tests/common gates and record exact commands, fixture digests, API differences, compile/dependency/binary-size snapshots, and limitations in `evidence/WP-001.md`

**Tests**: T009–T012 compare source compatibility and observable behavior rather
than only command success.

**Evidence**: `evidence/WP-001.md` plus versioned snapshots under
`evidence/compatibility/`.

**Acceptance Criteria**:

- Existing generated paths and representative clients build unchanged.
- XML/JSON, detection, metadata, features, and supported WASM exports have
  machine-comparable baselines.
- Schema-originated and hand-written changes can be reported separately.
- No breaking change is introduced by this WP; common gates pass.

---

## Phase 4: User Story 2 — Reproduce and Audit Generated Models (P1)

**Goal**: Make every generated type traceable and regeneration byte-reproducible.

**Independent Test**: Three clean generations from the recorded input set are
byte-identical, a tracked-tree check produces no diff, and tampering/missing/stale
inputs or outputs fail nonzero.

### WP-002 — Codegen and Schema Reproducibility

**Goal**: Replace implicit/floating generation state with pinned, strict,
transactional, fully tested generation.

**Scope**: Schema/generator manifests, hashes, toolchain/lock/config pins,
transactional generation, deterministic metadata, golden/unit/regeneration tests,
and the three recorded schema exceptions.

**Non-goals**: Upgrading official schema releases, manually correcting generated
files, or implementing runtime message APIs.

**Dependencies**: WP-001.

**Implementation**:

- [X] T015 [P] [US2] Write failing manifest validation tests for all 1,130 XSDs, unique normalized identities, raw SHA-256/length, namespace/root coverage, and the three recorded schema exceptions in `tools/codegen/tests/schema_manifest.rs`
- [X] T016 [P] [US2] Write failing generator golden tests for namespace, root, documentation, enum, optional/repeated, choice, simpleContent, keyword, and metadata cases in `tools/codegen/tests/golden.rs` and `tools/codegen/tests/fixtures/`
- [X] T017 [P] [US2] Write failing clean-regeneration tests for three identical output trees, tamper detection, stale/missing output, partial failure, unknown arguments, and `--only` isolation in `tools/codegen/tests/reproducibility.rs`
- [X] T018 [US2] Add `xsds/schema-set.json` and `xsds/schema-manifest.json` with explicit verified/partial/unverified provenance and complete bidirectional XSD coverage
- [X] T019 [US2] Pin the generator toolchain, tracked workspace lockfile, xsd-parser immutable revision, and deterministic configuration in `rust-toolchain.toml`, `Cargo.lock`, `tools/codegen/Cargo.toml`, and `tools/codegen/generator-config.toml`
- [X] T020 [US2] Implement canonical schema identity/namespace/root extraction and manifest verification in `tools/codegen/src/manifest.rs` without inventing legacy provenance
- [X] T021 [US2] Refactor generation into validate → temporary tree → complete output/metadata → hash/coverage verification → atomic install in `tools/codegen/src/main.rs` and `tools/codegen/src/generate.rs`
- [X] T022 [US2] Generate deterministic `tools/codegen/generator-manifest.json` and output digest records without timestamps, hostnames, or absolute paths from `tools/codegen/src/provenance.rs`
- [X] T023 [US2] Update generated headers to the real generator/manifests and add a zero-diff `generate --check` command in `tools/codegen/src/main.rs` and `docs/codegen.md`
- [X] T024 [US2] Run three clean regenerations, all generator tests/common gates, and record input/output/config/toolchain digests plus any attributable generated diff in `evidence/WP-002.md`

**Tests**: T015–T017 must fail before implementation and then prove semantic
shapes, deterministic bytes, strict failure behavior, and complete coverage.

**Evidence**: `evidence/WP-002.md`, schema/generator manifests, and clean-tree
output digests.

**Acceptance Criteria**:

- Every XSD and generated output is represented bidirectionally.
- Generator/parser/config/toolchain/lock identities are immutable and recorded.
- Identical inputs produce byte-identical Rust and metadata three times.
- Any partial/manual/stale/tampered state fails nonzero; common gates pass.

---

## Phase 5: User Story 3 — Identify, Inspect, and Discover Messages (P1)

**Goal**: Provide schema-derived metadata, uniform access over generated values,
and complete deterministic catalogue lookup.

**Independent Test**: Parse representative areas/versions, verify all identity
fields and typed generated access, and cross-check every catalogue index against
schema/generated coverage.

### WP-003 — Message and Field Metadata

**Goal**: Emit immutable schema descriptors that drive all later discovery,
logical paths, profiles, and version diff.

**Scope**: Message identity/provenance and normalized field graph metadata.

**Non-goals**: Message instances, hand-authored catalogue rows, validation logic,
or inferred field renames.

**Dependencies**: WP-002.

**Implementation**:

- [X] T025 [P] [US3] Write failing descriptor goldens for pacs.008, head AppHdr, semt namespace exception, field QNames/paths/types/cardinality/enums/docs in `tools/codegen/tests/schema_descriptor.rs`
- [X] T026 [P] [US3] Write failing bidirectional descriptor/generated/schema coverage tests in `tests/contracts/metadata_coverage.rs`
- [X] T027 [US3] Implement the read-only descriptor IR and normalization in `tools/codegen/src/descriptor.rs` and document that it is not a canonical message model
- [X] T028 [US3] Emit deterministic message/field descriptor data and feature ownership into `src/metadata/generated.rs` and public immutable types in `src/metadata/mod.rs`
- [X] T029 [US3] Run descriptor tests/common gates and record coverage counts, exceptions, and digests in `evidence/WP-003.md`

**Tests**: Descriptor shape goldens and full-set bidirectional coverage.

**Evidence**: `evidence/WP-003.md` and generated metadata digests.

**Acceptance Criteria**:

- All generated messages map to one schema-derived descriptor.
- Logical paths and wire/Rust field mappings are deterministic and tested.
- Metadata is descriptive only and cannot store a message instance.

### WP-004 — Unified Message Abstraction

**Goal**: Parse and inspect supported messages uniformly while preserving direct
access to their concrete generated values.

**Scope**: Additive identifiers/descriptors, generated `AnyMessage` dispatch,
`ParsedMessage`/`MessageRef`, typed errors, serialization delegation, and
feature-local dispatch.

**Non-goals**: Modifying required `MxMessage` items, adding a high-level message
DTO, or compiling unrelated areas.

**Dependencies**: WP-003 and parser gate work needed for the selected entry point.

**Implementation**:

- [X] T030 [P] [US3] Write failing contract tests for identity fields, XML/JSON delegation, unknown/mismatch/unsupported errors, typed generated recovery, and area feature isolation in `tests/contracts/message_api.rs`
- [X] T031 [P] [US3] Write failing compile tests proving `MxMessage` remains externally implementable and existing exhaustive types unchanged in `tests/compatibility/message_traits/`
- [X] T032 [US3] Generate feature-gated `AnyMessage` variants and typed accessors from descriptor coverage in `tools/codegen/src/dispatch.rs` and `src/generated/any.rs`
- [X] T033 [US3] Implement additive `MessageIdentifier`, `MessageDescriptor`, `ParsedMessage`, `MessageRef`, and typed errors in `src/core/message_api.rs` without copying generated fields
- [X] T034 [US3] Route existing parse/detect compatibility functions through the new core where behavior is equivalent, preserving legacy signatures in `src/core/mod.rs` and `src/lib.rs`
- [X] T035 [US3] Add runnable generated-access and feature-isolation examples to `examples/inspect_message.rs` and record WP results in `evidence/WP-004.md`

**Tests**: Native contract tests, downstream compile tests, serialization goldens,
and one-model-area builds.

**Evidence**: `evidence/WP-004.md`.

**Acceptance Criteria**:

- Required metadata is available through one uniform view.
- The underlying generated value is borrowable/consumable with type safety.
- No parallel canonical model or unrelated family compilation is introduced.
- Legacy behavior and common gates pass.

### WP-005 — Schema-Derived Message Catalogue

**Goal**: Provide complete lookups over metadata generated from the same source as
the models.

**Scope**: Identifier/family/version/latest/namespace/root indices, generated
coverage states, stable ordering, and legacy catalogue compatibility.

**Non-goals**: Runtime scraping as authoritative catalogue data or hardcoded
message lists.

**Dependencies**: WP-003; WP-004 for parsed-message integration.

**Implementation**:

- [X] T036 [P] [US3] Write failing lookup/ordering/unknown/known-without-model and full-coverage contract tests in `tests/contracts/catalogue.rs`
- [X] T037 [P] [US3] Write failing tests that delete/add descriptor rows and require bidirectional release verification failure in `tools/codegen/tests/catalogue_coverage.rs`
- [X] T038 [US3] Generate immutable catalogue indices from `SchemaDescriptor` in `tools/codegen/src/catalogue.rs` and `src/catalogue/generated.rs`
- [X] T039 [US3] Implement the additive catalogue API while retaining `CatalogueEntry` compatibility in `src/catalogue/mod.rs` and document runtime schema fetch as non-authoritative in `src/fetch.rs`
- [X] T040 [US3] Run full catalogue/coverage/common gates and record counts, lookup fixtures, ordering, and digest evidence in `evidence/WP-005.md`

**Tests**: All required lookup modes and bidirectional metadata/model consistency.

**Evidence**: `evidence/WP-005.md`.

**Acceptance Criteria**:

- No maintained hardcoded message list exists.
- All six lookup modes are complete, stable, and schema-derived.
- Coverage state distinguishes supported, known-unavailable, and unknown.

---

## Phase 6: User Story 4 — Obtain Explainable Validation Results (P1)

**Goal**: Deliver bounded L1, structured L2/L3 reports, traced rules, core
financial validators, and reusable cross-field semantics.

**Independent Test**: Valid/invalid/boundary fixtures return deterministic
layered reports with exact IDs/paths/sources, no sensitive values, and no panic.

### WP-006 — Validation Core, Bounded XML, and Explain Registry

**Goal**: Establish the safe three-layer pipeline and one descriptor/evaluator
registry while retaining the legacy facet `Validate` API.

**Scope**: Parser/XSD spikes, `ParseLimits`, shared XML gate, report/issue types,
rule registry/explain, layer availability, safe ordering, redaction primitives.

**Non-goals**: Claiming full XSD support before R-XSD-RUST passes, profile rule
content, or replacing generated structs.

**Dependencies**: WP-003–005; research gates R-XSD-RUST and R-PARSER for their
affected acceptance claims.

**Implementation**:

- [X] T041 [P] [US4] Build and document R-XSD-RUST coverage/no-network/limits/MSRV/WASM candidates in `docs/decisions/xsd-backend.md` with executable spikes under `tools/xsd-spike/`
- [X] T042 [P] [US4] Write failing attack and boundary tests for finite-by-default input/depth/event/element/attribute/text/decoded/collection limits, forbidden DTD/entities/XInclude/network schema fetch, malformed structure/namespace/encoding, and all XML entry points in `tests/security/bounded_xml.rs`
- [X] T043 [P] [US4] Write failing report/JSON/order/unavailable-layer/explain/redaction contract tests in `tests/contracts/validation.rs`
- [X] T044 [P] [US4] Write failing canary tests ensuring `Display`, `Debug`, errors, and logs omit full XML plus IBAN/account/name/address/reference/remittance values in `tests/security/redaction.rs`
- [X] T045 [US4] Implement typed `ParseLimits`, streaming `BoundedXmlReader`, encoding policy, and safe parse errors in `src/core/bounded_xml.rs` and route detect/metadata/envelope/from_xml through it
- [X] T046 [US4] Implement `ValidationLayer`, derived `ValidationReport`, `ValidationIssue`, typed paths/codes/severity, unavailable layers, and deterministic sorting in `src/validation/report.rs`
- [X] T047 [US4] Implement coupled `Rule`/`RuleSet`/`RuleDescriptor` registry, validation context, L1/L2/L3 pipeline, and `explain` lookup in `src/validation/{rule,registry,engine}.rs`
- [X] T048 [US4] Implement default-safe `RedactionPolicy` and double-gated unsafe logging in `src/privacy/mod.rs`, keeping unsafe logging out of adapter defaults
- [X] T049 [US4] Document exact L1 backend availability/limitations, run all validation/security/common gates, and record gate decisions plus attack results in `evidence/WP-006.md`

**Tests**: T042–T044 plus backend spike tests; existing `src/validate.rs`
compatibility tests remain green.

**Evidence**: `evidence/WP-006.md` and decision records.

**Acceptance Criteria**:

- Reports contain all contracted fields and never collapse to bool/string.
- Every XML entry point shares finite default bounds and typed panic-free errors.
- Important rules are explainable from the exact execution registry.
- Full L1 is claimed only if R-XSD-RUST evidence passes; otherwise it remains
  explicitly unavailable and this acceptance item remains open.

### WP-007 — Financial Field Validators and Reference Snapshots

**Goal**: Implement typed, scoped validators for the required financial fields.

**Scope**: IBAN, BIC/BICFI, ISO 4217 currency, country, amount/precision,
date/datetime, LEI, account, and clearing identifiers; pinned reference metadata.

**Non-goals**: BIC directory existence, online lookup, or regulatory acceptance.

**Dependencies**: WP-006 and recorded reference-data license decisions.

**Implementation**:

- [X] T050 [P] [US4] Write valid/invalid/boundary tables for every required validator in `tests/validation/financial_fields.rs`
- [X] T051 [P] [US4] Write property tests for IBAN MOD-97, LEI checksum, decimal precision, Unicode/length boundaries, and panic-free malformed input in `tests/validation/financial_properties.rs`
- [X] T052 [P] [US4] Add licensed-source/digest/release fixtures and failing manifest checks for IBAN lengths, currencies, and countries in `reference-data/manifest.json` and `tests/validation/reference_data.rs`
- [X] T053 [US4] Implement `Iban`, `Bic`/`BicFi`, and `Lei` typed validators with syntax/checksum versus existence explicitly separated in `src/helpers/identifiers.rs`
- [X] T054 [US4] Implement `Currency`, `CountryCode`, `Money`, amount/precision, date/datetime, account, and clearing identifier validation in `src/helpers/{currency,money,time,account}.rs`
- [X] T055 [US4] Generate minimal reviewed reference snapshots and provenance from `reference-data/manifest.json` into `src/helpers/reference_data.rs` without bundling the BIC Directory
- [X] T056 [US4] Register explainable L2 rules and generated-field conversions in `src/validation/financial.rs` and `src/helpers/mod.rs`
- [X] T057 [US4] Add validator/module docs and runnable helper examples to `docs/validation.md` and `examples/validated_values.rs`
- [X] T058 [US4] Run validator/property/common gates and record source/license notes, fixture counts, and rule outcomes in `evidence/WP-007.md`

**Tests**: Every validator has valid, invalid, boundary, and appropriate property
tests; fuzz targets follow in WP-029.

**Evidence**: `evidence/WP-007.md` and reference-data manifest.

**Acceptance Criteria**:

- Typed errors identify constraints and stable L2 rule IDs/paths.
- Offline snapshots are versioned, hashed, licensed/reviewed, and reproducible.
- Syntax/checksum success is never represented as registry existence.

### WP-008 — Reusable Cross-Field Validation Framework

**Goal**: Express relationship rules without scattered message-specific if/else.

**Scope**: Typed required-if, xor, at-least-one, cardinality dependency, general
relation, currency/amount, and agent/account combinators.

**Non-goals**: Profile-specific rule content or a string rule DSL.

**Dependencies**: WP-006–007 and WP-003 logical paths.

**Implementation**:

- [X] T059 [P] [US4] Write failing combinator truth-table, multi-issue ordering, path, and no-value-leak tests in `tests/validation/cross_field.rs`
- [X] T060 [P] [US4] Write failing generated-message integration fixtures for conditional presence, xor, cardinality, currency/amount, and agent/account relationships in `fixtures/iso/{valid,invalid}/`
- [X] T061 [US4] Implement typed predicates/combinators and safe diagnostic templates in `src/validation/cross_field.rs`
- [X] T062 [US4] Bind selected generated message paths to combinators through generated metadata accessors in `src/validation/bindings.rs`
- [X] T063 [US4] Document rule authoring, stable IDs, descriptor/source requirements, and non-DSL rationale in `docs/rule-authoring.md`
- [X] T064 [US4] Run truth-table/fixture/common gates and record exact expected IDs/paths/results in `evidence/WP-008.md`

**Tests**: Exhaustive combinator truth tables plus generated-message fixtures.

**Evidence**: `evidence/WP-008.md`.

**Acceptance Criteria**:

- All seven relationship classes are reusable and typed.
- Rule output is deterministic, explainable, and redaction-safe.
- Selected direct-generated values validate through the same engine.

---

## Phase 7: User Story 5 — Build Common Generated Messages Safely (P2)

**Goal**: Add typed construction for the eight phase-one messages while returning
only generated `Document` values.

**Independent Test**: Each builder produces its declared generated type,
round-trips enabled serialization, rejects missing/invalid/boundary inputs with
typed errors, and the result uses the common validator.

### WP-009 — pacs.008, pacs.009, and pacs.002 Builders

**Goal**: Provide guided interbank payment/status construction over generated pacs
types.

**Scope**: One declared generated version for each phase-one pacs message, typed
helpers/conversions, build-time invariants, examples and fixtures.

**Non-goals**: A pacs DTO hierarchy, all historical versions, or embedded CBPR+
policy.

**Dependencies**: WP-004, WP-007–008.

**Implementation**:

- [X] T065 [P] [US5] Write failing valid/missing/invalid/boundary/type-identity and XML round-trip tests for pacs.008/.009/.002 in `tests/builders/pacs.rs`
- [X] T066 [P] [US5] Add valid and invalid builder fixtures with expected generated paths/errors to `fixtures/iso/builders/pacs/`
- [X] T067 [US5] Implement typed `BuilderError` and shared generated-field conversion utilities in `src/builders/error.rs` and `src/builders/mod.rs`
- [X] T068 [US5] Implement pacs.008/.009/.002 builders returning their exact generated `Document` types in `src/builders/pacs.rs`
- [X] T069 [US5] Add runnable pacs examples and generated-access/validation demonstration in `examples/build_pacs.rs` and `docs/builders.md`
- [X] T070 [US5] Run pacs builder/serialization/validation/common gates and record versions, type proofs, and fixture results in `evidence/WP-009.md`

**Tests**: Construction invariants, exact Rust output type, round-trip, and common
validation on independently created/generated values.

**Evidence**: `evidence/WP-009.md`.

**Acceptance Criteria**: All three builders return generated types, use typed
errors/helpers, cover invalid/boundary paths, and contain no profile rules.

### WP-010 — pain.001 and pain.002 Builders

**Goal**: Provide guided customer initiation/status construction over generated
pain types.

**Scope**: One declared generated version for each phase-one pain message.

**Non-goals**: A parallel customer-payment model or all pain catalogue messages.

**Dependencies**: WP-009 shared builder core.

**Implementation**:

- [X] T071 [P] [US5] Write failing valid/missing/invalid/boundary/type-identity and XML round-trip tests in `tests/builders/pain.rs`
- [X] T072 [P] [US5] Add expected generated paths and builder-error fixtures to `fixtures/iso/builders/pain/`
- [X] T073 [US5] Implement pain.001/.002 builders returning exact generated `Document` types in `src/builders/pain.rs`
- [X] T074 [US5] Add runnable pain examples and module docs in `examples/build_pain.rs` and `src/builders/pain.rs`
- [X] T075 [US5] Run pain round-trip/validation/common gates and record evidence in `evidence/WP-010.md`

**Tests**: Exact output types, required/cross-field failures, round-trip, common
validation.

**Evidence**: `evidence/WP-010.md`.

**Acceptance Criteria**: Both builders meet the shared contract without a second
model; all common gates pass.

### WP-011 — camt.052, camt.053, and camt.054 Builders/Helpers

**Goal**: Provide guided cash-report construction and reusable non-message helper
values over generated camt types.

**Scope**: One declared generated version for each phase-one camt message.

**Non-goals**: Replacing generated statements/reports with ergonomic DTOs.

**Dependencies**: WP-009 shared builder core.

**Implementation**:

- [X] T076 [P] [US5] Write failing valid/missing/invalid/boundary/type-identity and XML round-trip tests in `tests/builders/camt.rs`
- [X] T077 [P] [US5] Add expected generated paths and builder-error fixtures to `fixtures/iso/builders/camt/`
- [X] T078 [US5] Implement camt.052/.053/.054 builders and scoped helpers returning exact generated `Document` types in `src/builders/camt.rs`
- [X] T079 [US5] Add runnable camt examples and explain helper-versus-model boundaries in `examples/build_camt.rs` and `docs/builders.md`
- [X] T080 [US5] Run camt round-trip/validation/common gates and record evidence in `evidence/WP-011.md`

**Tests**: Exact output types, boundary cases, round-trip, and direct/generated
validation parity.

**Evidence**: `evidence/WP-011.md`.

**Acceptance Criteria**: All three builders produce generated values and helpers
do not become message models.

---

## Phase 8: User Story 6 — Validate Explicit Profile Releases (P2)

**Goal**: Preserve and validate exact profile releases with source traceability
and honest coverage claims.

**Independent Test**: A concrete profile key selects only its supported
messages/rules, invalid fixtures return exact IDs/paths/sources, and adding a new
release does not change historical outcomes.

### WP-016 — Versioned Profile Framework

**Goal**: Create append-only release identity, support matrices, and source
manifests over the common validation engine.

**Scope**: Composite release keys, as-of resolution, immutable registries,
supported-message/version checks, fixture/evidence linkage.

**Non-goals**: Shipping placeholder rules or treating year-only names as releases.

**Dependencies**: WP-006–008; metadata/catalogue WPs.

**Implementation**:

- [X] T081 [P] [US6] Write failing release-key parse/order/as-of/history/support/unavailable tests in `tests/profiles/framework.rs`
- [X] T082 [P] [US6] Write failing source-manifest/digest and historical fixture immutability checks in `tests/profiles/provenance.rs`
- [X] T083 [US6] Implement composite `ProfileReleaseKey`, scheme identifiers, effective ranges, and typed resolution errors in `src/profiles/release.rs`
- [X] T084 [US6] Implement append-only profile registry, supported message/version sets, and L3 rule-set dispatch in `src/profiles/registry.rs`
- [X] T085 [US6] Add source-manifest and fixture-index schemas to `profiles/README.md` and `profiles/manifest.schema.json`
- [X] T086 [US6] Document exact-release selection and “implemented rules, not acceptance/certification” semantics in `docs/profiles.md`
- [X] T087 [US6] Run profile-framework/common gates and record history/support behavior in `evidence/WP-016.md`

**Tests**: Release identity, support rejection, deterministic as-of resolution,
append-only historical reproduction.

**Evidence**: `evidence/WP-016.md`.

**Acceptance Criteria**: No bare/unresolved profile reaches validation; old
release results remain addressable and unchanged.

### WP-017 — CBPR+ Authoritative Rule Set

**Goal**: Implement the declared CBPR+ release only from authorized authoritative
sources, with complete supported-message fixtures and traceable rules.

**Scope**: Source/access gate, support matrix, independent rule implementation,
valid/invalid fixtures, explanations.

**Non-goals**: Scraping or redistributing MyStandards/SWIFT content, synthetic
coverage, or calling framework-only work a rule set.

**Dependencies**: WP-016 and R-LEGAL-CBPR.

**Implementation**:

- [ ] T088 [US6] Record authoritative CBPR+ source edition, access, implementation/distribution decision, digests, and effective dates in `profiles/cbpr_plus/source-manifest.json`; leave this task open if rights/evidence are unavailable
- [ ] T089 [P] [US6] Write expected-valid/invalid, unsupported-version, boundary, exact-rule-ID/path/source tests for every claimed supported message in `tests/profiles/cbpr_plus.rs` and `fixtures/cbpr_plus/{valid,invalid}/`
- [ ] T090 [US6] Implement only evidenced CBPR+ rules and descriptors over generated structs in `src/profiles/cbpr_plus/`
- [X] T091 [US6] Publish the exact CBPR+ support/limitation matrix without copyrighted source text in `docs/profiles/cbpr-plus.md`
- [ ] T092 [US6] Run all claimed CBPR+ fixtures/common gates and record per-rule coverage and source digests in `evidence/WP-017.md`

**Tests**: Every claimed rule has positive/negative evidence and source mapping.

**Evidence**: `evidence/WP-017.md` plus source manifest.

**Acceptance Criteria**: R-LEGAL-CBPR passes; every claim is fixture-backed; no
unsupported field/message is silently accepted as covered. Otherwise WP remains
not Done with the exact external blocker recorded.

### WP-018 — SEPA Source and Profile Framework

**Goal**: Model SEPA rulebook and implementation-guideline editions separately
and clear derived-rule distribution rights.

**Scope**: R-LEGAL-SEPA record, shared SCT/SCT Inst source identity and support
framework.

**Non-goals**: Treating consultation material as a release or claiming “SEPA
2026” without exact documents.

**Dependencies**: WP-016 and R-LEGAL-SEPA.

**Implementation**:

- [ ] T093 [US6] Record authoritative current/future rulebook and IG editions, status, effective dates, digests, and derivation/distribution review in `profiles/sepa/source-manifest.json`
- [X] T094 [P] [US6] Write failing tests that distinguish rulebook versus IG edition, consultation versus effective release, and SCT versus SCT Inst in `tests/profiles/sepa_framework.rs`
- [X] T095 [US6] Implement shared SEPA release/source/support abstractions in `src/profiles/sepa/mod.rs` without message DTOs
- [X] T096 [US6] Document exact supported editions and unresolved legal/status limits in `docs/profiles/sepa.md`
- [X] T097 [US6] Run framework/common gates and record gate outcomes in `evidence/WP-018.md`

**Tests**: Release/source distinction and rejection of unresolved/consultation
identities.

**Evidence**: `evidence/WP-018.md`.

**Acceptance Criteria**: Exact documents and legal decision are recorded before
derived production-rule data is published.

### WP-019 — SEPA SCT Rule Set

**Goal**: Implement one exact effective SEPA SCT release with traceable coverage.

**Scope**: Supported messages/versions, independently implemented rules,
valid/invalid/boundary fixtures, explanations.

**Non-goals**: SCT Inst rules or unspecified future releases.

**Dependencies**: WP-018.

**Implementation**:

- [ ] T098 [P] [US6] Write expected-valid/invalid, unsupported-version, exact-ID/path/source tests for every claimed SCT message in `tests/profiles/sepa_sct.rs` and `fixtures/sepa/sct/{valid,invalid}/`
- [ ] T099 [US6] Implement evidenced SCT rules/descriptors directly over generated structs in `src/profiles/sepa/sct/`
- [ ] T100 [US6] Add historical-release reproduction and new-release append tests in `tests/profiles/sepa_sct_history.rs`
- [ ] T101 [US6] Publish exact SCT support/limitation matrix in `docs/profiles/sepa-sct.md`
- [ ] T102 [US6] Run SCT fixtures/common gates and record rule/source/fixture coverage in `evidence/WP-019.md`

**Tests**: Positive, negative, boundary, unsupported, and history fixtures.

**Evidence**: `evidence/WP-019.md`.

**Acceptance Criteria**: All claimed SCT coverage is release-bound,
source-traceable, and reproducible.

### WP-020 — SEPA SCT Inst Rule Set

**Goal**: Implement one exact effective SEPA SCT Inst release with traceable
coverage.

**Scope**: Supported messages/versions, independent rules, fixtures,
explanations, and history.

**Non-goals**: Reusing SCT outcomes without validating SCT Inst differences.

**Dependencies**: WP-018; reusable framework from WP-019.

**Implementation**:

- [ ] T103 [P] [US6] Write expected-valid/invalid, boundary, unsupported, and exact-ID/path/source tests in `tests/profiles/sepa_sct_inst.rs` and `fixtures/sepa/sct_inst/{valid,invalid}/`
- [ ] T104 [US6] Implement evidenced SCT Inst rules/descriptors over generated structs in `src/profiles/sepa/sct_inst/`
- [ ] T105 [US6] Add historical reproduction and explicit SCT-versus-SCT-Inst difference tests in `tests/profiles/sepa_sct_inst_history.rs`
- [ ] T106 [US6] Publish exact SCT Inst support/limitation matrix in `docs/profiles/sepa-sct-inst.md`
- [ ] T107 [US6] Run SCT Inst fixtures/common gates and record coverage in `evidence/WP-020.md`

**Tests**: Positive, negative, boundary, unsupported, historical, and scheme-
difference fixtures.

**Evidence**: `evidence/WP-020.md`.

**Acceptance Criteria**: Claimed SCT Inst rules are not placeholders or SCT
aliases and remain exactly release-addressable.

---

## Phase 9: User Story 7 — Automate Through Stable Thin Adapters (P2)

**Goal**: Expose core outcomes through CLI, local MCP, and a verified WASM subset
without reimplementing domain behavior.

**Independent Test**: Identical fixtures through core and each adapter produce
equivalent normalized DTOs; CLI exit codes are stable; MCP has no external
capabilities; supported WASM exports work and legacy exports remain compatible.

### WP-012 — CLI Detect and Machine Contract

**Goal**: Establish the separate thin CLI package and stable output/exit envelope.

**Scope**: `detect`, `--json`, stdin/file input at the CLI boundary, redacted
stderr, exit codes 0/2/3/4/5/10, legacy binary shim.

**Non-goals**: CLI-local XML scanning or a human-table machine protocol.

**Dependencies**: WP-004–006.

**Implementation**:

- [X] T108 [P] [US7] Write failing detect human/JSON/stdin/file/unknown/mismatch/limit/redaction/exit-code smoke tests in `crates/cli/tests/detect.rs`
- [X] T109 [US7] Create the separate CLI crate and versioned output envelope in `crates/cli/Cargo.toml` and `crates/cli/src/output.rs`
- [X] T110 [US7] Implement `detect` solely through core APIs and preserve the root `iso20022` binary as a compatibility shim in `crates/cli/src/main.rs` and `src/bin/iso20022.rs`
- [X] T111 [US7] Run CLI/core parity/common gates and record stdout/stderr schemas and all exit statuses in `evidence/WP-012.md`

**Tests**: Command contract, redaction, exit classification, core parity.

**Evidence**: `evidence/WP-012.md`.

**Acceptance Criteria**: Detect has stable versioned JSON and never duplicates
parsing/detection.

### WP-013 — CLI Inspect and Serialization

**Goal**: Add `inspect`, `to-json`, and `to-xml` as core adapters.

**Scope**: Metadata/field inspection, enabled-format conversion, human and JSON
views, safe diagnostics.

**Non-goals**: Adapter-owned field traversal or serialization.

**Dependencies**: WP-012 and WP-004/005.

**Implementation**:

- [X] T112 [P] [US7] Write failing inspect/to-json/to-xml parity, feature-unavailable, round-trip, and redaction tests in `crates/cli/tests/inspect_serialize.rs`
- [X] T113 [US7] Implement inspect output mapping in `crates/cli/src/commands/inspect.rs`
- [X] T114 [US7] Implement enabled serialization command mapping in `crates/cli/src/commands/serialize.rs`
- [X] T115 [US7] Run command/core parity/common gates and record JSON schemas/fixtures in `evidence/WP-013.md`

**Tests**: Core DTO equivalence, round-trip, target feature absence, safe errors.

**Evidence**: `evidence/WP-013.md`.

**Acceptance Criteria**: No CLI parser/serializer exists outside core; machine
outputs are stable and versioned.

### WP-014 — CLI Validate, Catalogue, and Versions

**Goal**: Expose validation and discovery with stable machine behavior.

**Scope**: `validate`, `catalog`, `versions`, explicit layer/profile release
selection, exit 4 invalid and exit 5 unavailable.

**Non-goals**: CLI-local rules/catalogue data or implicit profile release.

**Dependencies**: WP-005–008, WP-012, and applicable profile framework.

**Implementation**:

- [X] T116 [P] [US7] Write failing valid/invalid/unavailable/profile-release/catalogue/versions/JSON/exit-code tests in `crates/cli/tests/validate_catalogue.rs`
- [X] T117 [US7] Implement validation request/report mapping in `crates/cli/src/commands/validate.rs`
- [X] T118 [US7] Implement catalogue/version query mapping in `crates/cli/src/commands/catalogue.rs`
- [X] T119 [US7] Add CLI command reference and scripting examples to `docs/cli.md`
- [X] T120 [US7] Run CLI/core parity/common gates and record outputs/exit statuses in `evidence/WP-014.md`

**Tests**: All report states, exact profile selection, stable exit/JSON, catalogue
parity.

**Evidence**: `evidence/WP-014.md`.

**Acceptance Criteria**: Scripts need only JSON plus exit status; no human-text
parsing or implicit profile selection is required.

### WP-015 — CLI Validation Explain

**Goal**: Explain the exact descriptor used by validation.

**Scope**: `explain <rule-id>`, human/JSON output, unknown ID handling,
profile/source fields, redaction.

**Non-goals**: A second explanation database or copied source documents.

**Dependencies**: WP-006 and WP-012.

**Implementation**:

- [X] T121 [P] [US7] Write failing core/CLI parity tests for L2, L3, unknown ID, source fields, and safe messages in `crates/cli/tests/explain.rs`
- [X] T122 [US7] Implement explain mapping over the core registry in `crates/cli/src/commands/explain.rs`
- [X] T123 [US7] Add rule explanation examples and source-reference semantics to `docs/cli.md` and `docs/validation.md`
- [X] T124 [US7] Run explain parity/common gates and record representative outputs in `evidence/WP-015.md`

**Tests**: The returned descriptor is object-equal to the executed rule's
descriptor; unknown IDs are typed.

**Evidence**: `evidence/WP-015.md`.

**Acceptance Criteria**: CLI explanation has one source of truth and includes all
contracted traceability fields.

### WP-026 — Local MCP Adapter

**Goal**: Provide the seven local stdio tools with structured schemas and no
file/network/shell/upload capability.

**Scope**: detect, inspect, validate, message/field lookup, version compare, rule
explain; redacted stderr and in-memory inputs.

**Non-goals**: Remote transport, URL/file tools, network access, shell execution,
or adapter-owned domain logic.

**Dependencies**: WP-005–008, WP-015, WP-021.

**Implementation**:

- [X] T125 [P] [US7] Write failing JSON-schema and core-parity contract tests for all seven MCP tools in `crates/mcp/tests/tools.rs`
- [X] T126 [P] [US7] Write failing capability/dependency/redaction tests proving no file, URL, network, shell, or payload logging path in `crates/mcp/tests/security.rs`
- [X] T127 [US7] Create the stdio-only MCP crate with versioned schemas in `crates/mcp/Cargo.toml`, `crates/mcp/schemas/`, and `crates/mcp/src/main.rs`
- [X] T128 [US7] Implement thin tool handlers over core request/result types in `crates/mcp/src/tools.rs`
- [X] T129 [US7] Document local processing, capability exclusions, privacy, limits, and tool contracts in `docs/mcp.md`
- [X] T130 [US7] Run MCP/core parity/security/common gates and record dependency/capability evidence in `evidence/WP-026.md`

**Tests**: Schema validation, normalized core equality, absent capabilities,
bounded malicious input, redaction.

**Evidence**: `evidence/WP-026.md`.

**Acceptance Criteria**: All seven tools work locally; adapter has no independent
parser/rule/catalogue and no prohibited capability.

### WP-027 — WASM Compatibility and Verified Subset

**Goal**: Preserve legacy browser behavior and expose structured bounded core APIs
for the documented subset.

**Scope**: Parsing, serialization, detection, catalogue, basic validation,
feature/target preflight, serde bindings, npm/browser tests.

**Non-goals**: Silent XSD/profile degradation or hand-built JSON.

**Dependencies**: WP-004–008 and WP-001 export baseline.

**Implementation**:

- [X] T131 [P] [US7] Write failing legacy-export and native/WASM normalized contract tests in `crates/wasm/tests/` and `tests/compatibility/wasm/`
- [X] T132 [P] [US7] Write failing WASM limit/unavailable-feature/memory-bound/redaction tests in `crates/wasm/tests/security.mjs`
- [X] T133 [US7] Create the separate WASM crate and typed serde/wasm-bindgen DTO bindings in `crates/wasm/Cargo.toml` and `crates/wasm/src/lib.rs`
- [X] T134 [US7] Route legacy exports through compatibility shims and remove manual JSON construction in `src/wasm.rs` and `crates/wasm/src/compat.rs`
- [X] T135 [US7] Document supported/unsupported feature-target combinations and preflight errors in `docs/wasm.md`
- [X] T136 [US7] Run browser/node/native parity, export, size, and common gates and record results in `evidence/WP-027.md`

**Tests**: Legacy exports, native parity, bounded input, structured unavailable
states, supported-feature matrix.

**Evidence**: `evidence/WP-027.md`.

**Acceptance Criteria**: Documented subset works before user-code compilation;
unsupported layers are explicit; legacy exports remain compatible.

---

## Phase 10: User Story 8 — Migrate SWIFT MT With Explicit Loss Reporting (P3)

**Goal**: Parse bounded MT input and produce generated ISO messages with complete
field accounting.

**Independent Test**: Each recorded MT fixture yields its expected generated
message and a field-by-field report where every input field is classified and
unmapped/lossy/ambiguous data is visible.

### WP-022 — Bounded MT Parsing and Mapping Framework

**Goal**: Establish safe transient MT syntax and complete mapping-accountability
contracts.

**Scope**: Bounded blocks/tags/continuations/Unicode, source spans, duplicates,
typed errors, classifications, reports, safe diagnostics.

**Non-goals**: A second ISO model, silent field normalization, or specific
conversion maps.

**Dependencies**: WP-006 privacy/limits patterns and WP-004 generated target APIs.

**Implementation**:

- [X] T137 [P] [US8] Write failing valid/malformed/duplicate/sequence/continuation/Unicode/size/depth/field-count tests in `crates/migration/tests/mt_parser.rs`
- [X] T138 [P] [US8] Write failing report completeness/order/redaction/classification tests in `crates/migration/tests/mapping_report.rs`
- [X] T139 [US8] Create the migration crate and bounded transient `MtDocument` parser in `crates/migration/src/mt/`
- [X] T140 [US8] Implement typed `MappingEntry`, five classifications, complete source accounting, warnings, and unmapped fields in `crates/migration/src/report.rs`
- [X] T141 [US8] Document mapping evidence and no-lossless-claim rules in `docs/migration.md`
- [X] T142 [US8] Run parser/report/security/common gates and record corpus/limit results in `evidence/WP-022.md`

**Tests**: Boundary and malformed parser cases; every parsed field accounted;
canary redaction.

**Evidence**: `evidence/WP-022.md`.

**Acceptance Criteria**: Parser is bounded/panic-free and report completeness is
enforced structurally.

### WP-023 — MT103 to pacs.008

**Goal**: Convert evidenced MT103 fields into one declared generated pacs.008
version with explicit loss/ambiguity.

**Scope**: Field map, generated target, expected XML, mapping report, warnings,
unmapped fields, applicable round-trip expectations.

**Non-goals**: Silent fallback or unproved lossless wording.

**Dependencies**: WP-009 and WP-022.

**Implementation**:

- [X] T143 [P] [US8] Add non-trivial MT103 input/expected pacs.008/report fixtures and field-by-field failing tests in `fixtures/migration/mt103/` and `crates/migration/tests/mt103.rs`
- [X] T144 [US8] Implement explicit MT103 mapping table and conversion to the generated pacs.008 `Document` in `crates/migration/src/mt103.rs`
- [X] T145 [US8] Add loss/ambiguity/unsupported and no-silent-drop regression cases in `crates/migration/tests/mt103.rs`
- [X] T146 [US8] Document supported tags, target version, classifications, and limitations in `docs/migration/mt103.md`
- [X] T147 [US8] Run conversion/validation/common gates and record field coverage in `evidence/WP-023.md`

**Tests**: Field-by-field generated output and complete report including negative
cases.

**Evidence**: `evidence/WP-023.md`.

**Acceptance Criteria**: Generated pacs.008 validates at claimed layers and every
MT103 field is visibly classified.

### WP-024 — MT202 to pacs.009

**Goal**: Convert evidenced MT202 fields into one declared generated pacs.009
version with explicit loss/ambiguity.

**Scope**: MT202 field map, generated pacs.009, expected XML, complete report,
warnings, unmapped fields, and applicable round-trip expectations.

**Non-goals**: Silent fallback, unclassified input fields, or unproved lossless
wording.

**Dependencies**: WP-009 and WP-022.

**Implementation**:

- [X] T148 [P] [US8] Add non-trivial MT202 input/expected pacs.009/report fixtures and failing tests in `fixtures/migration/mt202/` and `crates/migration/tests/mt202.rs`
- [X] T149 [US8] Implement explicit MT202 mapping to generated pacs.009 in `crates/migration/src/mt202.rs`
- [X] T150 [US8] Add loss/ambiguity/unsupported and no-silent-drop regressions in `crates/migration/tests/mt202.rs`
- [X] T151 [US8] Document target version/tag coverage/limitations in `docs/migration/mt202.md`
- [X] T152 [US8] Run conversion/validation/common gates and record evidence in `evidence/WP-024.md`

**Tests**: Field-by-field fixtures, loss classifications, generated validation,
and complete input accounting.

**Evidence**: `evidence/WP-024.md`.

**Acceptance Criteria**: Generated pacs.009 and complete mapping report satisfy
the shared migration contract.

### WP-025 — MT940 to camt.053

**Goal**: Convert evidenced MT940 statement data into one declared generated
camt.053 version with explicit loss/ambiguity.

**Scope**: MT940 statement field map, generated camt.053, expected XML, complete
report, warnings, unmapped fields, and applicable balance expectations.

**Non-goals**: Silent fallback, unclassified entries/balances, or unproved
lossless wording.

**Dependencies**: WP-011 and WP-022.

**Implementation**:

- [X] T153 [P] [US8] Add non-trivial MT940 input/expected camt.053/report fixtures and failing tests in `fixtures/migration/mt940/` and `crates/migration/tests/mt940.rs`
- [X] T154 [US8] Implement explicit MT940 mapping to generated camt.053 in `crates/migration/src/mt940.rs`
- [X] T155 [US8] Add repeated-entry/balance/loss/ambiguity/unsupported/no-silent-drop regressions in `crates/migration/tests/mt940.rs`
- [X] T156 [US8] Document target version/tag coverage/balance semantics/limitations in `docs/migration/mt940.md`
- [X] T157 [US8] Run conversion/validation/common gates and record evidence in `evidence/WP-025.md`

**Tests**: Field-by-field fixtures, repeated entry/balance semantics, loss
classifications, generated validation, and complete input accounting.

**Evidence**: `evidence/WP-025.md`.

**Acceptance Criteria**: Generated camt.053 and report expose every balance/entry
mapping and limitation.

---

## Phase 11: User Story 9 — Compare Message Versions Structurally (P3)

**Goal**: Compute semantic schema differences from generated metadata.

**Independent Test**: Curated version pairs return exact added/removed/type/
cardinality/enum changes while formatting-only schema differences produce none.

### WP-021 — Schema-Metadata Version Comparison

**Goal**: Add typed, deterministic version diff without XML text comparison.

**Scope**: Comparable-family validation, schema hashes, normalized field graph,
six change classes, stable ordering, core/CLI/MCP DTO.

**Non-goals**: Instance XML diff or unsubstantiated rename detection.

**Dependencies**: WP-003 and WP-005.

**Implementation**:

- [X] T158 [P] [US9] Write failing curated pacs.008 version-pair tests for every change class and stable ordering in `tests/compare/version_diff.rs`
- [X] T159 [P] [US9] Write failing formatting-only/no-change and unrelated-family error tests with miniature XSD descriptors in `tools/codegen/tests/version_diff.rs`
- [X] T160 [US9] Implement normalized field-graph comparison and typed changes in `src/compare/mod.rs`
- [X] T161 [US9] Expose version comparison in core catalogue APIs and adapter DTOs in `src/compare/api.rs`
- [X] T162 [US9] Document hashes, ordering, comparable-family rules, and no-rename inference in `docs/version-comparison.md`
- [X] T163 [US9] Run diff/golden/common gates and record curated expected changes in `evidence/WP-021.md`

**Tests**: All six change types, no-change formatting, unrelated family, ordering.

**Evidence**: `evidence/WP-021.md`.

**Acceptance Criteria**: Output is complete for curated pairs, metadata-based,
hash-identified, and deterministic.

---

## Phase 12: User Story 10 — Operate Safely and Reproduce Releases (P3)

**Goal**: Prove bounded operation, privacy, fuzz resilience, and measured runtime/
build cost before release.

**Independent Test**: Attack corpus and fuzz targets show no panic/crash/UB and
stable resource failures, canaries never leak by default, and reproducible
benchmark/build evidence is recorded.

### WP-028 — Security and Privacy Hardening

**Goal**: Close all XML/adapter/privacy attack paths and publish supported limits.

**Scope**: Shared-entry-point audit, malicious corpus, exact-boundary behavior,
encoding/namespace/Unicode/collection attacks, redaction audit, unsafe-log guard.

**Non-goals**: Claiming formal verification or accepting unbounded overrides.

**Dependencies**: WP-006, all parsers/adapters, and R-PARSER.

**Implementation**:

- [X] T164 [P] [US10] Add minimized oversized/deep/entity/DTD/XInclude/namespace/Unicode/collection attack corpus with expected typed failures to `fixtures/security/xml/`
- [X] T165 [P] [US10] Add exact-limit-minus/at/plus boundary tests for every `ParseLimits` field across core/CLI/MCP/WASM in `tests/security/limits_matrix.rs`
- [X] T166 [P] [US10] Add whole-workspace canary scanning for logs/errors/Debug/reports/mapping/adapters/panic hooks in `tests/security/privacy_matrix.rs`
- [X] T167 [US10] Audit and route every remaining XML/MT entry point through bounded gates in `src/` and `crates/`, documenting unavoidable backend behavior in `docs/security.md`
- [X] T168 [US10] Harden allocation/collection accounting and unsafe logging compile/runtime guards in `src/core/bounded_xml.rs` and `src/privacy/mod.rs`
- [X] T169 [US10] Add security policy, vulnerability reporting, supported versions, and privacy disclosure guidance to `SECURITY.md`
- [X] T170 [US10] Run attack/privacy/target/common gates and record R-PARSER closure, resource observations, and residual risks in `evidence/WP-028.md`

**Tests**: Exact bounds, all entry points, malicious corpus, all sensitive canary
classes, native/WASM adapters.

**Evidence**: `evidence/WP-028.md`.

**Acceptance Criteria**: Finite documented defaults protect every entry point;
no default sensitive disclosure; no tested panic/crash/unbounded behavior.

### WP-029 — Coverage-Guided Fuzzing

**Goal**: Continuously exercise raw parsing/detection/validation/migration
surfaces with persistent regression corpora.

**Scope**: XML parser, message/namespace detector, IBAN, BIC, LEI, selected
validators, MT parser/converters; timeout/OOM/crash classification.

**Non-goals**: Running nightly fuzz tooling in the MSRV build or counting
proptest alone as coverage-guided fuzzing.

**Dependencies**: WP-007, WP-022–025, WP-028.

**Implementation**:

- [X] T171 [P] [US10] Create the separate nightly cargo-fuzz workspace and pinned fuzz toolchain/config in `fuzz/Cargo.toml`, `fuzz/rust-toolchain.toml`, and `fuzz/fuzz.toml`
- [X] T172 [P] [US10] Add bounded XML/detect/namespace fuzz targets and attack/valid seed corpora in `fuzz/fuzz_targets/` and `fuzz/corpus/`
- [X] T173 [P] [US10] Add IBAN/BIC/LEI/selected-validator fuzz targets with checksum/property invariants in `fuzz/fuzz_targets/financial.rs`
- [X] T174 [P] [US10] Add MT parser and three converter fuzz targets with complete-field-accounting invariants in `fuzz/fuzz_targets/migration.rs`
- [X] T175 [US10] Add bounded scheduled CI workflow, corpus minimization, and regression replay to `.github/workflows/fuzz.yml` and `scripts/run-fuzz-baseline.sh`
- [ ] T176 [US10] Run declared baselines and record engine/toolchain/targets/duration/executions/corpus digests plus crash/timeout/OOM counts in `evidence/WP-029.md`

**Tests**: Corpus replay is a normal deterministic job; scheduled fuzzing is
bounded and treats timeout/OOM as failures.

**Evidence**: `evidence/WP-029.md` and minimized regression corpus.

**Acceptance Criteria**: Every required target exists, seeds include attacks and
valid structures, and the baseline has no unresolved crash/panic/UB/resource
failure.

### WP-030 — Runtime, Compile, Dependency, and Size Baselines

**Goal**: Record reproducible measurements before any performance claim.

**Scope**: Six required runtime benchmarks, allocations/peak memory where
practical, default feature compile time, binary/WASM size, dependency counts,
measurement metadata.

**Non-goals**: Premature optimization or comparing unlike runners.

**Dependencies**: Relevant functional WPs and representative fixture digests.

**Implementation**:

- [X] T177 [P] [US10] Add pacs.008 parse/serialize/detect/validate, camt.053 parse, and catalogue lookup benchmarks to `crates/benchmarks/benches/sdk.rs`
- [X] T178 [P] [US10] Add platform-qualified allocation and peak-RSS harnesses that never label latency as memory data in `crates/benchmarks/src/memory.rs`
- [X] T179 [P] [US10] Add default/selected-feature compile-time, dependency-count, CLI binary-size, and WASM-size measurement scripts to `scripts/measure-build-baseline.sh`
- [X] T180 [US10] Define comparable-runner metadata and regression review policy in `docs/performance.md`
- [X] T181 [US10] Run runtime/memory/build/size baselines with commit/toolchain/CPU/OS/features/fixture digests in `evidence/performance/baseline.json`
- [X] T182 [US10] Run benchmark smoke/common gates and summarize results without unsupported optimization claims in `evidence/WP-030.md`

**Tests**: Benchmarks parse/validate their expected fixtures before timing; scripts
fail on missing metadata or measurement output.

**Evidence**: `evidence/WP-030.md` and `evidence/performance/baseline.json`.

**Acceptance Criteria**: All required workloads and build-cost dimensions have
reproducible context; no unmeasured performance claim remains.

---

## Phase 13: Polish, Documentation, Release, and Project Sustainability

**Purpose**: Integrate all independently proven stories into a documented,
auditable, reproducible release candidate without publishing it.

### WP-031 — Complete SDK Documentation

**Goal**: Make supported behavior, examples, limitations, architecture, and
compliance meaning discoverable.

**Scope**: Generated/high-level API, validation/profiles/builders, CLI,
migration, WASM, MCP, security/privacy, module docs, runnable examples, README.

**Non-goals**: Marketing unsupported profiles or bank/regulatory guarantees.

**Dependencies**: All implemented capability WPs.

**Implementation**:

- [X] T183 [P] Add or complete module-level docs for every public module under `src/` and `crates/*/src/`, including generated-access statements and typed error semantics
- [X] T184 [P] Add runnable examples for parse/inspect/catalogue/validation/explain/profile/builders/migration/version diff in `examples/` and doctest them with explicit features
- [X] T185 Update `README.md` first screen to say “Production-grade ISO 20022 SDK for Rust,” show only completed capabilities, state generated models remain accessible/high-level APIs additive, and add “Using rust_iso20022 in production?” sponsor language
- [X] T186 Complete user guides and feature/platform matrix under `docs/`, including exact profile/schema/migration versions and known limitations
- [X] T187 Add the schema/semantic/profile tooling versus bank/certification/onboarding/legal-compliance disclaimer consistently to `README.md`, `docs/`, CLI help, MCP schemas, and WASM package docs
- [X] T188 Run rustdoc with the docs.rs feature set, all doctests/link checks/common gates, and record public-module/example coverage in `evidence/WP-031.md`

**Tests**: Runnable examples, doctests, docs.rs-equivalent build, link and claim
audit.

**Evidence**: `evidence/WP-031.md`.

**Acceptance Criteria**: Every public module is documented; core examples run;
unsupported capabilities are not advertised; disclaimer is consistent.

### WP-032 — Release Engineering and Supply Chain

**Goal**: Produce a reproducible, unsigned/unpublished release baseline and
automation for a later authorized release.

**Scope**: SemVer/changelog, CI release matrix, GitHub/crates/docs.rs/CLI assets,
checksums, schema/profile/generator manifest attachment, audit/licenses/lockfile,
SBOM evaluation.

**Non-goals**: Publishing, pushing, tagging, signing with unavailable keys, or
accepting failing mandatory gates.

**Dependencies**: All preceding WPs and closed acceptance criteria.

**Implementation**:

- [X] T189 [P] Add release workflow templates for crates.io dry-run/package, docs.rs-equivalent docs, GitHub CLI binaries, WASM package, checksums, and manifest attachments to `.github/workflows/release.yml`
- [X] T190 [P] Add maintainable dependency audit, license policy, tracked lockfile, and cargo-deny decision/configuration to `.github/workflows/supply-chain.yml`, `deny.toml`, and `docs/supply-chain.md`
- [X] T191 [P] Add practical SBOM generation/evaluation with format/tool/version recorded to `scripts/generate-sbom.sh` and `docs/releasing.md`
- [X] T192 Update `CHANGELOG.md` with hand-written versus schema-generated changes, MSRV, feature aliases, SemVer decisions, migration notes, and no-publish status
- [X] T193 Add release baseline automation covering workspace build/test/clippy/fmt/diff, full feature/target/MSRV matrix, CLI/MCP/WASM smoke, profile/migration fixtures, fuzz replay/baseline, benchmarks, docs, audit/licenses, package contents, and checksums to `scripts/release-check.sh`
- [X] T194 Generate supported ISO versions, profile releases, migrations, MSRV, feature matrix, known limitations, schema release, generator version, and source digests into `evidence/release-baseline/`
- [X] T195 Run `cargo package --list` and `cargo package` for publishable crates without publishing, inspect docs.rs memory feature set, and record artifacts/checksums in `evidence/release-baseline/package.md`
- [ ] T196 Run the entire release check, require all mandatory gates to pass or record an exact external blocker without marking release ready, and write `evidence/WP-032.md`

**Tests**: Release scripts have failure-injection tests for a skipped matrix row,
dirty generated output, audit/license failure, missing checksum/evidence, and
failed adapter smoke.

**Evidence**: `evidence/WP-032.md` and complete
`evidence/release-baseline/`.

**Acceptance Criteria**:

- The release baseline is reproducible and all required commands/matrices pass.
- Package/docs.rs contents build within documented constraints.
- No publish/tag/push occurs in this WP.
- Any external gate still open prevents “release ready” status and names exact
  remaining work.

### WP-033 — Sponsorship and GitHub Project Metadata

**Goal**: Explain sustainable standards maintenance while keeping the core open.

**Scope**: Ko-fi metadata, sponsor policy, project health files, issue templates,
roadmap alignment.

**Non-goals**: Gating core code behind sponsorship or promising private
standards access.

**Dependencies**: WP-031 wording and WP-032 support matrix.

**Implementation**:

- [X] T197 [P] Verify `.github/FUNDING.yml` points to `ko_fi: jnz` and add sponsor-maintenance scope to `SPONSORS.md`
- [X] T198 [P] Add/update `CONTRIBUTING.md`, `SECURITY.md`, `ROADMAP.md`, and `CHANGELOG.md` so contribution, disclosure, Spec Kit WP, release, and standards-update processes agree
- [X] T199 [P] Add bug, schema issue, validation-rule issue, profile issue, and feature-request forms to `.github/ISSUE_TEMPLATE/` with source/release/evidence prompts and privacy warnings
- [X] T200 Run metadata link/content/common gates and record open-core/sponsor wording plus project-file inventory in `evidence/WP-033.md`

**Tests**: GitHub YAML/form syntax, links, privacy wording, and README/SPONSORS
consistency.

**Evidence**: `evidence/WP-033.md`.

**Acceptance Criteria**: Ko-fi is configured; sponsors support ongoing schemas,
profiles, migration, security, fuzzing, docs, and maintenance without unlocking
closed core functionality; all required project files/forms exist.

---

## Dependencies and Execution Order

### Critical path

```text
WP-001 compatibility
  → WP-002 reproducible codegen
  → WP-003 metadata
  → WP-004 message abstraction
  → WP-005 catalogue
  → WP-006 validation/security core
  → WP-007 financial validators
  → WP-008 cross-field rules
  ├→ WP-009 → WP-010/WP-011 builders
  ├→ WP-016 → WP-017 and WP-018 → WP-019/WP-020 profiles
  ├→ WP-012 → WP-013/WP-014/WP-015 CLI
  ├→ WP-021 version diff → WP-026 MCP
  └→ WP-022 → WP-023/WP-024/WP-025 migration

All applicable capabilities → WP-027/028 → WP-029/030 → WP-031 → WP-032 → WP-033
```

### External research gates

- R-XSD-RUST gates full L1 acceptance, not L2/L3 framework work.
- R-PARSER gates WP-028 hardened-parser acceptance, not metadata/builders.
- R-LEGAL-CBPR gates WP-017 completion, not WP-016 framework work.
- R-LEGAL-SEPA gates derived profile publication and WP-018–020 completion, not
  general validation.

### Parallel opportunities

- Within a WP, only tasks marked `[P]` may proceed concurrently; tests remain
  written before the implementation they specify.
- After WP-008, builder, CLI-foundation, profile-framework, version-diff, and
  migration-framework branches can proceed independently.
- After WP-016, CBPR+ source work and SEPA source work are independent.
- After WP-022, all three migration mappings are independent.
- Security/fuzz/performance fixture authoring can proceed in parallel once each
  target capability is stable.

## Parallel Examples

### User Story 3

```text
T025 descriptor goldens  ||  T026 full metadata coverage
then T027 → T028 → T029

T030 message contracts   ||  T031 compatibility compile tests
then T032 → T033 → T034 → T035
```

### User Story 6

```text
After WP-016:
  WP-017 CBPR+ authoritative-source branch
  WP-018 SEPA source/framework branch → WP-019 and WP-020 in parallel
```

### User Stories 7–9

```text
After their foundations:
  CLI WP-012..015
  version diff WP-021
  migration WP-022..025
Then MCP consumes core outputs; WASM parity can proceed against stable core APIs.
```

## Implementation Strategy

### MVP: Compatibility + Reproducible Canonical Model

The minimum trustworthy increment is User Stories 1 and 2 (WP-001 and WP-002).
It does not yet claim a production-grade SDK; it freezes what must not break and
makes the canonical generated model auditable. Validate both independently before
metadata/API work.

### Incremental delivery

1. Complete setup, foundational controls, WP-001, and WP-002.
2. Deliver schema-derived metadata/message/catalogue as an independently tested
   discovery slice.
3. Deliver bounded validation core and field/cross-field rules before profiles.
4. Add builders and native CLI as separate, demonstrable generated-model slices.
5. Add only profile rules whose exact source/release/legal evidence is available.
6. Add version comparison, migration, MCP, and WASM through core contracts.
7. Complete hardening, fuzzing, measurements, docs, and release evidence.
8. Do not publish until the user separately authorizes publishing and every
   release acceptance criterion passes.

## Format and Completeness Validation

- Tasks T001–T200 use the required checkbox, sequential ID, optional `[P]`, user
  story label where required, concrete action, and exact file path.
- Every user story has an independent test and is mapped to concrete WPs.
- Every WP declares Goal, Scope, Non-goals, Dependencies, Implementation, Tests,
  Evidence, and Acceptance Criteria.
- User-requested WP-001 through WP-033 are all present; WP-002 contains the
  manifest, pinning, transactional, and reproducibility sub-work identified by
  research.
- The full Definition of Done and no-false-completion rule apply to every WP.
