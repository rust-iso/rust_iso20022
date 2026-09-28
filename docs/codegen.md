# Reproducible code generation

The checked-in generated structs remain the sole canonical ISO 20022 data
model. Do not edit `src/generated/` or `src/catalogue/data.rs` manually.

The generation identity consists of:

- raw inputs and hashes in `xsds/schema-manifest.json`;
- the explicitly unverified legacy source status in `xsds/schema-set.json`;
- Rust 1.85.1 in `rust-toolchain.toml` and the tracked workspace `Cargo.lock`;
- the pinned `xsd-parser` commit and deterministic settings in
  `tools/codegen/generator-config.toml`;
- generator source/input/output hashes in
  `tools/codegen/generator-manifest.json`.

Generate the complete tree transactionally:

```console
cargo run --locked -p rust_iso20022_codegen --
```

The generator validates all schema hashes first, writes and formats a temporary
tree, rejects any partial failure, and only then replaces the generated tree,
catalogue, and generator manifest together.

Verify the tracked tree without changing it:

```console
cargo run --locked -p rust_iso20022_codegen -- --check
```

Regenerate the schema manifests only after reviewing an intentional schema-set
change:

```console
cargo run --locked -p rust_iso20022_codegen -- --write-manifests
```

`--only <basename.xsd>` is an isolated diagnostic. It validates and formats a
temporary subset but never rewrites the tracked generated tree or catalogue.
`--allow-unmanifested-input` exists for generator fixtures; it must not be used
for a release generation.
