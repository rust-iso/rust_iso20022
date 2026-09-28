# Contract: Migration, Version Comparison, and Security

## MT Migration

Supported phase-one conversions:

- MT103 → a declared pacs.008 generated version
- MT202 → a declared pacs.009 generated version
- MT940 → a declared camt.053 generated version

Every result contains the generated target message and a `MappingReport`. Every
parsed source field has one or more report entries classified as `Exact`,
`Derived`, `Lossy`, `Ambiguous`, or `Unsupported`. Unsupported and unmapped
fields remain visible. Warnings explain safe consequences without copying
sensitive values. No conversion is described as lossless without fixture and
field-by-field evidence.

## Version Comparison

Input is two message identifiers/descriptors in the same comparable family.
Output includes both schema hashes and ordered changes:

- added/removed field
- field type changed
- cardinality changed
- enum value added/removed

Each change contains logical path and normalized before/after values. Comparing
unrelated families is a typed error. Rename is not inferred. Source is schema
metadata, never XML instance text.

## Bounded XML Security

All XML-capable public entry points invoke the same gate before detection,
deserialization, schema validation, or adapter-specific work. The gate enforces
`ParseLimits`, rejects DTD/DOCTYPE, external/general entities beyond explicitly
supported predefined/numeric references, external schema fetching, and XInclude,
and validates balanced structure, namespace/root, and the documented encoding
policy.

Typed failures identify the exceeded category without echoing payload. Fuzz and
attack-regression acceptance includes no panic/crash/UB and stable timeout/OOM
handling under the defined harness budget.

## Privacy

The default redaction policy applies to `Display`, `Debug`, logs, errors, CLI
stderr, MCP stderr/results, WASM errors, mapping warnings, and validation issues.
Tests insert unique canaries for IBAN/account/name/address/reference/remittance and
assert they never appear. Unsafe full-message logging exists only when a dedicated
feature is compiled and runtime opt-in is explicit; it produces a prominent
warning and is excluded from default/release adapter configurations.

