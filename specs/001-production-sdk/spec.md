# Feature Specification: Production-Grade ISO 20022 SDK

**Feature Branch**: `not-created`

**Created**: 2026-09-27

**Status**: Draft

**Input**: Transform rust_iso20022 from its existing generated-model/codegen
library into a production-grade ISO 20022 SDK while retaining generated message
types as the sole canonical model and delivering traceable validation, profiles,
builders, migration, adapters, security, evidence, and a reproducible release
baseline.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Upgrade Without Breaking Existing Integrations (Priority: P1)

As an existing library user, I can upgrade to the production SDK while retaining
the generated type paths, serialization behavior, detection, metadata, feature
selection, and supported browser-facing behavior on which my application relies.
New high-level capabilities are additive and never prevent direct access to the
generated message value.

**Why this priority**: Compatibility is a prerequisite for every later
capability and protects existing financial integrations from unplanned change.

**Independent Test**: Capture the published public surface and representative
serialized messages before the upgrade, then demonstrate that unchanged client
programs and compatibility fixtures still build and produce equivalent results.

**Acceptance Scenarios**:

1. **Given** a client using an existing generated message path, **When** it
   upgrades without changing source, **Then** the path remains accessible and
   behaves according to the compatibility baseline.
2. **Given** a representative XML, JSON, metadata, detection, feature, or WASM
   fixture from the baseline, **When** it is processed by the enhanced SDK,
   **Then** its result remains equivalent unless a separately documented and
   approved breaking change applies.
3. **Given** a high-level parsed or built message, **When** a caller needs
   schema-level access, **Then** the caller can obtain and use the underlying
   generated message value.

---

### User Story 2 - Reproduce and Audit Generated Models (Priority: P1)

As a maintainer or auditor, I can determine which official schema, schema
release, generator version, and generator configuration produced any generated
message type, and I can repeat generation with identical inputs without a diff.

**Why this priority**: Every SDK layer depends on the accuracy and provenance of
the generated canonical model.

**Independent Test**: Regenerate from a recorded input set in a clean
environment, compare the complete output, and trace selected generated types back
to their recorded schema hashes and generator identity.

**Acceptance Scenarios**:

1. **Given** a generated message type, **When** its provenance is requested,
   **Then** the corresponding schema file, release, hash, generator identity,
   and configuration are available.
2. **Given** identical recorded generation inputs, **When** generation is run
   twice, **Then** the generated outputs and metadata are byte-for-byte equal.
3. **Given** a required change to generated output, **When** it is reviewed,
   **Then** the change is attributable to schema, generator, or configuration
   changes rather than a manual generated-file edit.

---

### User Story 3 - Identify, Inspect, and Discover Messages (Priority: P1)

As an application developer, I can parse an ISO 20022 message into a uniform
message view, identify its business area, family, number, version, namespace,
root element, and description, serialize it, validate it, and still access its
generated value. I can discover supported families and versions through a
catalogue produced from the same source as the generated models.

**Why this priority**: Consistent message identity and discovery are the entry
point for validation, profiles, builders, migration, and adapters.

**Independent Test**: Process representative messages from multiple business
areas and versions, compare their reported identities with the source catalogue,
and perform every required catalogue lookup.

**Acceptance Scenarios**:

1. **Given** a supported ISO 20022 message, **When** it is parsed, **Then** the
   uniform message view reports the correct identity, business area, family,
   number, version, namespace, root, and description.
2. **Given** a family or identifier, **When** catalogue queries request all
   versions or the latest version, **Then** the results are complete,
   deterministic, and consistent with generated coverage.
3. **Given** a namespace and root element, **When** catalogue lookup is
   performed, **Then** the matching message or an explicit not-found result is
   returned without relying on a manually maintained parallel list.

---

### User Story 4 - Obtain Explainable Validation Results (Priority: P1)

As a payment engineer or operations analyst, I can validate syntax/schema
constraints, ISO 20022 semantic rules, and market-profile rules separately or
together. Every issue identifies its severity, stable code, field path, rule ID,
reason, and relevant profile source so that I can explain and remediate it.

**Why this priority**: Production adoption requires more than deserialization;
validation must be actionable and auditable.

