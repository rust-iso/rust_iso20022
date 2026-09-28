# Data Model: Production-Grade ISO 20022 SDK

This document defines conceptual domain entities and invariants. Rust spellings
may be refined during a WP, but the ownership and canonical-model boundaries are
normative.

## Canonical-Model Boundary

`generated::<area>::<message>::Document` and its nested generated structs are the
only canonical message data. The following entities are descriptors, helpers,
reports, contexts, adapters, or transient parser state. None may duplicate a
complete ISO 20022 message for long-term storage or independent serialization.

## Provenance Entities

### SchemaSet

- `format_version`
- `schema_release` or explicit `unverified` status
- ordered `schemas: Vec<SchemaRecord>`
- aggregate digest

Invariant: records are uniquely keyed by normalized message identity and relative
path; aggregate digest is computed from canonical record order and content.

### SchemaRecord

- normalized identifier parts: area, functionality, variant, version
- relative source path and raw SHA-256/byte length
- source URL, publisher, release/publication timestamp, provenance status
- actual target namespace and root element/type
- generated module path and generation status

Invariant: every input XSD has exactly one record; unknown provenance is explicit.

### GeneratorManifest

- manifest/config format versions
- generator crate version and source-tree digest
- pinned upstream parser revision
- toolchain, lockfile, and configuration digests
- ordered output records and aggregate digest

Invariant: it contains no machine-specific absolute paths, hostnames, or wall-clock
generation timestamps that change output for identical inputs.

### SchemaDescriptor

- message identity and schema provenance reference
- namespace, root, description, generated type/module
- ordered field graph with wire QName, logical path, Rust accessor metadata,
  type identity, cardinality, enums, and documentation

Invariant: generated from schema/codegen inputs only. It is immutable descriptive
metadata, never a message instance.

## Message and Catalogue Entities

### MessageIdentifier

- business area
- message family/functionality
- variant/message number
- version
- canonical dotted string

Invariant: parse/format round-trips; numeric parts retain width for wire identity
and numeric values for version ordering.

### MessageDescriptor

- `MessageIdentifier`
- business area/family/number/version
- namespace and root element
- description
- generated availability/required feature
- schema record/digest reference

Invariant: one descriptor per generated message version and no hand-maintained
descriptor without a matching schema record.

### MessageCatalogue

Read-only indices over descriptors:

- by full identifier
- by family
- ordered versions
- latest version
- by namespace
- by `(namespace, root)`

Invariant: `latest` is numeric, deterministic, and scoped by family; every index
points to the same immutable descriptor instance.

### AnyMessage / ParsedMessage / MessageRef

- owns or borrows exactly one concrete generated message variant
- retains its descriptor and parse context
- exposes uniform metadata, serialization, and validation
- provides safe typed access/downcast and ownership recovery

Invariant: no fields are copied into a parallel message DTO. Serialization
delegates to the generated value.

## Parsing and Privacy Entities

### ParseLimits

- maximum input bytes
- maximum nesting depth
- maximum XML events/elements
- maximum attributes per element
- maximum text bytes and total decoded bytes
- maximum collection occurrence

Invariant: all limits are finite by default. Raising a limit is explicit at the
call site; adapter defaults equal core defaults.

### ParseContext / ParseError

Context contains limits, expected root/namespace if known, and encoding policy.
Typed errors classify limit, forbidden construct, encoding, XML well-formedness,
namespace/root detection, deserialization, and schema-validation failures.

Invariant: errors do not embed raw payload or sensitive field values.

### RedactionPolicy

- sensitivity class mapping
- mask strategy per identifier/value class
- safe diagnostic fields
- unsafe full-payload opt-in state

Invariant: the default policy is safe and cannot emit a full message. Unsafe
logging requires compile-time and runtime opt-in.

## Validation Entities

### ValidationLayer

`SyntaxSchema`, `IsoSemantic`, or `Profile`.

### ValidationIssue

- stable issue `code`
- `severity` (`Error` or `Warning` initially)
- logical `path`
- safe message template
- stable `rule_id`
- layer
- optional resolved profile release and source reference

Invariant: issue messages contain no raw sensitive value by default. Profile
issues always identify their exact resolved release.

### ValidationReport

- `valid`
- ordered `errors`
- ordered `warnings`
- message descriptor
- executed layers and unavailable/skipped-layer reasons
- resolved profile release(s)

Invariant: `valid` is derived from error presence and required-layer availability,
not independently mutable. Results sort by layer, path, and rule ID. An unavailable
required layer cannot silently produce `valid = true`.

### RuleDescriptor

- rule ID, layer, default severity
- message/version selector and logical field path
- reason and safe explanation
- source document ID/version/section/URL/digest
- optional profile release key

Invariant: each executable important rule has exactly one descriptor in the same
registry used by validation and explanation.

### Rule / RuleSet / ValidationContext

A rule borrows a generated value plus immutable context and emits zero or more
issues. A rule set is immutable and version-addressed. Context supplies reference
data snapshots, date/as-of settings, selected layers, and limits; it is not a
message representation.

### ProfileReleaseKey

- scheme (`CbprPlus`, `SepaSct`, `SepaSctInst`, extensible without mutating old
  public exhaustive compatibility enums)
- scheme release
- guideline edition
- effective-from/effective-until
- ordered source digests

Invariant: immutable and append-only. `latest(as_of)` resolves to one concrete key.

## Helper and Builder Entities

### Validated Helper Values

`Iban`, `Bic`, `Currency`, `CountryCode`, `Lei`, `Money`, `Amount`, `AccountId`,
`ClearingId`, `Party`, `Agent`, `Account`, and `PostalAddress` provide scoped
validation and conversion into generated fields.

Invariant: helpers contain only the value/context needed for ergonomics and
validation. They never reproduce a complete generated message.

### Builder

Message-specific construction state for one supported generated `Document`, with
typed required inputs and `BuilderError`.

State transition:

```text
empty/partial builder → typed field setters → build-time invariant validation
                      → generated::<...>::Document
```

Invariant: `build()` never returns a parallel high-level message. Required inputs,
conversion failures, and cross-field construction errors are typed.

## Migration Entities

### MtDocument (Transient)

A bounded parser representation of MT blocks/tags retaining source spans and all
recognized/unrecognized fields until mapping completes.

Invariant: it is not an ISO 20022 model, is not exposed as an alternative SDK
message, and cannot silently discard an input field.

### MappingEntry / MappingReport

- source tag/path/span identifier
- destination logical ISO path, if any
- classification: `Exact`, `Derived`, `Lossy`, `Ambiguous`, `Unsupported`
- safe rationale/warning (redacted)

The report includes all entries, warnings, and unmapped fields.

### MigrationResult<TGenerated>

- generated target message
- mapping report
- warnings/unmapped fields (or normalized views into the report)

Invariant: success requires accounting for every parsed source field; it does not
imply losslessness.

## Version-Comparison Entities

### VersionDiff

- from/to message descriptors and schema digests
- ordered semantic changes

### SchemaChange

- logical path
- change kind: added field, removed field, type changed, cardinality changed,
  enum value added, enum value removed
- before/after normalized values

Invariant: computed from schema descriptors; no XML text comparison and no
unsubstantiated rename inference.

## Adapter Contracts

CLI, MCP, and WASM DTOs are serialized views of core request/result entities.
They may add protocol envelope fields (schema version, request ID, exit status)
but may not create independent parsing, rule, catalogue, migration, or redaction
logic.
