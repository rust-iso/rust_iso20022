//! JSON conversion of message types (prowide `toJson`/`fromJson` parallel).
//! Run with `cargo test --features model,serde`.
#![cfg(all(feature = "model-camt", feature = "model-pacs", feature = "serde"))]

use rust_iso20022::generated::camt::camt_056_001_09::Document as Camt056;
use rust_iso20022::generated::pacs::pacs_002_001_10::Document as Pacs002;

#[test]
fn camt_056_json_roundtrips() {
    let xml = include_str!("data/camt.056.001.09.xml");
    let a: Camt056 = rust_iso20022::from_xml(xml).unwrap();
    let json = rust_iso20022::to_json(&a).unwrap();
    let b: Camt056 = rust_iso20022::from_json(&json).unwrap();
    assert_eq!(a, b);
}

#[test]
fn pacs_002_json_and_xml_agree() {
    let xml = include_str!("data/pacs.002.001.10.xml");
    let from_x: Pacs002 = rust_iso20022::from_xml(xml).unwrap();
    // JSON -> value -> JSON is stable, and the value matches the XML-parsed one.
    let json = rust_iso20022::to_json(&from_x).unwrap();
    let from_j: Pacs002 = rust_iso20022::from_json(&json).unwrap();
    assert_eq!(from_x, from_j);
}