**Independent Test**: Run valid, invalid, and boundary fixtures across all three
layers and confirm the exact rule IDs, paths, severities, explanations, and
profile releases in the resulting reports.

**Acceptance Scenarios**:

1. **Given** a message with invalid financial field values, **When** validation
   runs, **Then** the report identifies every implemented violation without
   panicking or reducing the result to a boolean or unstructured string.
2. **Given** cross-field violations involving conditional presence, exclusion,
   cardinality, amount/currency, or agent/account relationships, **When** reusable
   rules run, **Then** each issue names the affected field path and rule.
3. **Given** a validation rule ID, **When** its explanation is requested,
   **Then** the affected messages, profile and release when applicable, field
   path, reason, and source/reference are returned.
4. **Given** a profile-valid message, **When** validation succeeds, **Then** the
   result states validity according to implemented rules without claiming bank
   acceptance, regulatory certification, network approval, or legal compliance.

---

### User Story 5 - Build Common Generated Messages Safely (Priority: P2)

As an application developer, I can construct the first-phase payment and cash
management messages using validated helper values and guided builders. The final
result is the existing generated type, and construction failures are structured
domain errors.

**Why this priority**: Builders improve usability without creating a second
canonical message model.

**Independent Test**: Build each first-phase message from valid inputs, use the
result wherever its generated type is expected, and verify invalid and boundary
inputs yield the specified typed errors.

**Acceptance Scenarios**:

1. **Given** valid inputs for pacs.008, pacs.009, pacs.002, pain.001, pain.002,
   camt.052, camt.053, or camt.054, **When** construction completes, **Then** the
   result is that message version's generated type and round-trips through its
   supported serialization.
2. **Given** invalid IBAN, BIC/BICFI, currency, country, amount, precision, date,
   datetime, LEI, account, or clearing identifier input, **When** a helper or
   builder accepts it, **Then** a typed error identifies the failed constraint.
3. **Given** a generated value obtained independently of a builder, **When** it
   is validated, **Then** it follows the same validation path as a builder result.

---

### User Story 6 - Validate Explicit Profile Releases (Priority: P2)

As a financial institution, I can select an explicit CBPR+, SEPA SCT, or SEPA SCT
Inst release and validate only the messages and versions supported by that
release. Historical releases remain reproducible after newer rules are added.

**Why this priority**: Unversioned profile names are operationally ambiguous and
cannot support annual standards updates.

**Independent Test**: Validate the same message against recorded 2026 profile
fixtures and a retained historical release, confirming supported-version checks,
release-specific rules, and stable explanations.

**Acceptance Scenarios**:

1. **Given** a message and an explicit 2026 profile release, **When** validation
   runs, **Then** only that release's supported messages, versions, and rules are
   applied and identified in the report.
2. **Given** an unsupported message or version, **When** profile validation is
   requested, **Then** the result clearly reports unsupported coverage rather
   than silently applying a nearby rule set.
3. **Given** a newer profile release, **When** it is added, **Then** existing
   release fixtures and outcomes remain reproducible.

---

### User Story 7 - Automate SDK Capabilities Through Stable Adapters (Priority: P2)

As an operator or automation author, I can detect, inspect, validate, explain,
convert serialization, and query the catalogue from a command-line interface
with human-readable or machine-readable output. Browser and MCP consumers receive
the same core results without adapters reinterpreting the message.

**Why this priority**: Stable adapters make the core SDK usable in operations,
developer tooling, browsers, and local AI workflows.

**Independent Test**: Execute every required command and adapter operation on
the same fixtures, compare their structured results with direct SDK results, and
verify stable failure classifications.

**Acceptance Scenarios**:

1. **Given** a valid message, **When** detect, inspect, validate, explain,
   to-JSON, to-XML, catalogue, or version commands run, **Then** human and
   machine-readable output accurately represents the core SDK result.
2. **Given** validation, parse, and internal failures, **When** a command exits,
   **Then** each failure class has a distinct stable exit status.
3. **Given** a local MCP request for detection, inspection, validation, lookup,
   field lookup, version comparison, or error explanation, **When** it runs with
   defaults, **Then** processing remains local with no shell execution, external
   network access, or message upload.
4. **Given** a documented browser-supported capability, **When** it is built and
   exercised for the browser target, **Then** it behaves consistently with the
   core SDK; unsupported combinations are identified before user code fails deep
   in compilation.

