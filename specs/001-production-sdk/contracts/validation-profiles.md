# Contract: Validation and Profiles

## Report Shape

```rust
pub struct ValidationReport {
    pub valid: bool,
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
    pub executed_layers: Vec<ValidationLayer>,
    pub unavailable_layers: Vec<UnavailableLayer>,
    pub profile_releases: Vec<ProfileReleaseKey>,
}

pub struct ValidationIssue {
    pub code: IssueCode,
    pub severity: Severity,
    pub path: FieldPath,
    pub message: SafeMessage,
    pub rule_id: RuleId,
    pub layer: ValidationLayer,
    pub profile: Option<ProfileReleaseKey>,
    pub source: Option<RuleSource>,
}
```

`valid` is derived: it is false if errors exist or a requested required layer is
unavailable. Results are ordered by layer, path, and rule ID. JSON uses stable
snake_case field names and explicit schema versioning.

## Rule Contract

- A rule evaluates a borrowed generated value plus immutable
  `ValidationContext`.
- Rule registration couples executable logic and `RuleDescriptor` in one source.
- Rule IDs are stable, unique, and namespaced (for example
  `CBPRPLUS-<release>-R1234`); they are never reused for changed semantics.
- Cross-field rules use typed combinators for required-if, xor, at-least-one,
  cardinality dependency, and field relation.
- An issue contains a logical ISO path and safe explanation, not a raw value.
- `explain(rule_id)` returns the exact descriptor used by validation.

## Layer Semantics

- L1 `SyntaxSchema`: bounded well-formed XML, namespace/root, and offline
  manifested XSD validation. Deserialization success alone is not L1 success.
- L2 `IsoSemantic`: financial identifier/value and ISO semantic/cross-field
  rules evaluated on generated structs.
- L3 `Profile`: market/network rules bound to one immutable resolved release.

Callers may choose fail-fast, but the default collects all safely evaluable issues
within resource limits. A failed earlier layer clearly marks dependent layers as
not executed; it never relabels the failure.

## Profile Contract

```text
ProfileReleaseKey =
  scheme
  + scheme_release
  + guideline_edition
  + effective_from/effective_until
  + source_digests
```

- Releases and rules are append-only and addressable forever.
- `latest(as_of)` is only a resolver and returns a concrete key.
- Supported messages/versions, rule set, fixtures, and evidence belong to each
  concrete release.
- SEPA rulebook and implementation-guideline editions remain distinct.
- A profile cannot be labelled production-complete before its source/license gate
  and expected-valid/invalid fixture matrix pass.

## Compliance Wording

Reports and documentation say “valid according to the implemented rules and
identified releases.” They never promise bank acceptance, regulatory
certification, network onboarding, or legal compliance.

