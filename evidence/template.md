# WP-NNN Evidence

## Identity

- Work package:
- Specification:
- Repository state:
- Recorded at (UTC):
- Platform:
- Rust/Cargo:
- Relevant tools:

## Inputs

| Input | Version/identity | SHA-256 or provenance |
|---|---|---|
| Fixture/schema/profile/corpus | | |

## Acceptance Evidence

| Criterion | Command or inspection | Result | Status |
|---|---|---|---|
| | | | PASS / FAIL / BLOCKED |

## Commands

```console
# Exact commands, in execution order
```

## Common Quality Gates

| Gate | Feature/target selection | Result |
|---|---|---|
| `cargo fmt --check` | workspace | |
| `cargo clippy` | applicable workspace targets | |
| focused tests | explicit features | |
| workspace tests | explicit features | |
| doctests | explicit features | |
| `git diff --check` | repository | |
| panic/TODO/placeholder review | active WP diff | |

## Limitations and Follow-up

- None, or a precise limitation with owner, evidence, affected acceptance
  criterion, and remediation path.

## Conclusion

Not Done until every mandatory criterion above is PASS and no required
implementation is a stub, mock, always-success path, happy-path-only behavior, or
silent omission.

