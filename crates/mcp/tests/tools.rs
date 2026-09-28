use rmcp::handler::server::wrapper::Parameters;
use rust_iso20022_mcp::{
    Iso20022Mcp,
    dto::{CompareInput, ExplainInput, FieldInput, MessageIdInput, MessageInput, ValidateInput},
};
use serde_json::{Value, json};

const VALID: &str = include_str!("../../../fixtures/iso/valid/pacs.008.001.08-cross-field.xml");
const INVALID: &str = include_str!("../../../fixtures/iso/invalid/pacs.008.001.08-cross-field.xml");
const MESSAGE_ID: &str = "pacs.008.001.08";
const RULE_ID: &str = "ISO20022-L2-IBAN-CHECKSUM";

fn normalized(value: impl serde::Serialize) -> Value {
    serde_json::to_value(value).unwrap()
}

fn error_of<T>(result: Result<T, String>) -> String {
    match result {
        Ok(_) => panic!("request unexpectedly succeeded"),
        Err(error) => error,
    }
}

#[test]
fn exposes_exactly_seven_versioned_structured_tool_contracts() {
    let service = Iso20022Mcp::new();
    let mut tools = service.listed_tools();
    tools.sort_by(|left, right| left.name.cmp(&right.name));
    let names: Vec<_> = tools.iter().map(|tool| tool.name.as_ref()).collect();
    assert_eq!(
        names,
        [
            "compare_versions",
            "detect_message",
            "explain_validation_error",
            "inspect_message",
            "lookup_field",
            "lookup_message",
            "validate_message",
        ]
    );
    assert!(tools.iter().all(|tool| tool.output_schema.is_some()));
    assert!(tools.iter().all(|tool| !tool.input_schema.is_empty()));

    let tracked: Value = serde_json::from_str(include_str!("../schemas/tools-v1.json")).unwrap();
    assert_eq!(tracked["schema_version"], 1);
    assert_eq!(tracked["mcp_protocol"], "2026-07-28");
    assert_eq!(tracked["tools"], normalized(service.listed_tools()));
}

#[test]
fn detection_inspection_and_lookup_are_exact_core_views() {
    let service = Iso20022Mcp::new();
    let detected = service
        .detect_message(Parameters(MessageInput {
            message: VALID.to_owned(),
        }))
        .unwrap()
        .0;
    let core = rust_iso20022::detect_message(VALID, rust_iso20022::ParseLimits::DEFAULT).unwrap();
    assert_eq!(detected.message_id, core.message_id().as_str());
    assert_eq!(detected.namespace, core.namespace());
    assert_eq!(detected.root_element, core.root_element());

    let inspected = service
        .inspect_message(Parameters(MessageInput {
            message: VALID.to_owned(),
        }))
        .unwrap()
        .0;
    let looked_up = service
        .lookup_message(Parameters(MessageIdInput {
            message_id: MESSAGE_ID.to_owned(),
        }))
        .unwrap()
        .0;
    assert_eq!(inspected, looked_up);
    assert_eq!(looked_up.schema_sha256, core.descriptor().schema_sha256);
    assert_eq!(looked_up.field_count, core.descriptor().fields.len());

    let path = core.descriptor().fields[0].logical_path;
    let field = service
        .lookup_field(Parameters(FieldInput {
            message_id: MESSAGE_ID.to_owned(),
            logical_path: path.to_owned(),
        }))
        .unwrap()
        .0;
    let expected = rust_iso20022::catalogue::lookup_field(MESSAGE_ID, path).unwrap();
    assert_eq!(field.logical_path, expected.logical_path);
    assert_eq!(field.type_name, expected.type_name);
    assert_eq!(field.min_occurs, expected.min_occurs);
    assert_eq!(field.max_occurs, expected.max_occurs);
    assert_eq!(field.kind, normalized(expected.kind).as_str().unwrap());
}

#[test]
fn validation_matches_the_generated_message_binding() {
    let service = Iso20022Mcp::new();
    for (xml, expected_valid) in [(VALID, true), (INVALID, false)] {
        let actual = service
            .validate_message(Parameters(ValidateInput {
                message: xml.to_owned(),
                layer: Some("l2".to_owned()),
                profile_release: None,
            }))
            .unwrap()
            .0;
        let parsed = rust_iso20022::parse(xml).unwrap();
        let expected =
            rust_iso20022::validation::bindings::validate_parsed_message(&parsed).unwrap();
        assert_eq!(actual.valid, expected_valid);
        assert_eq!(actual.valid, expected.valid());
        assert_eq!(actual.complete, expected.complete());
        assert_eq!(actual.errors.len(), expected.errors().len());
        assert_eq!(actual.warnings.len(), expected.warnings().len());
        assert_eq!(
            actual
                .errors
                .iter()
                .map(|issue| issue.rule_id.as_str())
                .collect::<Vec<_>>(),
            expected
                .errors()
                .into_iter()
                .map(|issue| issue.rule_id.as_str())
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn compare_and_explain_are_normalized_core_results() {
    let service = Iso20022Mcp::new();
    let actual = service
        .compare_versions(Parameters(CompareInput {
            from_message_id: "pacs.008.001.08".to_owned(),
            to_message_id: "pacs.008.001.14".to_owned(),
        }))
        .unwrap()
        .0;
    let core =
        rust_iso20022::compare::compare_versions("pacs.008.001.08", "pacs.008.001.14").unwrap();
    assert_eq!(normalized(actual), normalized(core));

    let actual = service
        .explain_validation_error(Parameters(ExplainInput {
            rule_id: RULE_ID.to_owned(),
        }))
        .unwrap()
        .0;
    let registry =
        rust_iso20022::validation::RuleRegistry::new(rust_iso20022::validation::financial_rules())
            .unwrap();
    let core = registry.explain_details_str(RULE_ID).unwrap();
    assert_eq!(normalized(actual), normalized(core));
}

#[test]
fn unavailable_requests_fail_explicitly() {
    let service = Iso20022Mcp::new();
    let l1 = service.validate_message(Parameters(ValidateInput {
        message: VALID.to_owned(),
        layer: Some("l1".to_owned()),
        profile_release: None,
    }));
    assert_eq!(
        error_of(l1),
        "requested validation layer or profile release is unavailable"
    );

    let incomplete = service.validate_message(Parameters(ValidateInput {
        message: VALID.to_owned(),
        layer: None,
        profile_release: Some("cbpr-plus:2026".to_owned()),
    }));
    assert_eq!(
        error_of(incomplete),
        "profile release key is incomplete or invalid"
    );

    assert_eq!(
        json!(error_of(service.lookup_message(Parameters(
            MessageIdInput {
                message_id: "unknown.001.001.01".to_owned(),
            }
        )))),
        "unknown ISO 20022 message"
    );
}
