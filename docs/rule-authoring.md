# Validation rule authoring

Rules validate borrowed projections of canonical generated structs. A rule may
use helper values and `ValidationTarget`, but it must not introduce a second
long-lived ISO 20022 message model.

## Stable identity and explanation

Every executable rule owns one immutable `RuleDescriptor`. The descriptor and
the executable object are registered together in `RuleRegistry`, so `validate`
and `explain` cannot drift apart.

Rule IDs are permanent, uppercase identifiers. Core semantic rules use the
`ISO20022-L2-...` prefix. Profile rules include the profile and explicit
release, for example `CBPRPLUS-2026-R1234`. Do not reuse an ID after changing
its meaning.

Each descriptor must record:

- a concise title and value-free reason;
- the validation layer;
- authoritative source or schema-binding reference;
- affected message identifiers;
- for L3, both profile name and exact profile release.

Issue paths use schema logical names such as
`Document/FIToFICstmrCdtTrf/GrpHdr/NbOfTxs`. Messages must explain the
constraint without interpolating account numbers, names, references, XML, or
other source values.

## Reusable relationship rules

`validation::cross_field` provides typed combinators for:

| Relationship | Type |
|---|---|
| conditional required | `RequiredIfRule` + `FieldPredicate` |
| mutual exclusion / XOR | `MutualExclusionRule` |
| at least one | `AtLeastOneRule` |
| cardinality dependency | `CardinalityDependencyRule` |
| general two-field relation | `FieldRelationshipRule` |
| currency and amount | `CurrencyAmountRule` |
| agent and account | `AgentAccountRule` |

The API is deliberately typed Rust rather than a string rule DSL. Rust types,
normal review, rustdoc, and compiler errors keep predicates auditable; a custom
DSL would add a parser, an extra compatibility surface, and another place for
paths and type semantics to diverge from generated metadata.

## Generated bindings

A binding borrows fields directly from one generated `Document`, attaches
schema logical paths, and executes common combinators. It must:

1. name the exact message identifier and generated module;
2. audit every selected wire field against the schema-derived
   `MessageDescriptor`;
3. handle each repeated transaction independently rather than allowing a field
   from one transaction to satisfy another;
4. return `ValidationReport` (or a typed setup error), never `bool`, `String`,
   or a panic;
5. keep all diagnostic text free of source values.

`validation::bindings::validate_pacs_008_001_08` is the first direct-generated
example. Its fixtures cover conditional presence, account-ID XOR, declared
transaction cardinality, currency/amount precision, and debtor agent/account
relationships. The binding descriptor exposes the schema metadata used to
audit its field selection.

## Test and evidence requirements

For every new combinator or binding, add:

- a complete truth table, including absent and boundary states;
- deterministic multi-issue ordering assertions;
- a canary proving values are absent from `Display` and `Debug` diagnostics;
- valid and invalid generated-message fixtures;
- for each invalid fixture, expected rule IDs and field paths;
- source/release metadata and closure commands in the work-package evidence.

Profile rules additionally require licensed source review, versioned fixtures,
and evidence tied to one explicit profile release. Passing implemented rules
does not imply bank acceptance, regulatory certification, network onboarding,
or legal compliance.
