use rust_iso20022_wasm::api;

const VALID: &str = include_str!("../../../fixtures/iso/valid/pacs.008.001.08-cross-field.xml");
const INVALID: &str = include_str!("../../../fixtures/iso/invalid/pacs.008.001.08-cross-field.xml");

#[test]
fn detection_and_catalogue_are_normalized_core_views() {
    let actual = api::detect_message(VALID).unwrap();
    let core = rust_iso20022::detect_message(VALID, rust_iso20022::ParseLimits::DEFAULT).unwrap();
    assert_eq!(actual.message_id, core.message_id().as_str());
    assert_eq!(actual.namespace, core.namespace());

    let catalogue = api::catalogue_message(&actual.message_id).unwrap();
    assert_eq!(catalogue.message_id, core.descriptor().identity);
    assert_eq!(catalogue.schema_sha256, core.descriptor().schema_sha256);
}

#[test]
fn parse_and_serialize_round_trip_the_canonical_generated_type() {
    let parsed = api::parse_message(VALID).unwrap();
    assert_eq!(parsed.message_id, "pacs.008.001.08");
    let xml = api::serialize_message(&parsed.message_id, &parsed.document).unwrap();
    let reparsed = rust_iso20022::parse(&xml.xml).unwrap();
    assert_eq!(reparsed.message_id().as_str(), parsed.message_id);
    let reparsed_document: serde_json::Value =
        serde_json::from_str(&reparsed.to_json().unwrap()).unwrap();
    assert_eq!(reparsed_document, parsed.document);
}

#[test]
fn validation_is_object_equivalent_to_the_core_binding() {
    let actual = api::validate_message(INVALID, "l2").unwrap();
    let parsed = rust_iso20022::parse(INVALID).unwrap();
    let core = rust_iso20022::validation::bindings::validate_parsed_message(&parsed).unwrap();
    assert_eq!(actual.valid, core.valid());
    assert_eq!(actual.layer, "iso_semantic");
    assert_eq!(actual.complete, core.complete());
    assert_eq!(actual.errors.len(), core.errors().len());
    assert_eq!(
        actual
            .errors
            .iter()
            .map(|issue| issue.rule_id.as_str())
            .collect::<Vec<_>>(),
        core.errors()
            .into_iter()
            .map(|issue| issue.rule_id.as_str())
            .collect::<Vec<_>>()
    );
}