---

### User Story 8 - Migrate SWIFT MT With Explicit Loss Reporting (Priority: P3)

As a migration engineer, I can transform MT103, MT202, and MT940 inputs into the
corresponding generated ISO 20022 messages and receive a field-level report that
classifies exact, derived, lossy, ambiguous, unsupported, and unmapped data.

**Why this priority**: Migration is valuable only when data loss and ambiguity
are visible and testable.

**Independent Test**: Run the recorded MT fixtures, compare every output field
with its expected ISO 20022 value, and compare the complete mapping report and
warnings with the expected evidence.

**Acceptance Scenarios**:

1. **Given** an MT103, MT202, or MT940 fixture, **When** migration completes,
   **Then** the result contains the corresponding generated pacs.008, pacs.009,
   or camt.053 message plus a mapping report, warnings, and unmapped fields.
2. **Given** a field that cannot be represented exactly, **When** migration
   processes it, **Then** the result classifies it as Derived, Lossy, Ambiguous,
   or Unsupported and never silently discards it.
3. **Given** a conversion described as lossless, **When** its evidence is
   inspected, **Then** field-by-field proof and any applicable round-trip
   expectations support that claim.

---

### User Story 9 - Compare Message Versions Structurally (Priority: P3)

As an integration maintainer, I can compare two versions of a message and see
added or removed fields, type changes, cardinality changes, and enumeration
changes based on authoritative schema metadata.

**Why this priority**: Upgrade planning requires semantic schema differences,
not textual XML differences.

**Independent Test**: Compare known version pairs with curated expected changes
and verify that formatting-only schema differences do not appear as changes.

**Acceptance Scenarios**:

1. **Given** two supported versions of the same message family, **When** they are
   compared, **Then** the result classifies all known field, type, cardinality,
   and enum changes.
2. **Given** equivalent schema semantics with different source formatting,
   **When** versions are compared, **Then** no false change is reported.

---

### User Story 10 - Operate Safely and Reproduce Releases (Priority: P3)

As a security reviewer or release manager, I can verify that untrusted messages
are processed within defined resource limits, sensitive data is redacted by
default, performance and build costs are measured, and every release carries the
schema, profile, generator, compatibility, and supply-chain evidence needed to
reproduce it.

**Why this priority**: Production readiness requires bounded behavior and a
repeatable release, not only functional APIs.

**Independent Test**: Exercise malicious and boundary inputs, inspect logs,
execute the release verification matrix, and reconstruct the release identity
from recorded manifests and checksums.

**Acceptance Scenarios**:

1. **Given** oversized, deeply nested, entity-expanding, malformed namespace,
   malformed Unicode, or excessive-collection input, **When** processing occurs,
   **Then** it stops within documented bounds without panic, crash, or unbounded
   allocation.
2. **Given** a message containing sensitive financial fields, **When** normal or
   debug logging occurs, **Then** raw values and full messages are absent unless
   the user explicitly enables unsafe full logging.
3. **Given** a release candidate, **When** release verification completes,
   **Then** compatibility, workspace quality gates, adapters, browser support,
   profiles, migrations, fuzzing, benchmarks, MSRV, feature matrix, known
   limitations, checksums, and supply-chain results are recorded.

### Edge Cases

- A valid XML document has an unknown namespace, unknown root, or a known
  namespace paired with the wrong root.
- A message is valid at the ISO semantic layer but unsupported or invalid for the
  selected profile release.
- Multiple message versions share similar structures but different namespaces,
  required fields, cardinalities, or enum members.
- A catalogue entry exists without generated coverage, or generated coverage
  exists without provenance metadata.
- Helper values are syntactically valid but violate cross-field relationships in
  the containing generated message.
- A profile source is corrected after release; historical evidence must remain
  reproducible while the correction is introduced explicitly.
- MT input contains duplicate tags, unexpected sequences, continuation lines,
  non-Latin text, or data with no ISO 20022 equivalent.
- Redaction receives short, malformed, or non-ASCII identifiers and must not
  reveal more information than intended.
- The input reaches the exact configured size, depth, collection, or allocation
  boundary.
- A feature combination or browser target excludes a dependency required by a
  requested capability.
