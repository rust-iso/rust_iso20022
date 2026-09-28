use rust_iso20022::from_xml;
use rust_iso20022::generated::pacs::pacs_008_001_08::Document;
use rust_iso20022::validation::bindings::{
    pacs_008_001_08_binding_descriptor, validate_pacs_008_001_08,
};

const VALID: &str = include_str!("../../fixtures/iso/valid/pacs.008.001.08-cross-field.xml");
const INVALID: &str = include_str!("../../fixtures/iso/invalid/pacs.008.001.08-cross-field.xml");
const EXPECTED: &str =
    include_str!("../../fixtures/iso/invalid/pacs.008.001.08-cross-field.expected.json");

#[test]
fn binding_fields_are_audited_against_schema_derived_metadata() {
    let descriptor = pacs_008_001_08_binding_descriptor().expect("pacs.008 descriptor");
    assert_eq!(
        descriptor.generated_module,
        "src/generated/pacs/pacs_008_001_08.rs"
    );
    for wire_name in [
        "NbOfTxs",
        "CdtTrfTxInf",
        "InstdAmt",
        "XchgRate",
        "IBAN",
        "Othr",
        "IntrBkSttlmAmt",
        "DbtrAcct",
        "DbtrAgt",
    ] {
        assert!(
            descriptor
                .fields
                .iter()
                .any(|field| field.wire_name == wire_name),
            "missing schema field {wire_name}"
        );
    }
}

#[test]
fn direct_generated_pacs008_value_passes_cross_field_rules() {
    let document: Document = from_xml(VALID).expect("valid generated pacs.008 fixture");
    let report = validate_pacs_008_001_08(&document).unwrap();
    assert!(report.issues().is_empty(), "{report:?}");
}

#[test]
fn direct_generated_pacs008_value_emits_all_expected_ids_and_paths() {
    let document: Document = from_xml(INVALID).expect("parse generated pacs.008 fixture");
    let report = validate_pacs_008_001_08(&document).unwrap();
    let actual: Vec<_> = report
        .issues()
        .iter()
        .map(|issue| {
            serde_json::json!({
                "rule_id": issue.rule_id.as_str(),
                "path": issue.path.as_str(),
            })
        })
        .collect();
    let expected: serde_json::Value = serde_json::from_str(EXPECTED).unwrap();
    assert_eq!(serde_json::Value::Array(actual), expected["issues"]);
    assert!(!format!("{report:?}").contains("SECRET-OTHER-ACCOUNT"));
}
