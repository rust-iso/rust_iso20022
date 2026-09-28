use rust_iso20022::compare::compare_descriptors;
use rust_iso20022::compare::{CompareError, SchemaChangeKind, compare_versions};
use rust_iso20022::metadata::{FieldDescriptor, FieldKind, MessageDescriptor};

#[test]
fn pacs_008_curated_pairs_record_real_changes_deterministically() {
    let diff = compare_versions("pacs.008.001.08", "pacs.008.001.14").unwrap();
    let adjacent = compare_versions("pacs.008.001.08", "pacs.008.001.10").unwrap();
    let latest = compare_versions("pacs.008.001.10", "pacs.008.001.14").unwrap();

    assert_eq!(diff.from.identity, "pacs.008.001.08");
    assert_eq!(diff.to.identity, "pacs.008.001.14");
    assert_eq!(diff.from.schema_sha256.len(), 64);
    assert_eq!(diff.to.schema_sha256.len(), 64);
    for kind in [
        SchemaChangeKind::FieldAdded,
        SchemaChangeKind::FieldRemoved,
        SchemaChangeKind::TypeChanged,
    ] {
        assert!(
            [&diff, &adjacent, &latest]
                .into_iter()
                .any(|pair| pair.changes.iter().any(|change| change.kind == kind)),
            "missing {kind:?} from curated pacs.008 pairs"
        );
    }

    let keys: Vec<_> = diff
        .changes
        .iter()
        .map(|change| (change.path.as_str(), change.kind))
        .collect();
    let mut sorted = keys.clone();
    sorted.sort();
    assert_eq!(keys, sorted, "changes must have stable path/kind order");
    assert_eq!(
        diff,
        compare_versions("pacs.008.001.08", "pacs.008.001.14").unwrap()
    );
    assert!(diff.changes.iter().any(|change| {
        change.path == "AdditionalDateTime1/AccptncDtTm"
            && change.kind == SchemaChangeKind::FieldAdded
    }));
    assert!(diff.changes.iter().any(|change| {
        change.path == "BranchData3/Id" && change.kind == SchemaChangeKind::FieldRemoved
    }));
    assert!(diff.changes.iter().any(|change| {
        change.path == "Document/FIToFICstmrCdtTrf"
            && change.kind == SchemaChangeKind::TypeChanged
            && change.before.as_deref() == Some("FIToFICustomerCreditTransferV08")
            && change.after.as_deref() == Some("FIToFICustomerCreditTransferV14")
    }));
}

fn field(
    path: &'static str,
    type_name: &'static str,
    min_occurs: u32,
    max_occurs: Option<u32>,
    enum_values: &'static [&'static str],
) -> FieldDescriptor {
    FieldDescriptor {
        logical_path: path,
        namespace: "urn:test",
        wire_name: path,
        rust_name: path,
        type_name,
        min_occurs,
        max_occurs,
        enum_values,
        documentation: None,
        choice_group: None,
        kind: FieldKind::Element,
    }
}

fn descriptor(identity: &'static str, fields: Vec<FieldDescriptor>) -> MessageDescriptor {
    MessageDescriptor {
        identity,
        business_area: "test",
        namespace: "urn:test",
        root_element: "Document",
        root_type: "Document",
        description: None,
        schema_path: "test.xsd",
        schema_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        generated_module: "generated/test.rs",
        required_feature: "model-test",
        fields: Box::leak(fields.into_boxed_slice()),
    }
}

#[test]
fn normalized_graph_fixture_covers_all_six_change_classes() {
    let from = descriptor(
        "test.001.001.01",
        vec![
            field("Document/Removed", "Text", 0, Some(1), &[]),
            field("Document/Type", "OldType", 1, Some(1), &[]),
            field("Document/Cardinality", "Text", 0, Some(1), &[]),
            field("Document/Code", "Code", 1, Some(1), &["OLD", "SAME"]),
        ],
    );
    let to = descriptor(
        "test.001.001.02",
        vec![
            field("Document/Added", "Text", 0, Some(1), &[]),
            field("Document/Type", "NewType", 1, Some(1), &[]),
            field("Document/Cardinality", "Text", 1, None, &[]),
            field("Document/Code", "Code", 1, Some(1), &["NEW", "SAME"]),
        ],
    );
    let diff = compare_descriptors(&from, &to).unwrap();
    let kinds: Vec<_> = diff.changes.iter().map(|change| change.kind).collect();
    for expected in [
        SchemaChangeKind::FieldAdded,
        SchemaChangeKind::FieldRemoved,
        SchemaChangeKind::TypeChanged,
        SchemaChangeKind::CardinalityChanged,
        SchemaChangeKind::EnumValueAdded,
        SchemaChangeKind::EnumValueRemoved,
    ] {
        assert!(kinds.contains(&expected), "missing {expected:?}");
    }
}

#[test]
fn reverse_pair_reverses_directional_changes_without_rename_inference() {
    let forward = compare_versions("pacs.008.001.08", "pacs.008.001.14").unwrap();
    let reverse = compare_versions("pacs.008.001.14", "pacs.008.001.08").unwrap();

    assert_eq!(
        forward
            .changes
            .iter()
            .filter(|change| change.kind == SchemaChangeKind::FieldAdded)
            .count(),
        reverse
            .changes
            .iter()
            .filter(|change| change.kind == SchemaChangeKind::FieldRemoved)
            .count()
    );
    assert!(forward.changes.iter().all(|change| {
        !change.path.to_ascii_lowercase().contains("rename")
            && !change
                .before
                .as_deref()
                .unwrap_or_default()
                .contains("renamed")
            && !change
                .after
                .as_deref()
                .unwrap_or_default()
                .contains("renamed")
    }));
}

#[test]
fn unknown_and_unrelated_messages_are_typed_errors() {
    assert_eq!(
        compare_versions("pacs.008.001.08", "not-a-message"),
        Err(CompareError::UnknownMessage {
            message_id: "not-a-message".to_owned()
        })
    );
    assert_eq!(
        compare_versions("pacs.008.001.08", "pacs.009.001.08"),
        Err(CompareError::UnrelatedFamilies {
            from: "pacs.008.001.08".to_owned(),
            to: "pacs.009.001.08".to_owned(),
        })
    );
}
