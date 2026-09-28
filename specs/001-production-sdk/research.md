# Phase 0 Research: Production-Grade ISO 20022 SDK

**Feature**: `001-production-sdk`  
**Date**: 2026-09-27  
**Status**: Complete for planning; four explicit research/licensing gates remain before the affected implementation claims may be accepted.

## Repository Evidence

- The repository contains 1,130 XSD inputs, 32 generated business-area directories,
  and 1,130 generated message-version modules. Generated structs are already the
  public model and remain the sole canonical ISO 20022 representation.
- The current generator sorts inputs and uses ordered maps, but its `xsd-parser`
  dependency is a floating Git dependency. The ignored lockfile happens to pin
  commit `d476e854...`; no tracked schema manifest, generator manifest, input
  hashes, output digest, or transactional regeneration check exists.
- `src/validate.rs` implements only generated XSD facets through
  `Validate -> Result<(), String>` and defaults to success. It is a compatibility
  surface, not the production validation report required by this feature.
- Detection and XML scanning currently use string scanning without shared input,
  depth, event, or collection limits. The CLI only queries a catalogue, and the
  WASM layer uses a custom `direct_wasm` cfg and manually assembled JSON.
- There is no profile engine, migration subsystem, MCP server, fuzz workspace,
  benchmark baseline, or release CI. Existing gated tests can report zero tests
  unless their required feature set is selected explicitly.

## R-01: Compatibility Baseline and Workspace Boundary

**Decision**: Freeze a machine-readable compatibility baseline before changing
code generation or public surfaces. Capture public Rust API, generated paths,
features, XML/JSON golden output, detection and metadata results, WASM exports,
and downstream compile fixtures. Compare both the published `0.1.1` package and
the pending repository state. Use `cargo-public-api`, `cargo-semver-checks`,
rustdoc JSON where practical, and explicit fixtures rather than treating any one
tool as authoritative.

The root `rust_iso20022` package remains the compatibility façade, core SDK, and
home of generated types. Add workspace members for codegen, migration, CLI, MCP,
WASM, fuzzing, and benchmarks only when their WP begins. Message/catalogue,
validation, builders, profiles, and version comparison remain in the root crate
because they directly operate on generated structs.

**Rationale**: Existing exhaustive public enums (`Error`, `BusinessArea`),
public-field structs (`CatalogueEntry`, `MxId`), and the externally implementable
`MxMessage` trait make apparently additive field/variant/required-item changes
breaking. New capabilities therefore use new types and wrappers.

**Alternatives rejected**: Rewriting the root crate around a new high-level
message DTO would break generated paths and violate the canonical-model rule.
Putting every adapter and profile in the default root feature would inflate all
consumers.

**Risks**: The published baseline and current working tree differ. Evidence must
name which baseline it measures, and schema-driven changes must be separated from
hand-written API changes.

## R-02: Reproducible Schema and Generator Provenance

**Decision**: Introduce versioned `xsds/schema-set.json` and
`xsds/schema-manifest.json`. Each schema record includes message identity,
relative path, source/release status, source URL when known, raw SHA-256, byte
length, actual namespace, root element/type, generated module, and generation
status. Legacy sources with unknown provenance are explicitly marked
`unverified`; provenance is never invented.

Track `rust-toolchain.toml`, the workspace `Cargo.lock`,
`tools/codegen/generator-config.toml`, and
`tools/codegen/generator-manifest.json`. Pin `xsd-parser` to an immutable revision.
The generator manifest records generator version/source digest, upstream parser
revision, lock/config/toolchain digests, and metadata-format versions.

Generation becomes strict and transactional: validate all inputs, generate into
a temporary tree, require complete schema/output coverage, write deterministic
metadata and output hashes, then compare or atomically install. Unknown
arguments, duplicate identities, empty namespaces/roots, hash mismatch, stale
outputs, and any per-file failure return nonzero. Timestamps, hostnames, absolute
paths, and directory iteration order never influence output. `--only` is a
diagnostic temporary-tree mode and cannot rewrite the global catalogue.

The existing exceptions are recorded rather than hidden:
`acmt.017.001.03_0.xsd` needs normalized identity;
`semt.001.001.04` carries a `urn:swift:xsd:` namespace; root elements comprise
1,127 `Document`, two `AppHdr`, and one `Xchg`.

**Rationale**: Schema + pinned generator + declared configuration must reproduce
the same bytes and answer the origin of every generated type.

