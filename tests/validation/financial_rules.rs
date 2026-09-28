use rust_iso20022::validation::{
    RuleId, RuleRegistry, ValidationContext, ValidationLayer, ValidationTarget, financial_rules,
};

#[test]
fn registered_financial_rules_emit_stable_ids_paths_and_safe_messages() {
    let registry = RuleRegistry::new(financial_rules()).unwrap();
    let fields = [
        ("$financial.iban", "DE89A70400440532013000"),
        ("$financial.bic", "DEUTZZFF"),
        ("$financial.currency", "ANG"),
        ("$financial.amount_currency", "EUR"),
        ("$financial.country", "ZZ"),
        ("$financial.amount", "1.234"),
        ("$financial.date", "2023-02-29"),
    ];
    let report = registry.validate(
        &ValidationTarget::from_pairs(&fields),
        &ValidationContext::default(),
    );

    for (id, path) in [
        ("ISO20022-L2-IBAN-BBAN-FORMAT", "$financial.iban"),
        ("ISO20022-L2-BIC-COUNTRY", "$financial.bic"),
        ("ISO20022-L2-CURRENCY-CODE", "$financial.currency"),
        ("ISO20022-L2-COUNTRY-CODE", "$financial.country"),
        ("ISO20022-L2-AMOUNT-PRECISION", "$financial.amount"),
        ("ISO20022-L2-DATE-TIME-RANGE", "$financial.date"),
    ] {
        let issue = report
            .issues()
            .iter()
            .find(|issue| issue.rule_id.as_str() == id)
            .unwrap_or_else(|| panic!("missing {id}"));
        assert_eq!(issue.path.as_str(), path);
        assert!(!issue.message.contains(fields[0].1));
    }
}

#[test]
fn execution_and_explanation_use_the_same_rule_objects() {
    let registry = RuleRegistry::new(financial_rules()).unwrap();
    let descriptor = registry
        .explain(RuleId::new("ISO20022-L2-IBAN-CHECKSUM"))
        .expect("checksum explanation");
    assert_eq!(descriptor.layer, ValidationLayer::IsoSemantic);
    assert!(descriptor.source.unwrap().contains("SWIFT"));

    let fields = [
        ("$financial.iban", "DE89370400440532013000"),
        ("$financial.bic", "DEUTDEFF"),
        ("$financial.currency", "EUR"),
        ("$financial.country", "DE"),
        ("$financial.amount", "1.23"),
        ("$financial.date", "2024-02-29"),
    ];
    let report = registry.validate(
        &ValidationTarget::from_pairs(&fields),
        &ValidationContext::default(),
    );
    assert!(report.issues().is_empty());
}
