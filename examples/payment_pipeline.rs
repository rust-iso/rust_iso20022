//! End-to-end payment flow: detect, catalogue, parse, validate, and serialize.
//!
//! ```bash
//! cargo run --example payment_pipeline --features model-pacs,serde
//! ```

use rust_iso20022::prelude::Pacs008Document;
use rust_iso20022::validation::bindings::validate_pacs_008_001_08;
use rust_iso20022::{detect, from_xml, lookup_message, to_json, to_xml};

const XML: &str = include_str!("../fixtures/iso/valid/pacs.008.001.08-cross-field.xml");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let detected = detect(XML).ok_or("message was not detected")?;
    let entry = lookup_message(&detected.message_name()).ok_or("message is not catalogued")?;
    println!("detected: {} ({})", entry.message_name, entry.business_area);
    println!(
        "required feature: {}",
        rust_iso20022::required_feature(entry.message_name).unwrap()
    );

    let document: Pacs008Document = from_xml(XML)?;
    let report = validate_pacs_008_001_08(&document)?;
    if !report.valid() {
        return Err(format!("validation failed: {:?}", report.issues()).into());
    }
    let xml = to_xml(&document)?;
    let json = to_json(&document)?;
    println!(
        "validated and serialized: xml={} bytes, json={} bytes",
        xml.len(),
        json.len()
    );
    Ok(())
}
