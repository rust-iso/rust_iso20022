use rust_iso20022::profiles::public_reference::{
    ReferenceStatus, cbpr_plus_public_reference, sepa_sct_inst_public_reference,
    sepa_sct_public_reference,
};
use rust_iso20022::validation::{ValidationContext, ValidationTarget};

#[test]
fn public_reference_bundles_are_explicitly_non_authoritative() {
    let cbpr = cbpr_plus_public_reference();
    assert_eq!(cbpr.scheme(), "cbpr-plus");
    assert_eq!(cbpr.release(), "SR2026");
    assert_eq!(cbpr.guideline(), "public-reference");
    assert_eq!(cbpr.status(), ReferenceStatus::PublicReference);
    assert!(!cbpr.source_references().is_empty());

    let report = cbpr.into_registry().validate(
        &ValidationTarget::from_pairs(&[
            ("$financial.iban", "DE89370400440532013000"),
            ("$financial.bic", "COBADEFFXXX"),
            ("$financial.currency", "EUR"),
        ]),
        &ValidationContext::default(),
    );
    assert!(report.valid());
}

#[test]
fn sepa_public_reference_bundles_remain_separate() {
    let sct = sepa_sct_public_reference();
    let inst = sepa_sct_inst_public_reference();
    assert_eq!(sct.scheme(), "sepa-sct");
    assert_eq!(inst.scheme(), "sepa-sct-inst");
    assert_ne!(sct.rules().name, inst.rules().name);
    assert!(
        sct.into_registry()
            .explain_str("SEPA-SCT-PROVISIONAL-AMOUNT-LIMIT")
            .is_none()
    );
    assert!(
        inst.into_registry()
            .explain_str("SEPA-SCT-INST-PROVISIONAL-AMOUNT-LIMIT")
            .is_some()
    );
}

#[test]
fn every_public_reference_rule_has_explainable_id_and_path() {
    let cases = [
        (
            cbpr_plus_public_reference().into_registry(),
            &[
                ("CBPRPLUS-PROVISIONAL-IBAN", "$financial.iban"),
                ("CBPRPLUS-PROVISIONAL-BIC", "$financial.bic"),
                ("CBPRPLUS-PROVISIONAL-CURRENCY", "$financial.currency"),
                (
                    "CBPRPLUS-PROVISIONAL-HEADER-MESSAGE-ID",
                    "$header.biz_msg_id",
                ),
                ("CBPRPLUS-PROVISIONAL-ANYBIC-EXCLUSIVE", "$party.any_bic"),
                (
                    "CBPRPLUS-PROVISIONAL-REMITTANCE-LENGTH",
                    "$remittance.structured",
                ),
                (
                    "CBPRPLUS-PROVISIONAL-COMMODITY-CURRENCY",
                    "$settlement.currency",
                ),
            ][..],
        ),
        (
            sepa_sct_public_reference().into_registry(),
            &[
                ("SEPA-SCT-PROVISIONAL-IBAN", "$financial.iban"),
                ("SEPA-SCT-PROVISIONAL-CURRENCY", "$financial.currency"),
                ("SEPA-SCT-PROVISIONAL-EUR", "$settlement.currency"),
            ][..],
        ),
        (
            sepa_sct_inst_public_reference().into_registry(),
            &[
                ("SEPA-SCT-INST-PROVISIONAL-IBAN", "$financial.iban"),
                ("SEPA-SCT-INST-PROVISIONAL-CURRENCY", "$settlement.currency"),
                (
                    "SEPA-SCT-INST-PROVISIONAL-AMOUNT-LIMIT",
                    "$settlement.amount",
                ),
            ][..],
        ),
    ];

    for (registry, expected) in cases {
        for &(rule_id, path) in expected {
            let explanation = registry
                .explain_details_str(rule_id)
                .expect("public-reference rule must be registered");
            assert_eq!(explanation.descriptor.id.as_str(), rule_id);
            assert!(
                explanation
                    .field_paths
                    .iter()
                    .any(|candidate| **candidate == *path)
            );
            assert_eq!(explanation.descriptor.profile_version, Some("provisional"));
        }
    }
}
