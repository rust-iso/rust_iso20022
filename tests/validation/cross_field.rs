use rust_iso20022::validation::cross_field::{
    AgentAccountRule, AtLeastOneRule, CardinalityDependencyRule, CurrencyAmountRule,
    FieldPredicate, FieldRelationshipRule, MutualExclusionRule, RequiredIfRule,
};
use rust_iso20022::validation::{
    Rule, RuleDescriptor, RuleId, RuleRegistry, ValidationContext, ValidationLayer,
    ValidationTarget,
};

fn descriptor(id: &'static str) -> &'static RuleDescriptor {
    Box::leak(Box::new(RuleDescriptor {
        id: RuleId::new(id),
        layer: ValidationLayer::IsoSemantic,
        title: "cross-field test rule",
        reason: "configured fields do not satisfy their relationship",
        source: Some("test contract"),
        profile: None,
        profile_version: None,
        affected_messages: &[],
    }))
}

fn issues(rule: &dyn Rule, fields: &[(&str, &str)]) -> Vec<String> {
    RuleRegistry::new(&[rule])
        .unwrap()
        .validate(
            &ValidationTarget::from_pairs(fields),
            &ValidationContext::default(),
        )
        .issues()
        .iter()
        .map(|issue| issue.path.as_str().to_owned())
        .collect()
}

#[test]
fn required_if_truth_table() {
    let rule = RequiredIfRule::new(
        descriptor("TEST-REQUIRED-IF"),
        FieldPredicate::equals("payment.method", "WIRE"),
        "payment.account",
    );
    assert!(issues(&rule, &[]).is_empty());
    assert!(issues(&rule, &[("payment.method", "CASH")]).is_empty());
    assert_eq!(
        issues(&rule, &[("payment.method", "WIRE")]),
        ["payment.account"]
    );
    assert!(
        issues(
            &rule,
            &[("payment.method", "WIRE"), ("payment.account", "present")]
        )
        .is_empty()
    );
}

#[test]
fn mutual_exclusion_and_at_least_one_truth_tables() {
    let xor = MutualExclusionRule::new(descriptor("TEST-XOR"), &["party.iban", "party.other"]);
    assert!(issues(&xor, &[]).is_empty());
    assert!(issues(&xor, &[("party.iban", "present")]).is_empty());
    assert_eq!(
        issues(
            &xor,
            &[("party.iban", "secret-a"), ("party.other", "secret-b")]
        ),
        ["party.iban", "party.other"]
    );

    let any = AtLeastOneRule::new(descriptor("TEST-ANY"), &["party.name", "party.id"]);
    assert_eq!(issues(&any, &[]), ["party.name"]);
    assert!(issues(&any, &[("party.id", "present")]).is_empty());
}

#[test]
fn cardinality_and_general_relationship_truth_tables() {
    let cardinality = CardinalityDependencyRule::new(
        descriptor("TEST-CARDINALITY"),
        "transactions",
        "controls",
        1..=1,
    );
    assert!(issues(&cardinality, &[]).is_empty());
    assert_eq!(
        issues(&cardinality, &[("transactions", "one")]),
        ["controls"]
    );
    assert!(
        issues(
            &cardinality,
            &[("transactions", "one"), ("controls", "one")]
        )
        .is_empty()
    );
    assert_eq!(
        issues(
            &cardinality,
            &[
                ("transactions", "one"),
                ("controls", "one"),
                ("controls", "two")
            ]
        ),
        ["controls"]
    );

    let same = FieldRelationshipRule::new(
        descriptor("TEST-RELATION"),
        "settlement.currency",
        "instructed.currency",
        |left, right| left == right,
    );
    assert!(issues(&same, &[]).is_empty());
    assert!(
        issues(
            &same,
            &[
                ("settlement.currency", "EUR"),
                ("instructed.currency", "EUR")
            ]
        )
        .is_empty()
    );
    assert_eq!(
        issues(
            &same,
            &[
                ("settlement.currency", "EUR"),
                ("instructed.currency", "USD")
            ]
        ),
        ["instructed.currency"]
    );
}

#[test]
fn currency_amount_and_agent_account_relationships() {
    let money = CurrencyAmountRule::new(
        descriptor("TEST-CURRENCY-AMOUNT"),
        "amount.currency",
        "amount.value",
    );
    assert!(
        issues(
            &money,
            &[("amount.currency", "JPY"), ("amount.value", "100")]
        )
        .is_empty()
    );
    assert_eq!(
        issues(
            &money,
            &[("amount.currency", "JPY"), ("amount.value", "100.01")]
        ),
        ["amount.value"]
    );

    let agent = AgentAccountRule::new(
        descriptor("TEST-AGENT-ACCOUNT"),
        "creditor.account",
        "creditor.agent",
    );
    assert!(issues(&agent, &[]).is_empty());
    assert_eq!(
        issues(&agent, &[("creditor.account", "masked")]),
        ["creditor.agent"]
    );
    assert!(
        issues(
            &agent,
            &[("creditor.account", "masked"), ("creditor.agent", "masked")]
        )
        .is_empty()
    );
}

#[test]
fn diagnostics_are_deterministic_and_never_echo_values() {
    let required = RequiredIfRule::new(
        descriptor("TEST-Z-REQUIRED"),
        FieldPredicate::present("trigger"),
        "z.required",
    );
    let any = AtLeastOneRule::new(descriptor("TEST-A-ANY"), &["a.first", "a.second"]);
    let registry = RuleRegistry::new(&[&required, &any]).unwrap();
    let report = registry.validate(
        &ValidationTarget::from_pairs(&[("trigger", "TOP-SECRET-VALUE")]),
        &ValidationContext::default(),
    );
    let ids: Vec<_> = report
        .issues()
        .iter()
        .map(|issue| issue.rule_id.as_str())
        .collect();
    assert_eq!(ids, ["TEST-A-ANY", "TEST-Z-REQUIRED"]);
    assert!(!format!("{report:?}").contains("TOP-SECRET-VALUE"));
}
