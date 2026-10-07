use rust_iso20022_migration::{mt::MtDocument, mt103, mt202, mt940};

fn assert_complete(
    document: &MtDocument,
    entries: &[rust_iso20022_migration::report::MappingEntry],
) {
    assert_eq!(entries.len(), document.fields().len());
    for (index, entry) in entries.iter().enumerate() {
        assert_eq!(entry.source.index, index);
    }
}

#[test]
fn mt_parser_seed_corpus_replays_without_panics() {
    for input in [
        include_str!("../../../fuzz/corpus/mt_parser/valid.mt"),
        include_str!("../../../fuzz/corpus/mt_parser/malformed.mt"),
    ] {
        assert!(std::panic::catch_unwind(|| MtDocument::parse(input)).is_ok());
    }
}

#[test]
fn unicode_balance_crash_seed_returns_a_typed_error() {
    let input = include_str!("../../../fuzz/corpus/mt940/unicode-balance.mt");
    assert!(matches!(
        mt940::convert(input),
        Err(mt940::Mt940Error::InvalidField { tag: "60F" })
    ));
}

#[test]
fn successful_conversion_seeds_account_for_every_source_field() {
    let mt103_input = include_str!("../../../fuzz/corpus/mt103/valid.mt");
    let mt103_document = MtDocument::parse(mt103_input).unwrap();
    let mt103_result = mt103::convert(mt103_input).unwrap();
    assert_complete(&mt103_document, mt103_result.mapping_report.entries());

    let mt202_input = include_str!("../../../fuzz/corpus/mt202/valid.mt");
    let mt202_document = MtDocument::parse(mt202_input).unwrap();
    let mt202_result = mt202::convert(mt202_input).unwrap();
    assert_complete(&mt202_document, mt202_result.mapping_report.entries());

    let mt940_input = include_str!("../../../fuzz/corpus/mt940/valid.mt");
    let mt940_document = MtDocument::parse(mt940_input).unwrap();
    let mt940_result = mt940::convert(mt940_input).unwrap();
    assert_complete(&mt940_document, mt940_result.mapping_report.entries());
}
