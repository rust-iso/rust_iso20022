//! Convert MT103 into the canonical generated pacs.008 type and retain the
//! complete mapping-accountability report.

use rust_iso20022::generated::pacs::pacs_008_001_08::Document;
use rust_iso20022_migration::mt103;

const INPUT: &str = include_str!("../../../fixtures/migration/mt103/input.mt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converted = mt103::convert(INPUT)?;
    let _: &Document = &converted.message;
    println!(
        "{} mappings, {} warnings, {} unmapped fields",
        converted.mapping_report.entries().len(),
        converted.mapping_report.warnings().len(),
        converted.mapping_report.unmapped_fields().len()
    );
    Ok(())
}
