use rust_iso20022::profiles::sepa::{
    DocumentKind, DocumentStatus, SepaRelease, SepaReleaseError, SepaScheme, SourceDocument,
    public_sct_2025, public_sct_inst_2025,
};

#[test]
fn public_2025_releases_keep_rulebook_and_guideline_editions_distinct() {
    let sct = public_sct_2025().unwrap();
    assert_eq!(sct.scheme(), SepaScheme::Sct);
    assert_eq!(sct.rulebook().document_id(), "EPC125-05");
    assert_eq!(sct.rulebook().edition(), "2025-v1.1");
    assert_eq!(sct.rulebook().kind(), DocumentKind::Rulebook);
    assert!(sct.guidelines().iter().all(|source| {
        source.edition() == "2025-v1.0" && source.kind() != DocumentKind::Rulebook
    }));

    let instant = public_sct_inst_2025().unwrap();
    assert_eq!(instant.scheme(), SepaScheme::SctInst);
    assert_eq!(instant.rulebook().document_id(), "EPC004-16");
    assert_eq!(instant.rulebook().edition(), "2025-v1.1");
    assert_ne!(
        sct.rulebook().document_id(),
        instant.rulebook().document_id()
    );
}

#[test]
fn consultation_material_cannot_become_an_effective_release() {
    let consultation = SourceDocument::new(
        SepaScheme::Sct,
        "EPC008-26",
        "2026-consultation-v1.0",
        DocumentKind::Rulebook,
        DocumentStatus::Consultation,
        "2026-03-13",
        None,
        "https://www.europeanpaymentscouncil.eu/document-library/rulebooks/",
        None,
    )
    .unwrap();
    let guideline = public_sct_2025().unwrap().guidelines()[0].clone();
    assert_eq!(
        SepaRelease::new(SepaScheme::Sct, consultation, vec![guideline]),
        Err(SepaReleaseError::RulebookNotEffective)
    );
}

#[test]
fn sct_and_sct_inst_sources_cannot_be_mixed() {
    let sct = public_sct_2025().unwrap();
    let instant_guideline = public_sct_inst_2025().unwrap().guidelines()[0].clone();
    assert_eq!(
        SepaRelease::new(
            SepaScheme::Sct,
            sct.rulebook().clone(),
            vec![instant_guideline],
        ),
        Err(SepaReleaseError::SchemeMismatch)
    );
}

#[test]
fn metadata_only_sources_are_not_production_ready_without_content_digests() {
    let release = public_sct_2025().unwrap();
    assert!(release.is_effective());
    assert!(!release.is_production_ready());
    assert!(release.missing_digests().contains(&"EPC125-05"));
}