- Version comparison is requested across different message families rather than
  two versions of the same family.
- A release must be rebuilt after an upstream dependency or official schema is no
  longer available at its original network location.

## Requirements *(mandatory)*

### Scope Boundaries

The production SDK includes generated models, reproducible generation and
catalogue metadata, uniform message identity, layered validation, versioned
profiles, first-phase builders, CLI, local MCP, defined browser support, first-
phase MT migration, structural version comparison, privacy and parser hardening,
fuzzing, performance/build baselines, documentation, project governance, and
release evidence.

The first profile releases are CBPR+ 2026, SEPA SCT 2026, and SEPA SCT Inst 2026.
The first builder families are pacs.008, pacs.009, pacs.002, pain.001, pain.002,
camt.052, camt.053, and camt.054. The first migrations are MT103 to pacs.008,
MT202 to pacs.009, and MT940 to camt.053. Later profiles, builders, and migrations
are outside this feature's first release unless added by an approved specification
amendment.

This SDK provides schema validation, implemented semantic and profile rules, and
developer tooling. It does not guarantee acceptance by a particular bank,
regulatory certification, network onboarding approval, or legal compliance.

### Functional Requirements

#### Canonical Model and Compatibility

- **FR-001**: Generated schema-derived message values MUST remain the sole
  canonical ISO 20022 data model.
- **FR-002**: Every builder and successful migration MUST return an existing
  generated message value rather than a parallel long-lived message model.
- **FR-003**: Uniform message, validation, and profile capabilities MUST preserve
  direct access to the underlying generated value.
- **FR-004**: The product MUST publish a compatibility baseline covering existing
  generated paths, public APIs, serialization behavior, metadata, detection,
  feature flags, and supported browser-facing behavior.
- **FR-005**: Compatibility verification MUST detect removals or behavior changes
  against that baseline before release.
- **FR-006**: Every approved breaking change MUST identify its justification,
  affected users, migration instructions, and appropriate release impact.
- **FR-007**: Schema-originated structural changes MUST be reported separately
  from hand-authored API changes.

#### Schema, Generation, and Catalogue Provenance

- **FR-008**: Every schema input MUST have a manifest entry containing identity,
  release provenance, cryptographic hash, and generated coverage.
- **FR-009**: Generation evidence MUST record generator identity, configuration,
  input manifest, and output metadata.
- **FR-010**: Identical recorded inputs MUST reproduce identical generated output
  and generation metadata.
- **FR-011**: Generated output MUST be replaceable solely by regeneration; manual
  edits MUST be detected or prohibited by verification.
- **FR-012**: Generation verification MUST cover message structure, enums,
  optional and repeated fields, namespaces, documentation, and metadata.
- **FR-013**: The message catalogue MUST derive from the same recorded inputs and
  metadata as generated coverage.
- **FR-014**: The catalogue MUST support exact identifier lookup, namespace
  lookup, namespace-plus-root lookup, family listing, version listing, and latest
  version lookup.
- **FR-015**: Catalogue results MUST distinguish supported generated messages,
  known messages lacking generated coverage, and unknown messages.
- **FR-016**: Catalogue and generated coverage MUST be checked for bidirectional
  consistency on every release candidate.

#### Uniform Message Capability

- **FR-017**: A supported parsed message MUST expose identifier, business area,
  family, message number, version, namespace, root element, and description.
- **FR-018**: The uniform message capability MUST serialize supported messages to
  their enabled formats and submit them to the same validation engine used by
  direct generated values.
- **FR-019**: Detection MUST distinguish parse failure, unknown message,
  namespace/root mismatch, unsupported generated coverage, and successful
  identification.
- **FR-020**: Uniform parsing and detection MUST not require enabling unrelated
  message families.

#### Validation and Explainability

- **FR-021**: Validation MUST expose L1 syntax/schema, L2 ISO semantic, and L3
  market/network profile results as distinguishable layers.
- **FR-022**: Every validation run MUST return a report containing overall
  validity, errors, and warnings.
- **FR-023**: Every validation issue MUST contain a stable code, severity, field
  path, human-readable message, and rule ID.
- **FR-024**: Every profile validation issue MUST additionally contain profile
  identity, profile release, and source/reference.