**Alternatives rejected**: Relying on an ignored lockfile, generated comments, or
manual output patches cannot establish provenance or reproducibility.

**Risks**: A clean regeneration may expose historical transform bugs and large
generated diffs. The compatibility baseline must precede installation of those
diffs.

## R-03: Schema-Derived Metadata, Catalogue, and Version Diff

**Decision**: Codegen emits a read-only `SchemaDescriptor` intermediate
representation containing identity, provenance, namespace/root, description,
dispatch type, fields, wire names/QNames, logical paths, types, cardinality,
enums, and documentation. It describes schemas; it is not a runtime message model.
It is the sole source for `MessageDescriptor`, catalogue indices, field lookup,
rule-path mapping, and version comparison.

Version comparison uses a normalized semantic field graph keyed by message
family and numeric version. It reports added/removed fields, type changes,
cardinality changes, and enum-value additions/removals, with both schema hashes.
It does not infer renames without explicit evidence and never uses XML text diff.

**Rationale**: Generated code and all discovery APIs remain co-derived. The
existing `CatalogueEntry` is retained for compatibility rather than enlarged.

**Alternatives rejected**: A hand-maintained message list or reflection over Rust
source would drift from schemas. XML text diff confuses formatting with semantic
change.

**Risks**: Stable logical paths must bridge generated Rust names and ISO paths.
Their mapping requires codegen golden and bidirectional-coverage tests.

## R-04: Unified Message Abstraction

**Decision**: Add `ParsedMessage`, `MessageDescriptor`, and `MessageRef` as
additive APIs while leaving `MxMessage` unchanged. Codegen provides an
`AnyMessage` dispatch enum for supported generated messages. Parsed wrappers
expose identifier, area, family, number, version, namespace, root, description,
serialization, and validation while preserving `as_generated`/`into_generated`
access to the concrete generated value.

**Rationale**: A wrapper over generated values enables uniform behavior without
creating a parallel canonical message hierarchy.

**Alternatives rejected**: `HighLevelPacs008`-style DTOs require permanent
two-model synchronization. Adding required trait items to `MxMessage` breaks
external implementors.

**Risks**: Compiling a dispatch enum for all 1,130 types is expensive. Dispatch
must honor business-area feature gates and the minimal default build.

## R-05: Three-Layer Validation and Explainable Rules

**Decision**: Fix the pipeline as L1 `SyntaxSchema`, L2 `IsoSemantic`, and L3
`Profile`. All layers accumulate into `ValidationReport { valid, errors,
warnings }`. Each `ValidationIssue` contains code, severity, logical field path,
message template, rule ID, and layer; profile issues also carry profile release
and source reference. L1 failure is never presented as semantic/profile failure.

`Rule<TGenerated>` and `RuleSet<TGenerated>` receive the generated `Document`
and immutable `ValidationContext`. Reusable typed combinators implement
required-if, mutual exclusion, at-least-one, cardinality dependency, and field
relationships. Each evaluator and immutable `RuleDescriptor` are registered from
one definition. Results sort deterministically by layer, path, and rule ID.
`explain(rule_id)`, CLI, and MCP query the same registry. Issues do not contain
raw field values by default.

Keep the old generated `Validate -> Result<(), String>` as a compatibility facet
API; do not mislabel it as the new engine.

