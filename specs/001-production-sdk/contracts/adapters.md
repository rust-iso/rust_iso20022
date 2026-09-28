# Contract: CLI, MCP, and WASM Adapters

All adapters call core SDK functions and serialize core result DTOs. Contract
tests feed identical fixtures through native core and each supported adapter and
compare normalized results.

## CLI

Commands: `detect`, `inspect`, `validate`, `explain`, `to-json`, `to-xml`,
`catalog`, and `versions`; later `convert` and `compare` use the same pattern.

`--json` emits one versioned JSON document to stdout. Diagnostics go to stderr
through the shared redaction policy. Human output is not a machine contract.

Stable exit codes:

| Code | Meaning |
|---:|---|
| 0 | Command completed successfully and requested validation passed |
| 2 | CLI usage/configuration error |
| 3 | Parse/detection/deserialization failure |
| 4 | Validation completed with an invalid report |
| 5 | Requested capability/layer/profile unavailable |
| 10 | Internal failure |

Exit-code values may be extended only by reserving new meanings; existing meanings
do not change in an additive release.

## MCP

Transport is local stdio. Initial tools:

- `detect_message`
- `inspect_message`
- `validate_message`
- `lookup_message`
- `lookup_field`
- `compare_versions`
- `explain_validation_error`

Inputs contain in-memory text/bytes encoded by the protocol and limits/options.
The server has no tool for file paths, URLs, network, shell, or message upload.
Each tool declares versioned input/output JSON Schema and returns structured
content. Stderr logs are redacted. Protocol annotations do not substitute for
removing dependencies/capabilities.

## WASM

- Preserve existing JS export names/behavior through tested shims.
- New APIs return structured serde/wasm-bindgen values; no manual JSON assembly.
- Verified subset: bounded parsing, serialization, detection, catalogue, and
  basic validation.
- Unsupported validation backends/features fail with a structured unavailable
  result and are documented/feature-gated; no silent degradation.
- WASM uses the same default `ParseLimits`, metadata, fixtures, and safe error
  contracts as native core.

