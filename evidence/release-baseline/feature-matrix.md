# Feature and platform matrix

Default features are empty. Enable only the generated business areas and
capabilities an application needs.

| Capability | Cargo selection | Native | WASM | Notes |
|---|---|:---:|:---:|---|
| Detection, generic inspection, catalogue, version diff | default | yes | yes | No generated model required |
| Generated XML model | `model-<area>` | yes | build-dependent | Prefer one area; `model` compiles all 1,130 modules |
| JSON | `serde` or `json` | yes | yes | `serde` remains a compatibility feature |
| Typed scalar conversion | `convert` | yes | not verified | Decimal and chrono conversions |
| Structured validation primitives | default | yes | yes | Generated binding requires its model feature |
| pacs.008.001.08 L2 binding | `model-pacs` | yes | yes | Other versions return unavailable |
| Exact profile identity | `profiles` | yes | no adapter execution | Framework only; no complete L3 pack installed |
| Runtime schema fetch | `catalogue` | yes | no | Optional network feature; static catalogue stays offline |
| CLI | workspace package `rust_iso20022_cli` | yes | no | Thin adapter, stable JSON and exit codes |
| MCP | workspace package `rust_iso20022_mcp` | yes | no | Local stdio only; crate MSRV 1.88 |
| Migration | workspace package `rust_iso20022_migration` | yes | not verified | Features `mt103`, `mt202`, `mt940` |
| Structured WASM SDK | `rust_iso20022_wasm` + `model-pacs` | parity tests | yes | Detection-only build is available without model |

## Exact version coverage

- Generated schema catalogue: 1,130 checked-in ISO 20022 schema identities;
  each descriptor records its individual schema hash and provenance status.
- Generated validation binding: pacs.008.001.08.
- Builders: pacs.002.001.10, pacs.008.001.08, pacs.009.001.08,
  pain.001.001.09, pain.002.001.10, camt.052.001.08, camt.053.001.09, and
  camt.054.001.08.
- Migration: MT103 to pacs.008.001.08; MT202 to pacs.009.001.08; MT940 to
  camt.053.001.09.
- Profiles: exact-release framework only. CBPR+ SR2026 and SEPA 2025 sources are
  identified, but no complete executable production rule pack is claimed.

Core MSRV is Rust 1.85 with Rust 2024 edition. The MCP adapter independently
requires Rust 1.88 because of its protocol SDK. See [compatibility](compatibility.md),
[WASM](wasm.md), and [status](status.md) for limitations.
