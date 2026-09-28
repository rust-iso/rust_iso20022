use rust_iso20022::validation::{
    FieldPath, LayerAvailability, Rule, RuleDescriptor, RuleId, RuleRegistry, RuleSet, Severity,
    ValidationContext, ValidationIssue, ValidationLayer, ValidationPipeline, ValidationReport,
    ValidationTarget,
};

struct MissingMessageId;

static MISSING_MESSAGE_ID: RuleDescriptor = RuleDescriptor {
    id: RuleId::new("ISO20022-L2-MSGID-REQUIRED"),
    layer: ValidationLayer::IsoSemantic,
    title: "Message identifier is required",
    reason: "Business messages require a non-empty message identifier.",
    source: Some("ISO 20022 business-message semantics"),
    profile: None,
    profile_version: None,
    affected_messages: &["pacs.008.*"],
};

impl Rule for MissingMessageId {
    fn descriptor(&self) -> &'static RuleDescriptor {
        &MISSING_MESSAGE_ID
    }

    fn evaluate(
        &self,
        target: &ValidationTarget<'_>,
        _: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue> {
        if target.text("Document/FIToFICstmrCdtTrf/GrpHdr/MsgId") == Some("") {
            vec![ValidationIssue::new(
                &MISSING_MESSAGE_ID,
                Severity::Error,
                FieldPath::new("Document/FIToFICstmrCdtTrf/GrpHdr/MsgId"),
                "message identifier must not be empty",
            )]
        } else {
            Vec::new()
        }
    }
}

#[test]
fn report_derives_validity_and_partitions_deterministically() {
    let mut report = ValidationReport::new();
    report.push(ValidationIssue::standalone(
        "Z-WARN",
        ValidationLayer::Profile,
        Severity::Warning,
        "Document/Z",
        "warning",
    ));
    report.push(ValidationIssue::standalone(
        "A-ERROR",
        ValidationLayer::IsoSemantic,
        Severity::Error,
        "Document/A",
        "error",
    ));
    report.sort();

    assert!(!report.valid());
    assert_eq!(report.errors().len(), 1);
    assert_eq!(report.warnings().len(), 1);
    assert_eq!(report.issues()[0].rule_id.as_str(), "A-ERROR");
}

#[test]
fn unavailable_layers_are_explicit_and_do_not_pretend_to_pass() {
    let mut report = ValidationReport::new();
    report.mark_unavailable(
        ValidationLayer::SyntaxSchema,
        "no reviewed XSD backend is enabled",
    );

    assert!(!report.complete());
    assert_eq!(report.unavailable_layers().len(), 1);
    assert_eq!(
        report.unavailable_layers()[0].layer,
        ValidationLayer::SyntaxSchema
    );
}

#[test]
fn execution_and_explain_use_the_same_registry_descriptor() {
    let registry = RuleRegistry::new(&[&MissingMessageId]).expect("unique registry");
    let target = ValidationTarget::from_pairs(&[("Document/FIToFICstmrCdtTrf/GrpHdr/MsgId", "")]);
    let report = registry.validate(&target, &ValidationContext::default());
    let explained = registry
        .explain(RuleId::new("ISO20022-L2-MSGID-REQUIRED"))
        .expect("registered rule");

    assert_eq!(report.errors()[0].rule_id, explained.id);
    assert!(std::ptr::eq(explained, &MISSING_MESSAGE_ID));
    assert!(registry.explain(RuleId::new("UNKNOWN")).is_none());
}

#[test]
fn json_contract_contains_structured_fields_without_input_values() {
    let descriptor = RuleDescriptor {
        id: RuleId::new("CBPRPLUS-2026-R1234"),
        layer: ValidationLayer::Profile,
        title: "Example profile rule",
        reason: "A field relationship required by the profile.",
        source: Some("CBPR+ 2026 rulebook R1234"),
        profile: Some("CBPR+"),
        profile_version: Some("2026"),
        affected_messages: &["pacs.008.001.08"],
    };
    let mut report = ValidationReport::new();
    report.push(ValidationIssue::new(
        &descriptor,
        Severity::Error,
        FieldPath::new("Document/FIToFICstmrCdtTrf/CdtTrfTxInf/CdtrAcct"),
        "creditor account is required",
    ));

    let json = serde_json::to_value(&report).expect("report JSON");
    let issue = &json["errors"][0];
    assert_eq!(json["valid"], false);
    assert_eq!(issue["code"], "CBPRPLUS-2026-R1234");
    assert_eq!(issue["severity"], "error");
    assert_eq!(issue["rule_id"], "CBPRPLUS-2026-R1234");
    assert_eq!(issue["profile"], "CBPR+");
    assert_eq!(issue["profile_version"], "2026");
    assert_eq!(issue["source"], "CBPR+ 2026 rulebook R1234");
    assert!(!json.to_string().contains("DE89370400440532013000"));
}

#[test]
fn duplicate_rule_ids_are_rejected() {
    let duplicate = RuleRegistry::new(&[&MissingMessageId, &MissingMessageId]);
    let error = duplicate.expect_err("duplicate IDs must fail");
    assert_eq!(error.rule_id.as_str(), "ISO20022-L2-MSGID-REQUIRED");
}

#[test]
fn pipeline_runs_available_layers_and_reports_unavailable_ones() {
    let iso = RuleSet::new(
        "ISO 20022 semantics",
        None,
        ValidationLayer::IsoSemantic,
        &[&MissingMessageId],
    )
    .expect("consistent rule set");
    let registry = RuleRegistry::from_rule_sets(&[iso]).expect("unique rule IDs");
    let pipeline = ValidationPipeline::new(registry)
        .with_layer(
            ValidationLayer::SyntaxSchema,
            LayerAvailability::Unavailable("reviewed XSD backend not enabled"),
        )
        .with_layer(ValidationLayer::IsoSemantic, LayerAvailability::Available)
        .with_layer(
            ValidationLayer::Profile,
            LayerAvailability::Unavailable("no profile release selected"),
        );
    let target = ValidationTarget::from_pairs(&[("Document/FIToFICstmrCdtTrf/GrpHdr/MsgId", "")]);
    let report = pipeline.validate(&target, &ValidationContext::default());

    assert_eq!(report.errors().len(), 1);
    assert_eq!(report.unavailable_layers().len(), 2);
    assert!(!report.complete());
    assert!(!report.valid());
}

#[test]
fn profile_rule_sets_require_an_explicit_matching_release() {
    let error = RuleSet::new(
        "CBPR+",
        None,
        ValidationLayer::Profile,
        &[&MissingMessageId],
    )
    .expect_err("profile release is mandatory");
    assert_eq!(
        error.to_string(),
        "profile rule set requires an explicit version"
    );
}
