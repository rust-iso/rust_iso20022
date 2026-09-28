//! Validate a canonical generated `pacs.008.001.08` value.
//!
//! Run with `cargo run --example validation --features model-pacs`.

use rust_iso20022::from_xml;
use rust_iso20022::generated::pacs::pacs_008_001_08::Document;
use rust_iso20022::validation::bindings::validate_pacs_008_001_08;

const XML: &str = include_str!("../fixtures/iso/valid/pacs.008.001.08-cross-field.xml");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let generated: Document = from_xml(XML)?;
    let report = validate_pacs_008_001_08(&generated)?;
    assert!(report.valid(), "unexpected issues: {:?}", report.issues());
    Ok(())
}
