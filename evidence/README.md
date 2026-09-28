# Verification Evidence

This directory contains reviewable proof for Spec Kit work packages. Evidence is
part of the implementation: a WP is not Done merely because code exists or a
focused command returned success.

Each `WP-NNN.md` record must identify:

- the WP and acceptance criteria being demonstrated;
- repository state (commit when available, otherwise explicit dirty-tree scope);
- UTC timestamp, OS/architecture, Rust/Cargo and relevant tool versions;
- exact commands and feature/target selections;
- fixture, schema, profile, generator, or corpus identities and digests;
- exit status and a concise result that proves the asserted behavior;
- skipped/unavailable checks and known limitations;
- the final common-gate outcome.

Generated logs or payloads must follow the project redaction policy. Never store
real financial messages, credentials, raw private data, machine secrets, or
unreviewed copyrighted standards material here.

Use [template.md](./template.md) for every WP. Performance, fuzz, profile,
migration, and release records add the fields required by their contracts.

