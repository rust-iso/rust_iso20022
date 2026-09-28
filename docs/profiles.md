# Versioned market profiles

Enable the `profiles` feature to use exact-release identity and profile support
dispatch. Concrete CBPR+ and SEPA rule packs are separate later capabilities;
the framework does not claim that those rules are present.

Current source reviews and exact limitations are documented separately for
[CBPR+](profiles/cbpr-plus.md) and [SEPA SCT/SCT Inst](profiles/sepa.md).

A profile validation request must identify all of:

- scheme;
- scheme release;
- implementation-guideline edition;
- inclusive effective-from and effective-until dates;
- ordered SHA-256 digests of the reviewed source documents.

```rust
use rust_iso20022::profiles::{ProfileDate, ProfileReleaseKey};

let key = ProfileReleaseKey::parse(concat!(
    "test-network|2025.1|ig-2025.1|2025-01-01|2025-12-31|",
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
))?;
assert!(key.effective_on(ProfileDate::parse("2025-06-01")?));
# Ok::<(), Box<dyn std::error::Error>>(())
```

`ProfileRegistry::resolve_as_of` is only a convenience resolver. It returns a
stored concrete release; validation and support checks still use the complete
key. Historical releases remain separately addressable after later releases are
added. Unsupported message versions and unavailable rule packs are typed errors,
not silent successes.

Profile results mean “valid according to the implemented rules and identified
release.” They do not guarantee acceptance by a bank or network, regulatory
certification, onboarding approval, or legal compliance. A profile is not
labelled production-complete until its authoritative source/license decision,
support matrix, valid/invalid fixtures, and evidence all pass.