- **FR-025**: Validation MUST accumulate applicable issues rather than return only
  the first failure, except when input cannot be safely parsed further.
- **FR-026**: Rule explanations MUST be retrievable by rule ID and identify rule
  purpose, affected messages, field paths, reason, profile release when
  applicable, and source/reference.
- **FR-027**: The SDK MUST validate IBAN, BIC/BICFI, ISO 4217 currency, country,
  amount, decimal precision, date, datetime, LEI, account identifiers, and
  clearing identifiers with structured errors.
- **FR-028**: Each financial field validator MUST cover valid, invalid, and
  boundary cases and MUST include fuzz evidence when arbitrary input can exercise
  meaningful parser or checksum behavior.
- **FR-029**: Reusable cross-field rules MUST express conditional required fields,
  mutual exclusion, at-least-one constraints, cardinality dependencies, field
  relationships, currency/amount relationships, and agent/account relationships.
- **FR-030**: Validation MUST operate on generated values regardless of whether
  they came from parsing, a builder, migration, or direct construction.
- **FR-031**: Validation and rule explanation MUST never panic on caller-provided
  message data.

#### Profiles

- **FR-032**: Every profile selection MUST include an explicit release identity;
  an unversioned profile selection MUST be rejected.
- **FR-033**: Every profile release MUST declare supported messages, supported
  versions, rule set, fixtures, references, and validation evidence.
- **FR-034**: CBPR+ 2026, SEPA SCT 2026, and SEPA SCT Inst 2026 MUST be available
  in the first production release.
- **FR-035**: Adding a profile release MUST preserve reproducibility of retained
  historical releases.
- **FR-036**: Profile validation MUST explicitly report an unsupported message or
  version and MUST NOT silently substitute another release.

#### Helpers and Builders

- **FR-037**: Typed helper values for Money, Currency, IBAN, BIC, Party, Agent,
  Account, and Postal Address MUST provide validation and conversion ergonomics
  without duplicating complete generated message structures.
- **FR-038**: Helper construction and conversion failures MUST use structured
  domain errors.
- **FR-039**: Builders for pacs.008, pacs.009, pacs.002, pain.001, pain.002,
  camt.052, camt.053, and camt.054 MUST produce generated values.
- **FR-040**: Builders MUST enforce their documented construction invariants and
  distinguish incomplete input, invalid helper values, and cross-field conflicts.
- **FR-041**: Every first-phase builder MUST include runnable successful and
  failing usage examples.

#### CLI and Adapter Consistency

- **FR-042**: The CLI MUST provide detect, inspect, validate, explain, to-JSON,
  to-XML, catalogue, and versions operations.
- **FR-043**: Every CLI operation MUST support a stable machine-readable output
  option in addition to its human-readable output where meaningful.
- **FR-044**: CLI exit statuses MUST stably distinguish success, validation
  failure, parse/input failure, and internal failure.
- **FR-045**: CLI, MCP, and browser adapters MUST return results derived from the
  core SDK rather than independently implemented parsing, validation, catalogue,
  migration, or conversion logic.
- **FR-046**: MCP MUST provide message detection, inspection, validation, message
  lookup, field lookup, version comparison, and validation-error explanation.
- **FR-047**: MCP default operation MUST require no external network access, shell
  execution, or message upload.
- **FR-048**: Browser support MUST explicitly list and test parsing,
  serialization, detection, catalogue, and basic validation capabilities.
- **FR-049**: Unsupported browser feature combinations MUST be documented and
  rejected as early as practical.

#### Migration and Version Comparison

- **FR-050**: MT103, MT202, and MT940 parsing MUST preserve recognized source
  fields and report malformed or unsupported constructs without panic.
- **FR-051**: MT103 migration MUST produce a generated pacs.008 value, MT202 a
  generated pacs.009 value, and MT940 a generated camt.053 value.
- **FR-052**: Every migration result MUST contain the generated message, mapping
  report, warnings, and unmapped fields.
- **FR-053**: Every source-field mapping MUST be classified as Exact, Derived,
  Lossy, Ambiguous, or Unsupported.
- **FR-054**: Migration MUST never silently omit a populated source field.
- **FR-055**: Every migration MUST have fixtures containing input MT, expected ISO
  20022 output, expected mapping report, and expected warnings or unmapped fields.
