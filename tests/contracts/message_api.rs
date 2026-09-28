use rust_iso20022::generated::any::AnyMessage;
use rust_iso20022::{ParseError, parse};

const PACS002: &str = include_str!("../data/pacs.002.001.10.xml");

#[test]
fn parsed_message_exposes_uniform_identity_and_generated_access() {
    let parsed = parse(PACS002).expect("parse pacs.002");
    assert_eq!(parsed.message_id().as_str(), "pacs.002.001.10");
    assert_eq!(parsed.business_area(), "pacs");
    assert_eq!(parsed.family(), "pacs.002.001");
    assert_eq!(parsed.version().as_str(), "10");
    assert_eq!(
        parsed.namespace(),
        "urn:iso:std:iso:20022:tech:xsd:pacs.002.001.10"
    );
    assert_eq!(parsed.root_element(), "Document");
    assert_eq!(parsed.descriptor().identity, "pacs.002.001.10");

    let message_ref = parsed.as_message_ref();
    assert!(matches!(
        message_ref.message(),
        AnyMessage::Pacs_002_001_10(_)
    ));
    assert!(message_ref.message().as_pacs_002_001_10().is_some());

    let xml = parsed.to_xml().expect("serialize through generated value");
    assert!(xml.contains("Document"));
    let json = parsed
        .to_json()
        .expect("serialize JSON through generated value");
    assert!(json.contains("FIToFIPmtStsRpt"));

    let generated = parsed
        .into_message()
        .into_pacs_002_001_10()
        .expect("typed generated recovery");
    let _: rust_iso20022::generated::pacs::pacs_002_001_10::Document = *generated;
}

#[test]
fn parse_errors_are_typed_for_unknown_root_mismatch_and_disabled_area() {
    let unknown = parse(r#"<Document xmlns="urn:example:unknown"/>"#).unwrap_err();
    assert!(matches!(unknown, ParseError::UnknownMessage));

    let mismatch =
        parse(r#"<Wrong xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.002.001.10"/>"#).unwrap_err();
    assert!(matches!(mismatch, ParseError::RootMismatch { .. }));

    let disabled =
        parse(r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pain.001.001.09"/>"#).unwrap_err();
    assert!(matches!(
        disabled,
        ParseError::UnsupportedFeature {
            required_feature: "model-pain",
            ..
        }
    ));
}

#[test]
fn identifier_round_trips_without_losing_numeric_width() {
    let parsed = parse(PACS002).expect("parse pacs.002");
    let id = parsed.message_id();
    assert_eq!(id.business_area(), "pacs");
    assert_eq!(id.functionality(), "002");
    assert_eq!(id.variant(), "001");
    assert_eq!(id.version().numeric(), 10);
    assert_eq!(id.to_string(), "pacs.002.001.10");
}

#[test]
fn non_document_schema_root_uses_its_generated_root_type() {
    let xml = r#"<AppHdr xmlns="urn:iso:std:iso:20022:tech:xsd:head.001.001.04"/>"#;
    let parsed = parse(xml).expect("parse AppHdr root");
    assert_eq!(parsed.root_element(), "AppHdr");
    let generated = parsed
        .message()
        .as_head_001_001_04()
        .expect("typed generated AppHdr root");
    let _: &rust_iso20022::generated::head::head_001_001_04::BusinessApplicationHeaderV04 =
        generated;
}
