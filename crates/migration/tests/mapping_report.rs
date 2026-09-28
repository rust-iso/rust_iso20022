use rust_iso20022_migration::{
    mt::MtDocument,
    report::{
        MappingClassification::*, MappingEntry, MappingReport, MappingReportError, MappingWarning,
        SourceFieldRef,
    },
};

const MT: &str = "{4:\n:20:SECRET-REFERENCE\n:32A:260928EUR1,00\n:50K:SECRET-NAME\n:59:SECRET-ACCOUNT\n:72:FREE TEXT\n-}";

fn entry(
    document: &MtDocument,
    index: usize,
    classification: rust_iso20022_migration::report::MappingClassification,
) -> MappingEntry {
    MappingEntry::new(
        SourceFieldRef::from_document(document, index).unwrap(),
        (!matches!(classification, Ambiguous | Unsupported))
            .then(|| format!("Document/Target/{index}")),
        classification,
        "value-free mapping rationale",
    )
}

#[test]
fn report_orders_and_accounts_for_every_field_with_all_classifications() {
    let document = MtDocument::parse(MT).unwrap();
    let report = MappingReport::build(
        &document,
        vec![
            entry(&document, 4, Unsupported),
            entry(&document, 2, Lossy),
            entry(&document, 0, Exact),
            entry(&document, 3, Ambiguous),
            entry(&document, 1, Derived),
        ],
        vec![MappingWarning {
            code: "LOSS".to_owned(),
            message: "field requires review".to_owned(),
            source_index: Some(2),
        }],
    )
    .unwrap();
    assert_eq!(
        report
            .entries()
            .iter()
            .map(|e| e.source.index)
            .collect::<Vec<_>>(),
        [0, 1, 2, 3, 4]
    );
    assert_eq!(
        report
            .unmapped_fields()
            .iter()
            .map(|f| f.index)
            .collect::<Vec<_>>(),
        [3, 4]
    );
    let json = serde_json::to_string(&report).unwrap();
    for secret in [
        "SECRET-REFERENCE",
        "SECRET-NAME",
        "SECRET-ACCOUNT",
        "FREE TEXT",
    ] {
        assert!(!json.contains(secret));
    }
}

#[test]
fn missing_duplicate_unknown_and_mismatched_sources_are_rejected() {
    let document = MtDocument::parse(MT).unwrap();
    assert_eq!(
        MappingReport::build(&document, vec![], vec![]).unwrap_err(),
        MappingReportError::MissingSource { index: 0 }
    );

    let mut complete: Vec<_> = (0..document.fields().len())
        .map(|i| entry(&document, i, Exact))
        .collect();
    complete.push(entry(&document, 0, Exact));
    assert_eq!(
        MappingReport::build(&document, complete, vec![]).unwrap_err(),
        MappingReportError::DuplicateSource { index: 0 }
    );

    let mut complete: Vec<_> = (0..document.fields().len())
        .map(|i| entry(&document, i, Exact))
        .collect();
    complete[0].source.index = 99;
    assert_eq!(
        MappingReport::build(&document, complete, vec![]).unwrap_err(),
        MappingReportError::UnknownSource { index: 99 }
    );

    let mut complete: Vec<_> = (0..document.fields().len())
        .map(|i| entry(&document, i, Exact))
        .collect();
    complete[0].source.tag = "21".to_owned();
    assert_eq!(
        MappingReport::build(&document, complete, vec![]).unwrap_err(),
        MappingReportError::SourceMismatch { index: 0 }
    );
}
