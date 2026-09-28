# Package candidate

Status: verified locally, not published.

## Commands and results

```text
cargo +stable package --list --allow-dirty
PASS: 1,266 files; 1,164 generated Rust files; no `.agents`, `.specify`,
`evidence`, `specs`, `tests`, `tools`, `xsds`, `crates`, `fuzz`, or `profiles`
repository-internal paths.

CARGO_TARGET_DIR=/tmp/rust_iso20022-package \
  cargo +stable package --locked --allow-dirty --offline --jobs 1
PASS: package verified by compiling the extracted archive.
```

Archive: `rust_iso20022-0.1.2.crate`

- Files: 1,268
- Size: 9,833,317 bytes (Cargo reported 9.4 MiB compressed).
- SHA-256: `ff919394f88e94c1b464a3f83ab3bcc93f4c4e56dfcf40c9fd4551738eb8c242`.
- Limit gate: less than 10,485,760 bytes.
- Uncompressed package content: approximately 101.8 MiB.

The first candidate was 12,623,543 bytes. The source was the schema metadata
blob, not optional user documentation. Codegen now Brotli-compresses that same
descriptor corpus; `src/metadata/generated.bin` is 963,171 bytes. A second full
`codegen --check` over all 1,130 schemas confirmed deterministic output.

The normal online `cargo package` attempt could not update the crates.io index
because the execution sandbox's configured proxy was unavailable. `--offline`
used the locked, cached dependencies and completed package verification. CI is
configured to repeat the locked online command.

## docs.rs memory selection

`package.metadata.docs.rs` selects `model-head`, `model-pacs`, `model-pain`,
`serde`, and `convert`. It intentionally does not select the umbrella `model`
feature. The equivalent strict rustdoc build and 16 doctests passed in WP-031;
the compile artifact directory reached roughly 13 GiB on disk with one job,
which confirms that an all-family docs build would be inappropriate for the
docs.rs memory sandbox.