- **FR-056**: Message version comparison MUST use schema metadata and report added
  fields, removed fields, type changes, cardinality changes, and enum changes.
- **FR-057**: Version comparison MUST reject or explicitly classify comparisons
  across unrelated message families.

#### Privacy, Security, and Resilience

- **FR-058**: Default and debug logs MUST not contain complete messages or raw
  sensitive financial values.
- **FR-059**: Shared redaction MUST cover IBAN, account number, name, address,
  transaction reference, BIC, and payment detail values while preserving only the
  minimum characters needed for safe diagnosis.
- **FR-060**: Full-message logging MUST require an explicit unsafe opt-in and MUST
  communicate its disclosure risk.
- **FR-061**: XML processing MUST enforce documented input-size, nesting-depth,
  collection-size, and allocation controls.
- **FR-062**: XML processing MUST reject entity expansion and remain panic-free
  for malformed namespaces, malformed Unicode, and adversarial structure.
- **FR-063**: Fuzz verification MUST cover XML parsing, message detection,
  namespace detection, IBAN, BIC, selected message validators, and migration
  parsers.
- **FR-064**: Security verification MUST demonstrate no panic, crash, undefined
  behavior, or unbounded resource growth for the defined fuzz and adversarial
  corpus.

#### Performance, Packaging, Documentation, and Release

- **FR-065**: Performance baselines MUST cover pacs.008 parse, serialize, detect,
  and validate; camt.053 parse; and catalogue lookup.
- **FR-066**: Baselines MUST record latency and, where reliably measurable,
  allocations and peak memory, with environment context.
- **FR-067**: Build baselines MUST record default-feature compile time, artifact
  size, and dependency count.
- **FR-068**: Default installation MUST not include all profiles, CLI, MCP, and
  browser-specific capabilities.
- **FR-069**: Feature and platform documentation MUST identify supported
  combinations and their dependency or size implications.
- **FR-070**: The declared minimum supported language/toolchain version MUST be
  documented, verified in continuous integration, and changed only with release
  notes.
- **FR-071**: Public modules MUST have module-level documentation and core public
  capabilities MUST have runnable examples.
- **FR-072**: Documentation MUST cover generated APIs, high-level APIs,
  validation, profiles, CLI, migration, browser support, MCP, security limits,
  privacy defaults, known limitations, and the compliance disclaimer.
- **FR-073**: The project MUST maintain contribution, security, roadmap,
  sponsorship, changelog, and issue-reporting guidance for bugs, schemas,
  validation rules, profiles, and feature requests.
- **FR-074**: Release evidence MUST include supported message versions, supported
  profile releases, supported migrations, schema manifest, generator identity,
  minimum supported version, feature matrix, known limitations, and verification
  results.
- **FR-075**: The release process MUST produce release notes, published library
  documentation, distributable command-line artifacts where supported, and
  checksums, and MUST evaluate inclusion of a software bill of materials.
- **FR-076**: Supply-chain verification MUST include dependency vulnerability and
  license-policy checks whose maintenance cost is documented and accepted.
- **FR-077**: No work package may be marked complete while required behavior is a
  stub, placeholder, mock, always-success result, happy-path-only implementation,
  silent omission, or undocumented TODO.
- **FR-078**: Every work package MUST record its goal, scope, non-goals,
  dependencies, implementation outcome, tests, evidence, acceptance criteria, and
  exact verification results.

### Key Entities

- **Generated Message**: The sole canonical schema-derived ISO 20022 value,
  identified by message identifier, namespace, root element, and schema lineage.
- **Message Identity**: Business area, family, message number, variant, version,
  namespace, root element, and description associated with a message.
- **Schema Manifest Entry**: Schema identity, release/source provenance,
  cryptographic hash, generated coverage, and generation status.
- **Generator Manifest**: Generator version or commit, configuration identity,
  input manifest identity, and generation metadata.
- **Catalogue Entry**: Discoverable message identity and coverage information
  derived from generation metadata.
- **Validation Report**: Overall validity plus structured errors and warnings,
  separated by validation layer.
- **Validation Issue**: Stable code, severity, path, message, and rule ID, plus
  profile release and reference when applicable.