**Rationale**: ISO describes XML Schema as syntax validation, while market usage
guidelines add multiplicity and conditional rules beyond the base schema. A
single descriptor/evaluator registry makes outcomes auditable. Primary context:
[ISO catalogue](https://www.iso20022.org/understanding-iso-20022-business-process-catalogue),
[SWIFT usage-guideline practices](https://www2.swift.com/knowledgecentre/rest/v1/publications/mystds_usg_guid_edit_best_prac/1.0/mystds_usg_guid_edit_best_prac.pdf),
and [EPC SCT IG](https://www.europeanpaymentscouncil.eu/sites/default/files/kb/file/2024-11/EPC115-06%20SCT%20Inter-PSP%20IG%202025%20V1.0.pdf).

**Alternatives rejected**: Boolean/string-only results, closure-only rules,
string DSLs, and message-specific if/else cannot provide stable audit evidence.

**Risks**: A complete, offline Rust XSD backend with required MSRV, resource
limits, and WASM properties is not yet proven; see gate `R-XSD-RUST`.

## R-06: Profile Release Identity and Source Rights

**Decision**: A profile release key contains scheme, scheme release, guideline
edition, effective range, and source digests. Rule sets are append-only; old
releases remain reproducible. `latest(as_of)` resolves to a concrete key, and
evidence always records that resolved key. SEPA rulebook and implementation-
guideline versions remain separate. “2026” is not accepted as sufficient release
identity.

ISO base schemas enter the reproducible schema pipeline. Profile definitions
store provenance, narrow references, independently implemented rules, and tests;
copyrighted source documents or complete third-party rule tables are not copied
without explicit rights. As of 2026-09-27, the EPC identifies SCT 2025 Rulebook
v1.1 as current through 2027-11-21; its 2026 material is consultation material,
not an implemented release. CBPR+ must anchor to an authoritative SWIFT final
usage-guideline edition and standards-release lifecycle.

**Rationale**: Releases can receive patches without changing the ISO message
version. Digests and effective dates are required for reproduction. Sources:
[ISO maintenance](https://www.iso20022.org/iso-20022-standard-maintenance-process),
[SWIFT releases](https://www.swift.com/standards/standards-releases), and
[EPC current SCT release](https://www.europeanpaymentscouncil.eu/what-we-do/epc-payment-schemes/sepa-credit-transfer-sct/sepa-credit-transfer-rulebook-and).

**Alternatives rejected**: A bare `Profile::CbprPlus`, year-only keys, scraped
MyStandards content, or synthetic placeholder rules cannot support reproducible
production claims.

**Risks / gates**:

- `R-LEGAL-CBPR`: obtain authoritative source access and evidence that the rules
  may be implemented and the necessary derived metadata distributed.
- `R-LEGAL-SEPA`: record a licensing review for derived open-source rule data.

These gates do not block the core profile framework, but they block claims of
complete CBPR+/SEPA production coverage. ISO itself states that there is no
official ISO 20022 certification authority.

## R-07: Financial Helper Validation and Reference Data

**Decision**: Separate format/checksum validation from registry/existence
validation. IBAN uses structure, MOD-97, and a versioned country-length snapshot.
BIC/BICFI validates ISO 9362 syntax and country/location shape but does not claim
directory existence. Currency and country validators use pinned, licensed code-
list snapshots. LEI uses format/checksum; future existence checks require a
separate optional offline dataset. Amount, precision, date/datetime, account, and
clearing identifiers use typed errors and documented scopes.

Every snapshot records publisher, release, source URL, digest, and license note.
The BIC Directory is not bundled. Registry updates are reviewable data changes.

**Rationale**: Offline snapshots are reproducible and privacy preserving.
[SWIFT IBAN registry](https://www.swift.com/standards/data-standards/iban-international-bank-account-number),
[SIX ISO 4217](https://www.six-group.com/en/products-services/financial-information/market-reference-data/data-standards.html),
and [GLEIF terms](https://www.gleif.org/en/meta/lei-data-terms-of-use) identify
available authoritative sources.

**Alternatives rejected**: Runtime network lookup is non-reproducible and leaks
queries. Copying arbitrary third-party lists creates accuracy and licensing risk.

**Risks**: “Free use” is not automatically permission to redistribute a complete
collection. Each embedded snapshot requires a recorded licensing decision.

## R-08: Hardened XML Boundary and Privacy

**Decision**: Every XML entry point first uses a shared streaming
`BoundedXmlReader`. `ParseLimits` limits input bytes, nesting depth, events,
elements, attributes per element, text bytes, collection occurrences, and total
decoded bytes. Reject DTD/DOCTYPE, external entities, XInclude, external
`schemaLocation` fetching, malformed tag structure, namespace/root errors, and
invalid encoding with typed errors and no panic. XSDs load only from a trusted
manifested local bundle. Defaults are fixed; callers may explicitly lower or
raise limits.

Core defines a data-class-aware `RedactionPolicy`. Default logs, Debug output,
CLI diagnostics, MCP errors, and panic-safe adapters omit raw XML/JSON and
sensitive field values. Unsafe full-payload logging requires both a compile-time
feature and explicit runtime opt-in. Tests use canary values to detect leakage.

**Rationale**: OWASP recommends disabling DTD, external entities/loading, and
XInclude and imposing resource limits. Data minimisation and confidentiality are
privacy-by-default requirements. Sources: [OWASP XXE guidance](https://cheatsheetseries.owasp.org/cheatsheets/XML_External_Entity_Prevention_Cheat_Sheet.html)
and [GDPR text](https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32016R0679).

**Alternatives rejected**: Regex/substring DOCTYPE checks, parser defaults, and
caller-owned redaction leave inconsistent attack and disclosure surfaces.

**Risks / gates**: `R-PARSER` must prove that the streaming gate covers every
entry point and define UTF-16 behavior. Double parsing may cost latency and must
be benchmarked.

## R-09: Builders, Migration, and Thin Adapters

**Decision**: Builders use validated helper values, typed builder errors, and
return the relevant generated `Document`. They cover only the eight specified
message families in phase one. Migration parsers may use transient MT syntax IR,
but every successful conversion yields a generated document plus a field-level
mapping report classifying Exact, Derived, Lossy, Ambiguous, or Unsupported.
Nothing is silently discarded.

CLI, MCP, and WASM call core request/result APIs. CLI provides stable JSON schemas
and exit codes (0 success; distinct validation, parse, and internal failures).
MCP v1 uses local stdio/in-memory inputs with no file, network, shell, or upload
capability. WASM uses typed serde bindings for the verified subset and preserves
legacy exports through compatibility shims. Native and WASM share contract
fixtures and parsing limits.

**Rationale**: All entry points must return identical domain results. MCP tool
annotations are hints, not a security boundary; capabilities must be absent from
the process design. Primary references: [MCP server primitives](https://modelcontextprotocol.io/specification/draft/server/index)
and [wasm-bindgen](https://github.com/wasm-bindgen/wasm-bindgen).

**Alternatives rejected**: Adapter-local parsers/rules, human table output as a
machine protocol, file/URL-taking MCP tools, and hand-built JSON all drift.

**Risks**: Full XSD validation may be unavailable on WASM; unsupported layers
must be feature-gated and explicitly reported, never silently skipped.

## R-10: Feature, Test, Fuzz, Benchmark, and Release Strategy

**Decision**: Preserve current feature names and minimal `default = []`. Add
`json`, validation, builder-family, profile-family, version-comparison, and
schema-fetch features as dependencies require. Existing names become compatibility
aliases for at least one release cycle. XML cannot be made optional until the
unconditional `MxMessage`/yaserde ABI is deliberately migrated.

The test matrix explicitly selects default, JSON, every model area, high-value
feature combinations, CLI, WASM, MSRV, codegen reproducibility, security,
profiles, and migrations. A separate `fuzz/` workspace uses cargo-fuzz targets
for XML, detection, identifiers, selected validators, and MT parsing/conversion;
nightly fuzzing is outside the normal MSRV gate. A benchmark crate records the
six required workloads with commit, toolchain, OS/CPU, features, and fixture
digests. Latency, allocation, and peak memory are reported only by tools that
measure those quantities.

Release evidence includes workspace build/test/clippy/fmt/diff checks, adapter
smoke tests, WASM checks, fixture matrices, fuzz/benchmark baselines, compile
time/dependency/binary-size baselines, supported versions/releases/migrations,
MSRV, feature matrix, known limitations, audit/licenses/lockfile, docs.rs,
checksums, and practical SBOM evaluation.

**Rationale**: Feature-gated zero-test runs and noisy ad-hoc timing do not prove
coverage or performance. Rust fuzzing constraints are documented by the
[Rust Fuzz Book](https://rust-fuzz.github.io/book/) and
[cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz).

**Alternatives rejected**: `default = everything`, per-PR unbounded fuzzing, and
single wall-clock samples are costly without reliable evidence.

**Risks**: Benchmark comparisons require equivalent runners; fuzz timeout/OOM is
a failure even without a panic. Financial fixtures must be synthetic or
irreversibly anonymized.

## Required Pre-Claim Gates

| Gate | Must be resolved before | Required evidence |
|---|---|---|
| `R-XSD-RUST` | Claiming full L1 XSD validation | Coverage spike, no-network behavior, limits, MSRV, native/WASM decision |
| `R-PARSER` | Accepting hardened XML WP | All entry points use one bounded gate; encoding policy and attack corpus results |
| `R-LEGAL-CBPR` | Claiming CBPR+ rule coverage | Authoritative release sources plus access/implementation/distribution decision |
| `R-LEGAL-SEPA` | Publishing derived SEPA rule database | Rulebook/IG version and recorded redistribution/derivation review |

Framework and unrelated SDK work continue while a gate is open. Acceptance
criteria for the gated claim remain incomplete until its evidence exists.
