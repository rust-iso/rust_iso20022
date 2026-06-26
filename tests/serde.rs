//! Serde round-trips for the core/catalogue types (run with `--features serde`).
#![cfg(feature = "serde")]

use rust_iso20022::{BusinessArea, MxId};

#[test]
fn mxid_serializes_to_canonical_name() {
    let id = MxId::parse("urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08").unwrap();
    let json = serde_json::to_string(&id).unwrap();
    assert_eq!(json, "\"pacs.008.001.08\"");

    let back: MxId = serde_json::from_str(&json).unwrap();
    assert_eq!(back, id);

    // also accepts a full namespace on the way in
    let from_ns: MxId =
        serde_json::from_str("\"urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08\"").unwrap();
    assert_eq!(from_ns, id);
}

#[test]
fn business_area_serializes_to_code() {
    let json = serde_json::to_string(&BusinessArea::pacs).unwrap();
    assert_eq!(json, "\"pacs\"");
    let back: BusinessArea = serde_json::from_str("\"camt\"").unwrap();
    assert_eq!(back, BusinessArea::camt);
}

#[test]
fn catalogue_entry_serializes() {
    let e = rust_iso20022::catalogue::from_message_name("pacs.008.001.08").unwrap();
    let json = serde_json::to_string(e).unwrap();
    assert!(json.contains("pacs.008.001.08"));
    assert!(json.contains("has_model"));
}
