use std::process::{Command, Output, Stdio};

use rust_iso20022::validation::{
    Rule, RuleDescriptor, RuleId, RuleRegistry, ValidationContext, ValidationIssue,
    ValidationLayer, ValidationTarget,
};

const L2_RULE: &str = "ISO20022-L2-IBAN-CHECKSUM";
const L3_RULE: &str = "TESTNETWORK-2025-R0001";

struct ProfileRule;

static PROFILE_RULE: ProfileRule = ProfileRule;
static PROFILE_DESCRIPTOR: RuleDescriptor = RuleDescriptor {
    id: RuleId::new(L3_RULE),
    layer: ValidationLayer::Profile,
    title: "Synthetic profile explanation contract",
    reason: "Proves generic L3 mapping without shipping an unlicensed market rule.",
    source: Some("tests/fixtures/profile-source#R0001"),
    profile: Some("test-network"),
    profile_version: Some("2025.1/ig-2025.1"),
    affected_messages: &["pacs.008.001.08"],
};

impl Rule for ProfileRule {
    fn descriptor(&self) -> &'static RuleDescriptor {
        &PROFILE_DESCRIPTOR
    }

    fn field_paths(&self) -> Vec<&'static str> {
        vec!["Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Test"]
    }

    fn evaluate(
        &self,
        _target: &ValidationTarget<'_>,
        _context: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue> {
        Vec::new()
    }
}

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_iso20022"))
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap()
}

#[test]
fn l2_json_is_object_equal_to_the_executed_core_descriptor() {
    let output = run(&["explain", "--json", L2_RULE]);
    assert_eq!(output.status.code(), Some(0));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();

    let registry = RuleRegistry::new(rust_iso20022::validation::financial_rules()).unwrap();
    let descriptor = registry.explain_details_str(L2_RULE).unwrap();
    assert_eq!(
        value["data"],
        serde_json::to_value(descriptor).unwrap(),
        "the adapter must serialize the descriptor used by core execution"
    );
    assert_eq!(
        value["data"]["source"],
        "ISO 13616 and SWIFT IBAN Registry release 102 (June 2026)"
    );
    assert_eq!(value["data"]["field_paths"][0], "$financial.iban");
    assert_eq!(value["data"]["profile"], serde_json::Value::Null);
}

#[test]
fn generic_l3_mapping_preserves_profile_version_source_and_messages() {
    let registry = RuleRegistry::new(&[&PROFILE_RULE]).unwrap();
    let expected = registry.explain_details_str(L3_RULE).unwrap();
    let actual =
        rust_iso20022_cli::commands::explain_from_rules(&[&PROFILE_RULE], L3_RULE).unwrap();

    assert_eq!(actual, expected);
    let value = serde_json::to_value(&actual).unwrap();
    assert_eq!(value["profile"], "test-network");
    assert_eq!(value["profile_version"], "2025.1/ig-2025.1");
    assert_eq!(value["source"], "tests/fixtures/profile-source#R0001");
    assert_eq!(value["affected_messages"][0], "pacs.008.001.08");
    assert_eq!(
        value["field_paths"][0],
        "Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Test"
    );
}

#[test]
fn human_output_is_traceable_and_unknown_ids_are_typed_and_redacted() {
    let human = run(&["explain", L2_RULE]);
    assert_eq!(human.status.code(), Some(0));
    let stdout = String::from_utf8(human.stdout).unwrap();
    assert!(stdout.contains(L2_RULE));
    assert!(stdout.contains("IBAN checksum"));
    assert!(stdout.contains("ISO 13616"));

    let canary = "SECRET-ACCOUNT-RULE";
    let unknown = run(&["explain", "--json", canary]);
    assert_eq!(unknown.status.code(), Some(5));
    let value: serde_json::Value = serde_json::from_slice(&unknown.stdout).unwrap();
    assert_eq!(value["error"]["code"], "rule_not_found");
    assert!(!String::from_utf8(unknown.stderr).unwrap().contains(canary));
    assert!(!value.to_string().contains(canary));
}
