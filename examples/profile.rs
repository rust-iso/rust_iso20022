//! Demonstrate exact-release profile addressing without claiming a rule pack.
//!
//! Run with `cargo run --example profile --features profiles`.

use rust_iso20022::profiles::{ProfileRegistry, ProfileRelease, ProfileReleaseKey};

const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = ProfileReleaseKey::parse(&format!(
        "example-network|2026.1|ig-2026.1|2026-01-01|2026-12-31|{DIGEST}"
    ))?;
    let release = ProfileRelease::unavailable(
        key.clone(),
        &["pacs.008.001.08"],
        "example only: no authoritative profile rule pack is installed",
        "example-fixture-index",
        "example-evidence",
    )?;
    let registry = ProfileRegistry::new(vec![release])?;
    assert!(registry.supports(&key, "pacs.008.001.08").is_ok());
    assert!(registry.rule_set_for(&key, "pacs.008.001.08").is_err());
    Ok(())
}
