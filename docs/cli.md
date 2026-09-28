# Command-line interface

The `iso20022` binary is a thin adapter over the `rust_iso20022` core SDK. It
does not maintain its own message catalogue, XML/JSON parser, or validation
rules. Generated message types remain the canonical ISO 20022 data model.

Build the standalone CLI with the capabilities needed by the command. For the
currently verified validation and serialization slice:

```console
cargo build -p rust_iso20022_cli --features json,model-pacs,profiles
```

Every command that reads a document accepts a file path or `-` for standard
input. Input is bounded before processing. The CLI never writes source message
content to logs or error messages.

## Detection and inspection

Detect a message from its namespace and root element:

```console
iso20022 detect payment.xml
iso20022 detect --json - < payment.xml
```

Inspect its generated-code descriptor:

```console
iso20022 inspect --json payment.xml
```

The JSON response is a versioned envelope. Its top-level fields are
`schema_version`, `command`, `status`, and either `data` or `error`.

## Serialization

Convert XML into the generated model's JSON representation, or deserialize
JSON into an explicitly selected generated message type and serialize it as
XML:

```console
iso20022 to-json --json payment.xml
iso20022 to-xml pacs.008.001.08 --json payment.json
```

These operations require the `json` feature and the applicable `model-*`
feature. A missing compile-time capability returns exit 5 instead of silently
using a different representation.

## Validation

Run the implemented ISO 20022 semantic rules (L2):

```console
iso20022 validate --layer l2 payment.xml
iso20022 validate --json --layer l2 - < payment.xml
```

The JSON result contains the detected `message_id`, the executed layer, and a
`ValidationReport` with `valid`, `complete`, `errors`, `warnings`, and
`unavailable_layers`. Invalid documents return the same report with exit 4, so
scripts do not need to parse human-readable output.

L1 offline XSD validation is not implemented yet and returns exit 5. Profile
(L3) validation never selects a release implicitly. `--profile` accepts only a
complete immutable release key:

```text
scheme|scheme_release|guideline_edition|effective_from|effective_until|sha256[,sha256]
```

For example, a syntactically complete but uninstalled release returns exit 5:

```console
iso20022 validate --json \
  --profile 'test-network|2025.1|ig-2025.1|2025-01-01|2025-12-31|aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa' \
  payment.xml
```

Short aliases such as `cbpr-plus:2026` are rejected with exit 2 because they do
not identify a reproducible rule release. See [profiles.md](profiles.md) and
[validation.md](validation.md) for the release and report contracts.

## Catalogue and versions

Catalogue queries are direct views of code-generated core metadata:

```console
iso20022 catalog pacs.008.001
iso20022 catalog --json pacs.008.001
iso20022 versions --json pacs.008.001
```

No message list is copied into the CLI. An empty result is represented by an
empty array and exit 0.

## Explain a validation rule

`explain` returns the same immutable descriptor used by core rule execution:

```console
iso20022 explain ISO20022-L2-IBAN-CHECKSUM
iso20022 explain --json ISO20022-L2-IBAN-CHECKSUM
```

The descriptor includes its rule ID, validation layer, title, reason,
source/reference, profile and exact profile version when applicable, affected
message IDs, and logical field paths. The CLI does not copy these fields into a
separate explanation database.

Profile explanations become available only from an installed exact-release
rule set. The generic adapter preserves L3 `profile`, `profile_version`,
`source`, `affected_messages`, and `field_paths`; this release does not claim
that gated CBPR+ or SEPA rule packs are installed. An unknown or unavailable
rule returns exit 5 and the stable JSON code `rule_not_found` without echoing
the requested value.

## Compare message versions

`compare` returns the core schema-metadata diff for two exact versions of the
same message family:

```console
iso20022 compare pacs.008.001.08 pacs.008.001.10
iso20022 compare --json pacs.008.001.08 pacs.008.001.10
```

The response pins both source schema hashes and lists deterministically ordered
field additions/removals, type changes, cardinality changes, and enum value
changes. It does not compare XML text or infer renames. See
[version-comparison.md](version-comparison.md) for the complete contract.

## Stable exit codes

| Code | Meaning |
|---:|---|
| 0 | Command succeeded; validation, when requested, is valid |
| 2 | Invalid command-line usage or unresolved profile selector |
| 3 | Input/read/detection/parse/serialization failure |
| 4 | Validation completed and reported errors |
| 5 | Requested layer, profile release, generated model, feature, or rule is unavailable |
| 10 | Internal adapter failure |

When `--json` is present, operational errors also use the versioned envelope
and a stable `error.code`. Diagnostics are deliberately generic and must not
contain account numbers, names, addresses, transaction references, or the
original financial message.

## Compliance boundary

A successful result means the message passed the rules implemented and selected
by this SDK. It does not guarantee acceptance by a bank or network, regulatory
certification, onboarding approval, or legal compliance.
