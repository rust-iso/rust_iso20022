# Release candidate process

The repository publishes only after the local evidence gate. Version 0.1.6 is
the next release candidate after `v0.1.5`; the workflow remains manual-only,
has read-only repository permissions, and does not publish on every push.

## Local baseline

```bash
scripts/test-release-check.sh
scripts/release-check.sh --plan
scripts/release-check.sh
```

To reproduce the docs.rs feature set locally without committing generated HTML:

```bash
scripts/build-docs-local.sh
```

The script uses `model-head,model-pacs,model-pain,serde,convert`, one job by
default, and writes disposable output under `target/docs-local/`.

`.github/workflows/docs-pages.yml` runs the same script with one Cargo job and
deploys the disposable HTML artifact to
`https://rust-iso.github.io/rust_iso20022/`. GitHub Pages is the stable primary
documentation URL; docs.rs remains a useful registry mirror but is not the
only documentation delivery path.

The full check runs with one Cargo job and debug information disabled to bound
memory. Generated model families are compiled as separate matrix rows because
building all 1,130 modules in one rustc invocation is unnecessarily expensive.

The required evidence includes the supported message/profile/migration matrix,
MSRV and feature matrix, schema and generator manifests, known limitations,
package file list, checksums, and results of every mandatory gate. A failed or
unavailable mandatory gate leaves the candidate not ready.

`scripts/check-package-size.sh` enforces the crates.io 10 MiB upload ceiling.
The metadata descriptor corpus is Brotli-compressed by codegen so the complete
generated API remains in one crate without exceeding that limit.

## SBOM

`scripts/generate-sbom.sh` uses `cargo-cyclonedx 0.5.9` and emits CycloneDX
1.5 JSON. It sets `SOURCE_DATE_EPOCH` from the environment or the checked-out
commit so the generator omits random serial data and uses a reproducible
timestamp. The script resolves all features and target platforms against the
tracked lockfile and collects one SBOM per workspace package, including the
adapters, beside each package's SHA-256 checksum. It rejects an unexpected tool
version, an invalid SBOM, or a lockfile changed by the generator. Adapter
binaries have separate build artifacts and are not crates.io packages.

## Future authorized publication

Publication requires an explicit maintainer decision after all WPs are closed.
At that point, review the package archive, sign artifacts using the project's
chosen key policy, publish the crate, create the GitHub Release, and verify the
docs.rs result. None of those state-changing steps is automated here.
