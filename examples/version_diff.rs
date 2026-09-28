//! Compare two versions using normalized schema metadata, not XML text.

use rust_iso20022::compare::compare_versions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let diff = compare_versions("pacs.008.001.08", "pacs.008.001.10")?;
    println!(
        "{} -> {}: {} semantic changes",
        diff.from.identity,
        diff.to.identity,
        diff.changes.len()
    );
    Ok(())
}