- **Validation Rule**: Versioned, explainable constraint with affected messages,
  paths, reason, and source.
- **Profile Release**: Named market/network release with supported messages and
  versions, rules, fixtures, references, and evidence.
- **Helper Value**: Validated ergonomic value used for conversion or builder
  input without representing a complete message model.
- **Mapping Result**: Generated target message plus classified source-field
  mappings, warnings, and unmapped fields.
- **Version Difference**: Schema-derived added, removed, type, cardinality, or
  enumeration change between versions of one message family.
- **Release Baseline**: Immutable record of compatibility, manifests, supported
  coverage, verification, benchmarks, feature/MSRV data, known limitations, and
  distributable artifact checksums.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of published compatibility-baseline examples and fixtures pass
  unchanged, or every exception is explicitly approved and accompanied by SemVer
  impact and migration guidance.
- **SC-002**: 100% of generated message modules are linked to a schema hash,
  schema release/source, generator identity, and configuration identity.
- **SC-003**: Three consecutive clean regenerations from identical recorded
  inputs produce zero unaccounted file differences.
- **SC-004**: Catalogue-to-generated and generated-to-catalogue consistency checks
  report zero unexplained mismatches across all 1,130 baseline message versions.
- **SC-005**: Every validation issue produced by the acceptance fixtures contains
  all required structured fields, and every important rule ID resolves to an
  explanation and source/reference.
- **SC-006**: All required financial validators pass their valid, invalid, and
  boundary suites with 100% expected-outcome agreement and complete the defined
  fuzz baseline without panic or crash.
- **SC-007**: CBPR+ 2026, SEPA SCT 2026, and SEPA SCT Inst 2026 each publish a
  supported-message matrix and achieve 100% expected rule/path agreement across
  their versioned valid and invalid acceptance fixtures.
- **SC-008**: All eight first-phase builders return generated values, reject every
  documented invalid or incomplete fixture with a structured error, and provide
  runnable successful and failing examples.
- **SC-009**: Each first-phase MT migration achieves 100% field-by-field agreement
  with its expected fixtures, and every populated source field appears in the
  target or mapping report.
- **SC-010**: CLI and MCP structured results match direct SDK results for 100% of
  shared adapter fixtures; CLI success, validation, parse, and internal outcomes
  use four distinct stable exit classes.
- **SC-011**: The adversarial corpus and agreed fuzz-duration baseline complete
  with zero panic, crash, undefined behavior finding, raw-message log disclosure,
  or configured resource-bound violation.
- **SC-012**: The defined runtime benchmarks and build/package measurements have a
  recorded baseline environment; subsequent release candidates explain every
  regression greater than 10% before approval.
- **SC-013**: Every documented browser-supported capability passes its target
  verification, and every unsupported feature combination is identified in the
  feature matrix.
- **SC-014**: 100% of public modules have module-level documentation, every core
  workflow has a runnable example, and all maintained documentation links and
  examples pass release verification.
- **SC-015**: A release candidate can be reconstructed and audited from its
  recorded schema/profile/generator identities, checksums, supported coverage,
  feature matrix, minimum supported version, known limitations, and verification
  evidence without relying on an unrecorded maintainer action.

## Assumptions

- Official ISO 20022 schema files already present in the repository are the
  baseline inputs; missing official schemas remain explicitly reported until
  lawfully obtained and verified.
- Authoritative CBPR+ and SEPA 2026 rule material and redistribution rights will
  be available to maintainers before those profile work packages can be accepted.
- Existing generated modules and current published behavior form the initial
  compatibility baseline even where documentation is incomplete.
- Generated message coverage begins at the repository's current 1,130-version
  baseline; later schema updates are separate, explicitly versioned changes.
- Resource limits and fuzz durations will be selected from measured baseline
  behavior during planning, then treated as acceptance thresholds rather than
  left unspecified.
- “Latest” catalogue lookup means the greatest supported version according to
  normalized message-version ordering, not the most recently downloaded file.
- Browser support refers to the documented local capabilities and does not imply
  network fetching from browser code.
- MCP processes message content locally and does not persist it by default.
- Benchmark results are comparable only when their environment and input fixture
  identity are recorded.
- Enterprise sponsorship funds ongoing open standards maintenance and does not
  unlock a separate closed-source core feature set.
