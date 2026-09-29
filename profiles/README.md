# Profile artifacts

Profile validation is keyed by an exact, immutable release identity. A bare
scheme or year such as `CBPR+ 2026` or `SEPA 2026` is not dispatchable.

The stable key form is:

```text
scheme|scheme_release|guideline_edition|effective_from|effective_until|sha256[,sha256]
```

Effective ranges are inclusive. Source digests remain in manifest order because
each digest identifies a specific reviewed document. New releases are appended;
an existing release key, support matrix, fixture index, or evidence reference is
never mutated to represent new semantics.

`manifest.schema.json` contains two schemas:

- `source_manifest` records exact release/source identity, publisher, URL or
  explicit null, section/reference, content digest, access classification, and
  recorded implementation/distribution decisions. It must not embed protected
  source text.
- `fixture_index` binds a complete profile release key to valid/invalid fixtures,
  their SHA-256 digests, expected rule IDs and paths, and an evidence reference.

Production profile directories must include both artifacts before a release can
be registered as available. A registry entry may explicitly remain unavailable
with a reason; framework availability alone is never presented as a rule pack.

Synthetic files under `tests/profiles/fixtures/` test the format and history
invariants only. They are marked `test_only` and are not CBPR+, SEPA, or other
market-profile content.

For usable developer checks before authoritative source artifacts are
available, use the opt-in public-reference bundles documented in
[`docs/profiles/public-reference.md`](../docs/profiles/public-reference.md).
They are explicitly labelled `provisional` and never enter the production
release registry.
