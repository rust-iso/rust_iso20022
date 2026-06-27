//! Round-trip parse/serialize tests over real ISO 20022 sample messages.
//!
//! These require the generated message model, so run with:
//! ```bash
//! cargo test --features model
//! ```

#![cfg(all(feature = "model-pacs", feature = "model-pain", feature = "model-camt"))]

use rust_iso20022::{from_xml, to_xml};

/// Parse a sample message, re-serialize it, and confirm the value round-trips
/// (parsing the re-serialized XML yields an equal value).
fn assert_roundtrips<T>(xml: &str)
where
    T: yaserde::YaDeserialize + yaserde::YaSerialize + PartialEq + std::fmt::Debug,
{
    let parsed: T = from_xml(xml).expect("parse sample");
    let serialized = to_xml(&parsed).expect("serialize");
    let reparsed: T = from_xml(&serialized).expect("re-parse serialized");
    assert_eq!(parsed, reparsed, "value changed across a serialize/parse cycle");
}

#[test]
fn pain_001_001_09_roundtrips() {
    // pain.001 carries amounts inside a `<choice>` (InstdAmt vs EqvtAmt). With
    // choices modelled as optional fields, the amount and its `Ccy` attribute
    // both round-trip.
    use rust_iso20022::generated::pain::pain_001_001_09::Document;
    let xml = include_str!("data/pain.001.001.09.xml");
    assert_roundtrips::<Document>(xml);
}

#[test]
fn pacs_002_001_10_roundtrips() {
    use rust_iso20022::generated::pacs::pacs_002_001_10::Document;
    let xml = include_str!("data/pacs.002.001.10.xml");
    assert_roundtrips::<Document>(xml);
}

#[test]
fn camt_056_001_09_roundtrips() {
    use rust_iso20022::generated::camt::camt_056_001_09::Document;
    let xml = include_str!("data/camt.056.001.09.xml");
    assert_roundtrips::<Document>(xml);
}
