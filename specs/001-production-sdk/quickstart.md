# Quickstart and Verification Path

This document describes the intended end state and the commands used to verify it.
Commands whose WP is not yet implemented are acceptance targets, not claims about
the current repository.

## Use the Generated Model Directly

```rust
use rust_iso20022::generated::pacs::pacs_008_001_10::Document;

fn keep_full_schema_access(message: Document) -> Document {
    message
}
```

Generated models remain fully accessible. High-level APIs are additive.

## Detect and Parse Uniformly

```rust
let message = rust_iso20022::parse(xml.as_bytes())?;
assert_eq!(message.message_id().to_string(), "pacs.008.001.10");
println!("{} {}", message.business_area(), message.version());

// Typed access retrieves/consumes the underlying generated Document.
let generated = message.into_pacs_008_001_10()?;
```

## Validate with an Explicit Profile Release

```rust
let profile = profiles::resolve(ProfileScheme::CbprPlus, as_of)?;
let report = validate(&generated, ValidationRequest::all_layers(profile))?;

for issue in report.errors.iter().chain(&report.warnings) {
    eprintln!("{} {} {}", issue.rule_id, issue.path, issue.message);
}
```

The result means only “valid according to the implemented rules and identified
release.” It does not guarantee bank acceptance, certification, onboarding, or
legal compliance.

## Build a Generated Message

```rust
let document: generated::pacs::pacs_008_001_10::Document =
    Pacs008Builder::new()
        .message_id("MSG-001")?
        .settlement_amount(Money::try_new("EUR", "125.00")?)?
        .debtor_iban(Iban::try_from("DE89370400440532013000")?)?
        .build()?;
```

## CLI Machine Contract

```console
$ iso20022 detect --json payment.xml
{"schema_version":1,"message_id":"pacs.008.001.10",...}

$ iso20022 validate --profile cbpr-plus:<exact-release> --json payment.xml
```

Validation failure returns exit code 4; parse failure returns 3. Scripts consume
JSON and exit codes, not human text.

## Development Baseline

```console
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets
cargo fmt --check
git diff --check
```

The release WP additionally runs the declared feature matrix, MSRV job, clean
three-run code generation, CLI/MCP/WASM smoke contracts, profile and migration
fixtures, fuzz baseline, benchmarks, docs.rs-equivalent build, dependency audit,
license check, checksums, and SBOM evaluation. Results are recorded in
`evidence/` with fixture and source digests.

## Regeneration Contract

```console
cargo run -p rust_iso20022_codegen -- generate --check
cargo run -p rust_iso20022_codegen -- verify-manifests
```

Identical schemas, pinned generator source/config/toolchain, and lockfile must
produce byte-identical generated Rust and metadata. Any partial conversion,
hash mismatch, missing/stale file, or manual generated edit fails nonzero.
