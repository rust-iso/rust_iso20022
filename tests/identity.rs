//! Per-message identity + auto-detection (prowide `AbstractMX` parallel).
//! Run with `cargo test --features model`.
#![cfg(feature = "model")]

use rust_iso20022::generated::camt::camt_056_001_09::Document as Camt056;
use rust_iso20022::generated::pain::pain_001_001_09::Document as Pain001;
use rust_iso20022::{detect, parse_as, BusinessArea, MxMessage};

#[test]
fn message_carries_its_identity() {
    assert_eq!(Pain001::MESSAGE_NAME, "pain.001.001.09");
    assert_eq!(Pain001::BUSINESS_AREA, BusinessArea::pain);
    assert_eq!(Pain001::FUNCTIONALITY, "001");
    assert_eq!(Pain001::VERSION, "09");
    assert_eq!(Pain001::mx_id().message_name(), "pain.001.001.09");
    assert_eq!(
        Pain001::NAMESPACE,
        "urn:iso:std:iso:20022:tech:xsd:pain.001.001.09"
    );
}

#[test]
fn detect_finds_the_message_type() {
    let xml = include_str!("data/pain.001.001.09.xml");
    let id = detect(xml).expect("detect");
    assert_eq!(id.message_name(), "pain.001.001.09");
    assert_eq!(id.business_area, BusinessArea::pain);
}

#[test]
fn parse_as_guards_the_type() {
    let xml = include_str!("data/pain.001.001.09.xml");
    // correct type parses
    assert!(parse_as::<Pain001>(xml).is_ok());
    // wrong type is rejected before parsing
    assert!(parse_as::<Camt056>(xml).is_err());
}

#[test]
fn parse_auto_dispatches_to_the_right_variant() {
    use rust_iso20022::generated::any::{parse_auto, AnyMessage};

    let xml = include_str!("data/camt.056.001.09.xml");
    let msg = parse_auto(xml).expect("parse_auto");
    assert_eq!(msg.message_name(), "camt.056.001.09");
    assert_eq!(msg.mx_id().business_area, BusinessArea::camt);
    assert!(matches!(msg, AnyMessage::Camt_056_001_09(_)));
}
