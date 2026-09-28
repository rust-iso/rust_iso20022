use rust_iso20022::{MxNode, ParseLimits, XmlLimit, XmlReadError, validate_xml};

#[derive(Debug)]
struct DummyDocument;

impl yaserde::YaDeserialize for DummyDocument {
    fn deserialize<R: std::io::Read>(_: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        Ok(Self)
    }
}

fn limit_error(xml: &str, limits: ParseLimits, expected: XmlLimit) {
    let error = validate_xml(xml, limits).expect_err("input must exceed a finite limit");
    assert!(matches!(error, XmlReadError::LimitExceeded { limit, .. } if limit == expected));
    assert!(!format!("{error}").contains(xml));
    assert!(!format!("{error:?}").contains("SECRET-ACCOUNT-123"));
}

#[test]
fn every_resource_dimension_has_a_finite_enforced_limit() {
    limit_error(
        "<Document>SECRET-ACCOUNT-123</Document>",
        ParseLimits::default().with_max_input_bytes(8),
        XmlLimit::InputBytes,
    );
    limit_error(
        "<a><b><c/></b></a>",
        ParseLimits::default().with_max_depth(2),
        XmlLimit::Depth,
    );
    limit_error(
        "<a><b/><c/></a>",
        ParseLimits::default().with_max_events(3),
        XmlLimit::Events,
    );
    limit_error(
        "<a><b/><c/></a>",
        ParseLimits::default().with_max_elements(2),
        XmlLimit::Elements,
    );
    limit_error(
        "<a x='1' y='2'/>",
        ParseLimits::default().with_max_attributes(1),
        XmlLimit::Attributes,
    );
    limit_error(
        "<a>abcd</a>",
        ParseLimits::default().with_max_text_bytes(3),
        XmlLimit::TextBytes,
    );
    limit_error(
        "<a>&amp;&amp;</a>",
        ParseLimits::default().with_max_decoded_bytes(1),
        XmlLimit::DecodedBytes,
    );
    limit_error(
        "<a><b/><b/></a>",
        ParseLimits::default().with_max_collection_items(1),
        XmlLimit::CollectionItems,
    );
}

#[test]
fn active_xml_features_and_non_utf8_declarations_are_forbidden() {
    for xml in [
        "<!DOCTYPE a [<!ENTITY x 'boom'>]><a>&x;</a>",
        "<a xmlns:xi='http://www.w3.org/2001/XInclude'><xi:include href='file:///etc/passwd'/></a>",
        "<?xml version='1.0' encoding='ISO-8859-1'?><a/>",
    ] {
        let error = validate_xml(xml, ParseLimits::default()).expect_err("must be forbidden");
        assert!(matches!(error, XmlReadError::Forbidden { .. }));
    }
}

#[test]
fn schema_location_is_inert_and_never_resolved() {
    let xml = "<a xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance' xsi:schemaLocation='urn:test https://example.invalid/a.xsd'/>";
    validate_xml(xml, ParseLimits::default()).expect("schema hint is inert metadata");
}

#[test]
fn malformed_structure_namespace_and_unicode_are_typed_errors() {
    for xml in ["<a><b></a>", "<p:a/>", "<a>\u{0}</a>"] {
        let error = validate_xml(xml, ParseLimits::default()).expect_err("must be malformed");
        assert!(matches!(error, XmlReadError::Malformed { .. }));
        assert!(!error.to_string().contains(xml));
    }
}

#[test]
fn public_xml_entry_points_apply_safe_defaults_before_processing() {
    let oversized = format!(
        "<Document>{}</Document>",
        "x".repeat(ParseLimits::DEFAULT.max_input_bytes)
    );

    assert!(rust_iso20022::detect(&oversized).is_none());
    assert!(MxNode::parse(&oversized).is_none());
    assert_eq!(
        rust_iso20022::metadata::extract(&oversized),
        Default::default()
    );
    assert!(rust_iso20022::app_hdr::parse_business_header(&oversized).is_none());
    assert_eq!(
        rust_iso20022::read_business_message(&oversized),
        Default::default()
    );
    assert!(rust_iso20022::from_xml::<DummyDocument>(&oversized).is_err());
}

#[test]
fn normal_iso_message_is_accepted_and_counted() {
    let xml = r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08"><A Ccy="EUR">1</A></Document>"#;
    let stats = validate_xml(xml, ParseLimits::default()).expect("bounded XML");
    assert_eq!(stats.elements, 2);
    assert_eq!(stats.attributes, 1);
    assert!(stats.events >= 6);
    assert_eq!(stats.max_depth, 2);
    assert_eq!(stats.max_collection_items, 1);
}
