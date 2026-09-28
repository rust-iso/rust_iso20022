use rust_iso20022::profiles::provisional::{
    cbpr_plus_provisional_rules, sepa_sct_inst_provisional_rules, sepa_sct_provisional_rules,
};
use rust_iso20022::validation::{RuleRegistry, ValidationContext, ValidationTarget};

#[test]
fn cbpr_provisional_rules_validate_scalar_fields_without_claiming_release_support() {
    let rules = cbpr_plus_provisional_rules();
    assert_eq!(rules.version, Some("provisional"));
    let registry = RuleRegistry::from_rule_sets(&[rules]).unwrap();
    let valid = ValidationTarget::from_pairs(&[
        ("$financial.iban", "DE89370400440532013000"),
        ("$financial.bic", "COBADEFFXXX"),
        ("$financial.currency", "EUR"),
    ]);
    assert!(
        registry
            .validate(&valid, &ValidationContext::default())
            .valid()
    );

    let invalid = ValidationTarget::from_pairs(&[
        ("$financial.iban", "DE89370400440532013001"),
        ("$financial.bic", "bad"),
        ("$financial.currency", "ZZZ"),
    ]);
    let report = registry.validate(&invalid, &ValidationContext::default());
    assert!(!report.valid());
    assert_eq!(report.errors().len(), 3);
    assert!(
        report
            .errors()
            .iter()
            .all(|issue| issue.profile == Some("cbpr-plus")
                && issue.profile_version == Some("provisional"))
    );
}

#[test]
fn sepa_provisional_rules_are_separate_from_cbpr_rules() {
    let rules = sepa_sct_provisional_rules();
    let registry = RuleRegistry::from_rule_sets(&[rules]).unwrap();
    let report = registry.validate(
        &ValidationTarget::from_pairs(&[
            ("$financial.iban", "DE89370400440532013000"),
            ("$financial.currency", "EUR"),
        ]),
        &ValidationContext::default(),
    );
    assert!(report.valid());
    assert!(registry.explain_str("SEPA-SCT-PROVISIONAL-IBAN").is_some());
    assert!(registry.explain_str("CBPRPLUS-PROVISIONAL-IBAN").is_none());
}

#[test]
fn candidate_cross_field_rules_are_executable_and_explainable() {
    let rules = cbpr_plus_provisional_rules();
    let registry = RuleRegistry::from_rule_sets(&[rules]).unwrap();
    let report = registry.validate(
        &ValidationTarget::from_pairs(&[
            ("$header.biz_msg_id", "header-id"),
            ("$message.group_msg_id", "group-id"),
            ("$party.any_bic", "COBADEFFXXX"),
            ("$party.name", "Example"),
            ("$settlement.currency", "XAU"),
            ("$remittance.structured", "x"),
        ]),
        &ValidationContext::default(),
    );
    assert_eq!(report.errors().len(), 3);
    assert!(
        registry
            .explain_str("CBPRPLUS-PROVISIONAL-ANYBIC-EXCLUSIVE")
            .is_some()
    );
}

#[test]
fn instant_limit_is_not_reused_by_sct() {
    let inst = RuleRegistry::from_rule_sets(&[sepa_sct_inst_provisional_rules()]).unwrap();
    let report = inst.validate(
        &ValidationTarget::from_pairs(&[
            ("$financial.iban", "DE89370400440532013000"),
            ("$financial.currency", "EUR"),
            ("$settlement.currency", "EUR"),
            ("$settlement.amount", "100000.01"),
        ]),
        &ValidationContext::default(),
    );
    assert!(!report.valid());
    assert!(
        report
            .errors()
            .iter()
            .any(|issue| issue.rule_id.as_str() == "SEPA-SCT-INST-PROVISIONAL-AMOUNT-LIMIT")
    );

    let sct = RuleRegistry::from_rule_sets(&[sepa_sct_provisional_rules()]).unwrap();
    let report = sct.validate(
        &ValidationTarget::from_pairs(&[("$settlement.currency", "EUR")]),
        &ValidationContext::default(),
    );
    assert!(report.valid());
}
