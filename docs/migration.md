# SWIFT MT migration framework

The migration crate provides a bounded transient SWIFT MT parser and a
value-free mapping-accountability report. It does not introduce another ISO
20022 model. Specific converters must finish by constructing an existing
generated `Document` type from the core crate.

## Parser contract

`MtDocument::parse` recognizes ordered SWIFT blocks `1`, `2`, `3`, `4`, `5`,
and `S`, including nested braces in non-text blocks. Block 4 is parsed into
ordered fields with:

- numeric tags with an optional uppercase option letter, such as `20`, `32A`,
  and `50K`;
- continuation lines preserved with `\n`;
- duplicate tags preserved with a one-based occurrence number;
- UTF-8 field values preserved without silent normalization; and
- byte source spans plus stable zero-based source indices.

The default finite limits are:

| Dimension | Limit |
|---|---:|
| Input bytes | 1 MiB |
| Block nesting depth | 8 |
| Top-level blocks | 16 |
| Block-4 fields | 512 |
| One field value | 64 KiB |
| Continuation lines per field | 256 |

Empty, malformed, misordered, duplicate-block, missing-text-block, invalid-tag,
and limit failures are typed. Display and Debug diagnostics contain only the
error category and byte offset, never the MT field value.

This parser is intentionally a bounded migration syntax layer, not a complete
claim of FIN network conformance. Literal unmatched braces in block content are
rejected because braces delimit the block grammar.

## Mapping accountability

Every converter builds a `MappingReport` containing exactly one `MappingEntry`
for every parsed block-4 field. Construction fails when a source is missing,
duplicated, unknown, or does not match its tag, occurrence, and source span.
Entries are returned in source order regardless of converter insertion order.

Each entry has exactly one classification:

| Classification | Meaning |
|---|---|
| `Exact` | Source meaning and representation map directly |
| `Derived` | Target value is deterministically computed from source data |
| `Lossy` | Some source precision or structure cannot be retained |
| `Ambiguous` | More than one reasonable interpretation or target exists |
| `Unsupported` | The converter does not map this source field |

`Ambiguous` and `Unsupported` sources are automatically included in
`unmapped_fields`. `Lossy` remains mapped but must carry a value-free rationale
and should normally have a warning. Reports store tag/index/span metadata, not
source values, so serialization cannot expose account numbers, names, or free
text from the original MT.

## Evidence rules for converters

Each MT103, MT202, or MT940 converter must add fixtures containing the input,
expected generated ISO 20022 XML, mapping report, warnings, and unmapped fields.
Tests must compare generated fields individually and demonstrate complete
source accounting. A converter must never silently skip a field.

Do not describe conversion as lossless unless fixtures and field-by-field
evidence prove every supported source representation round-trips without
semantic loss. `Exact` applies to an individual mapping entry; it is not a
blanket claim about the full message.

Implemented conversions:

- [MT103 to generated pacs.008.001.08](migration/mt103.md)
- [MT202 to generated pacs.009.001.08](migration/mt202.md)
- [MT940 to generated camt.053.001.09](migration/mt940.md)

## Compliance boundary

Passing parser, mapping, or validation checks means only that the implemented
rules succeeded. It does not guarantee bank or network acceptance, regulatory
certification, onboarding approval, or legal compliance.
