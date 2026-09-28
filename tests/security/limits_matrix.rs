use rust_iso20022::{ParseLimits, XmlLimit, XmlReadError, validate_xml};

fn assert_boundary(
    xml: &str,
    actual: usize,
    limit: XmlLimit,
    set: impl Fn(ParseLimits, usize) -> ParseLimits,
) {
    let base = ParseLimits::DEFAULT;
    if actual > 0 {
        let error = validate_xml(xml, set(base, actual - 1)).expect_err("limit minus one");
        assert!(matches!(
            error,
            XmlReadError::LimitExceeded {
                limit: found,
                maximum
            } if found == limit && maximum == actual - 1
        ));
    }
    validate_xml(xml, set(base, actual)).expect("exact limit must be accepted");
    validate_xml(xml, set(base, actual + 1)).expect("limit plus one must be accepted");
}

#[test]
fn every_resource_dimension_has_exact_inclusive_boundaries() {
    let basic = "<a/>";
    assert_boundary(basic, basic.len(), XmlLimit::InputBytes, |limits, value| {
        limits.with_max_input_bytes(value)
    });

    let nested = "<a><b><c/></b></a>";
    let stats = validate_xml(nested, ParseLimits::DEFAULT).unwrap();
    assert_boundary(nested, stats.max_depth, XmlLimit::Depth, |limits, value| {
        limits.with_max_depth(value)
    });
    assert_boundary(nested, stats.events, XmlLimit::Events, |limits, value| {
        limits.with_max_events(value)
    });
    assert_boundary(
        nested,
        stats.elements,
        XmlLimit::Elements,
        |limits, value| limits.with_max_elements(value),
    );

    let attributed = "<a x='1' y='2' z='3'/>";
    let stats = validate_xml(attributed, ParseLimits::DEFAULT).unwrap();
    assert_boundary(
        attributed,
        stats.attributes,
        XmlLimit::Attributes,
        |limits, value| limits.with_max_attributes(value),
    );

    let text = "<a>1234567</a>";
    assert_boundary(text, 7, XmlLimit::TextBytes, |limits, value| {
        limits.with_max_text_bytes(value)
    });
    assert_boundary(text, 7, XmlLimit::DecodedBytes, |limits, value| {
        limits.with_max_decoded_bytes(value)
    });

    let collection = "<a><b/><c/><d/></a>";
    let stats = validate_xml(collection, ParseLimits::DEFAULT).unwrap();
    assert_eq!(stats.max_collection_items, 3);
    assert_boundary(
        collection,
        stats.max_collection_items,
        XmlLimit::CollectionItems,
        |limits, value| limits.with_max_collection_items(value),
    );
}

#[test]
fn minimized_attack_corpus_has_stable_typed_failures_without_panics() {
    let forbidden = [
        include_str!("../../fixtures/security/xml/dtd.xml"),
        include_str!("../../fixtures/security/xml/entity-expansion.xml"),
        include_str!("../../fixtures/security/xml/xinclude.xml"),
        include_str!("../../fixtures/security/xml/non-utf8-declaration.xml"),
    ];
    for xml in forbidden {
        let result = std::panic::catch_unwind(|| validate_xml(xml, ParseLimits::DEFAULT));
        assert!(matches!(result, Ok(Err(XmlReadError::Forbidden { .. }))));
    }

    for xml in [
        include_str!("../../fixtures/security/xml/unbound-namespace.xml"),
        include_str!("../../fixtures/security/xml/malformed.xml"),
        include_str!("../../fixtures/security/xml/invalid-unicode.xml"),
    ] {
        let result = std::panic::catch_unwind(|| validate_xml(xml, ParseLimits::DEFAULT));
        assert!(matches!(result, Ok(Err(XmlReadError::Malformed { .. }))));
    }

    let deep = include_str!("../../fixtures/security/xml/deep-template.xml");
    let error = validate_xml(deep, ParseLimits::DEFAULT.with_max_depth(3)).unwrap_err();
    assert!(matches!(
        error,
        XmlReadError::LimitExceeded {
            limit: XmlLimit::Depth,
            ..
        }
    ));

    let oversized = include_str!("../../fixtures/security/xml/oversized-template.xml");
    let error = validate_xml(
        oversized,
        ParseLimits::DEFAULT.with_max_input_bytes(oversized.len() - 1),
    )
    .unwrap_err();
    assert!(matches!(
        error,
        XmlReadError::LimitExceeded {
            limit: XmlLimit::InputBytes,
            ..
        }
    ));

    let collection = include_str!("../../fixtures/security/xml/collection-template.xml");
    let error = validate_xml(
        collection,
        ParseLimits::DEFAULT.with_max_collection_items(2),
    )
    .unwrap_err();
    assert!(matches!(
        error,
        XmlReadError::LimitExceeded {
            limit: XmlLimit::CollectionItems,
            ..
        }
    ));
}
