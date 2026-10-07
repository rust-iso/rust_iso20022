# Contributing

Thank you for improving rust_iso20022. The generated structs are the only
canonical ISO 20022 model. Additive APIs—builders, validation, profiles,
migration, CLI, MCP, and WASM—must consume or produce those structs rather than
introducing a second long-lived message model.

## Before opening a change

- Use synthetic or fully redacted financial data. Never commit real account
  numbers, names, addresses, transaction references, credentials, or messages.
- Identify the message version and, for profile work, the exact scheme release,
  effective date, source document, redistribution rights, and source digest.
- Do not manually edit `src/generated`, generated catalogue/metadata files, or
  generator manifests. Change the schema input or generator and regenerate.
- Preserve existing generated paths, serialization, metadata, detection,
  features, and WASM behavior unless a breaking change is explicitly justified
  with SemVer and migration notes.

## Spec Kit workflow

Material work follows the active specification under
`specs/001-production-sdk/`:

```text
Specification -> Plan -> Work Package -> Implementation -> Verification
              -> Evidence -> Release Baseline
```

Each work package must state its goal, scope, non-goals, dependencies,
implementation, tests, evidence, and acceptance criteria. A task is not done
when it contains a stub, placeholder, happy path only, or unverified claim.
Record commands and results in `evidence/WP-XXX.md` and update `tasks.md` only
after acceptance is demonstrated.

## Development checks

Use the smallest relevant feature set during development. Generated family
builds are intentionally separated to keep memory bounded.

```bash
cargo +stable fmt --all --check
cargo +stable clippy --workspace --all-targets --jobs 1 -- -D warnings
cargo +stable test --workspace --no-default-features --jobs 1
cargo +1.85 check --workspace --exclude rust_iso20022_mcp \
  --no-default-features --jobs 1
git diff --check
```

Run the focused tests for the work package as well. Release candidates use
`scripts/release-check.sh`; this command is intentionally much slower and also
requires external tools and network access.

GitHub Actions tests the large CAMT and CAAA families in source-bounded shards.
Run `python3 scripts/test-model-shards.py --matrix` to see the current plan and
`python3 scripts/test-model-shards.py --area camt --shard 0` to reproduce one
job. Each shard runs the existing generated smoke tests both without and with
Serde. CAMT shard 0 also tests the real builders, prelude alias, XML dispatch,
and JSON dispatch. All messages are covered, with a maximum of 2 MiB of model
source per compilation; the CAMT builder dependencies are retained in every
CAMT shard. The plan grows automatically as models are added, and inventory or
dispatch mismatches fail the job instead of silently dropping coverage.

Sharding operates in a disposable copy and masks only declarations and dispatch
guards for messages assigned to other shards. Model implementations, public
features, core, builders, and Cargo.lock remain unchanged. The ordinary family
jobs still compile complete families, and a separate job tests the production
focused CAMT feature. Full-family builds remain available through the ordinary
Cargo features; release checks continue to use full-family builds on capable
machines. CI uploads memory/process/disk measurements for each large-model job.

## Standards and profile changes

Public landing pages and community implementations are useful research, but do
not by themselves establish a complete rule pack. Field-level rules require a
traceable rule ID, exact release, affected message/version and path, reason,
source/reference, fixtures, and executable tests. Record whether source files
may be redistributed. Never infer or invent a digest.

## Pull requests

Explain compatibility impact, feature flags, MSRV impact, tests, evidence, and
known limitations. Separate hand-written API changes from schema-generated
changes. Link the relevant WP and issue. Release and publication remain a
maintainer action and require every mandatory gate to pass.

Security vulnerabilities must follow [SECURITY.md](SECURITY.md), not a public
issue. Sponsorship policy is documented in [SPONSORS.md](SPONSORS.md).
