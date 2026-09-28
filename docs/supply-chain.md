# Supply-chain policy

`Cargo.lock` is tracked and is the dependency resolution used by CI and release
candidate checks. Release automation uses `--locked`; a lockfile change must be
reviewed like a source change.

The project uses two complementary checks:

- `cargo-audit 0.22.2` checks the RustSec advisory database.
- `cargo-deny 0.20.2` enforces source and license policy and also checks
  advisories. Unknown registries and Git repositories are denied. The one
  code-generator Git dependency is pinned to an exact revision and explicitly
  allowed; it is not part of the published runtime crate.

`deny.toml` permits Apache-2.0, MIT, BSD, ISC, MPL-2.0, Unicode-3.0, Zlib, and
the LLVM exception. This is a dependency-admission policy, not legal advice.
Multiple versions produce a warning because the generated-model stack and MCP
SDK currently require different proc-macro generations; each warning remains
visible for maintenance rather than being hidden by broad skip rules.

Run locally with:

```bash
cargo audit --locked
cargo deny --locked --all-features check
```

Both commands can require network access to update advisory data and download
license texts. An offline partial check may be run with:

```bash
cargo deny --offline --locked --no-default-features \
  --exclude-unpublished check licenses bans sources
```

The full online result is the release gate. A network-restricted local result
must be recorded as blocked, never silently treated as a pass.

