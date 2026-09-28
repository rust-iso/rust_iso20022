# Message version comparison

`rust_iso20022::compare` computes deterministic semantic changes from the
normalized field graph generated from XSD metadata. It never compares instance
XML or raw XSD text.

```rust
use rust_iso20022::compare::{SchemaChangeKind, compare_versions};

let diff = compare_versions("pacs.008.001.08", "pacs.008.001.10")?;
assert_eq!(diff.from.identity, "pacs.008.001.08");
assert_eq!(diff.to.identity, "pacs.008.001.10");
assert!(diff.changes.iter().any(|change| {
    change.kind == SchemaChangeKind::TypeChanged
        && change.path == "Document/FIToFICstmrCdtTrf"
}));
# Ok::<(), rust_iso20022::compare::CompareError>(())
```

## Contract

Both endpoints contain the exact message identity and SHA-256 of the source XSD.
Changes are ordered by logical path, change kind, and before/after value. The six
supported classes are:

- field added;
- field removed;
- type changed;
- cardinality changed;
- enumeration value added;
- enumeration value removed.

Only versions of the same three-component family are comparable. Unknown IDs
and unrelated families return typed errors. Comparing a descriptor with an
identical normalized field graph produces no changes even when the source XSD
bytes, formatting, namespace version, and schema hash differ.

`None` for `max_occurs` means unbounded. Cardinalities are rendered as
`min..max`, for example `0..1` or `0..unbounded`. Added/removed fields include a
compact type and cardinality shape; type, cardinality, and enum changes carry
explicit before/after values.

## Rename boundary

The comparator intentionally does not infer renames. Codegen logical paths are
rooted in schema type/declaration identities. If a schema release replaces a
type such as `CreditTransferTransaction39` with `CreditTransferTransaction73`,
the declarations under those distinct paths are reported as removed and added,
not guessed to be renames. This is conservative and reproducible.

For the repository's available `pacs.008.001.08` → `.14` schemas, the current
metadata yields 282 added fields, 251 removed fields, and one type change. Those
real versions contain no same-path cardinality or enum changes. All six change
algorithms are therefore also covered by a miniature normalized-graph fixture;
the project does not fabricate absent changes in the official schemas.

## CLI

The CLI is a direct serialization of the core `VersionDiff`:

```console
iso20022 compare pacs.008.001.08 pacs.008.001.10
iso20022 compare --json pacs.008.001.08 pacs.008.001.10
```

JSON uses the common versioned envelope. Unknown messages return exit 5 with
`unknown_message`; unrelated families return usage exit 2 with
`unrelated_families`. Diagnostics do not echo supplied identifiers.
