# Foundation Evidence

## Identity

- Scope: T001–T008 shared implementation/evidence controls
- Feature directory: `specs/001-production-sdk`
- Repository state: dirty working tree containing user-directed pre-existing and
  current Spec Kit changes; no commit was created
- Recorded at: 2026-09-27 (Asia/Taipei)
- Platform: Darwin 25.6.0 arm64
- Rust: `rustc 1.96.0 (ac68faa20 2026-05-25)`
- Cargo: `cargo 1.96.0 (30a34c682 2026-05-25)`

## Acceptance Evidence

| Criterion | Command/inspection | Result | Status |
|---|---|---|---|
| Evidence and fixture structure exists | inspect `evidence/`, `fixtures/`, `tests/{compatibility,contracts,security,matrix}/` | Purpose README/template/sidecar contract/matrix present | PASS |
| Matrix enumerates required dimensions | inspect `tests/matrix/matrix.toml` | Message, version, serialization, layer, profile, expectation plus six mandatory baseline rows | PASS |
| Common-gate runner is executable shell | `bash -n scripts/check-work-package.sh` | exit 0 | PASS |
| CI YAML parses | Ruby standard-library YAML parser over `.github/workflows/ci.yml` | exit 0 | PASS |
| All generated area features appear in CI contract | shell loop over 32 `model-*` features | no missing feature | PASS |
| Stable workspace check | `cargo check --workspace --no-default-features` | exit 0 | PASS |
| MSRV workspace check | `rustup run 1.85 cargo check --workspace --no-default-features` | Cargo 1.85.1, exit 0 | PASS |
| Diff whitespace | `git diff --check` | exit 0 | PASS |

## Known Baseline Gaps

1. `cargo fmt --check` exits 1 because pre-existing codegen output, generated
   modules, catalogue data, and examples are not rustfmt-clean under Rust 1.96.
   Generated output must not be manually reformatted. WP-002 must make formatting
   part of deterministic generation and install a fully regenerated tree.
2. The `cargo +1.85` shorthand reports a local rustup manifest-alias error, but
   the installed toolchain itself is healthy: `rustup run 1.85 cargo check
   --workspace --no-default-features` passes with Cargo 1.85.1. CI independently
   installs the declared toolchain.
3. The full common-gate runner is intentionally not claimed passing at foundation
   time because the formatting baseline is known red and representative
   feature suites are part of WP-001 verification.

These are recorded baseline facts, not waived release criteria.

## Commands

```console
bash -n scripts/check-work-package.sh
ruby -e "require 'yaml'; YAML.load_file('.github/workflows/ci.yml')"
cargo fmt --check
cargo check --workspace --no-default-features
rustup run 1.85 cargo check --workspace --no-default-features
git diff --check
```

## Conclusion

T001–T008 are complete: the shared structure, contracts, runner, CI matrix, and
known starting gaps are explicit. This record does not mark WP-001 or any release
gate complete.
