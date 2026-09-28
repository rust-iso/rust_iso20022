use rust_iso20022::helpers::{
    AccountIdentifier, Bic, ClearingIdentifier, CountryCode, Currency, Iban, IsoDate, IsoDateTime,
    Lei, Money,
};
use rust_iso20022::{MxId, ParseLimits, detect_message, validate_xml};

#[test]
fn xml_detection_and_namespace_seed_corpora_replay_without_panics() {
    for xml in [
        include_str!("../../fuzz/corpus/xml/valid.xml"),
        include_str!("../../fuzz/corpus/xml/dtd.xml"),
        include_str!("../../fuzz/corpus/xml/deep.xml"),
        include_str!("../../fuzz/corpus/detect/valid.xml"),
        include_str!("../../fuzz/corpus/detect/root-mismatch.xml"),
        include_str!("../../fuzz/corpus/detect/entity.xml"),
    ] {
        let outcome = std::panic::catch_unwind(|| {
            let _ = validate_xml(xml, ParseLimits::DEFAULT);
            let _ = detect_message(xml, ParseLimits::DEFAULT);
        });
        assert!(outcome.is_ok());
    }

    for namespace in [
        include_str!("../../fuzz/corpus/namespace/full.txt"),
        include_str!("../../fuzz/corpus/namespace/bare.txt"),
        include_str!("../../fuzz/corpus/namespace/malformed.txt"),
    ] {
        let value = namespace.trim();
        let outcome = std::panic::catch_unwind(|| MxId::parse(value));
        assert!(outcome.is_ok());
    }
}

#[test]
fn financial_seed_corpus_replays_validator_invariants() {
    for seed in [
        include_str!("../../fuzz/corpus/financial/iban.txt"),
        include_str!("../../fuzz/corpus/financial/bic.txt"),
        include_str!("../../fuzz/corpus/financial/lei.txt"),
    ] {
        let outcome = std::panic::catch_unwind(|| {
            let mut fields = seed.trim().split('|');
            let value = fields.next().unwrap_or_default();
            let auxiliary = fields.next().unwrap_or(value);
            if let Ok(parsed) = Iban::parse(value) {
                assert!(Iban::parse(parsed.as_str()).is_ok());
            }
            if let Ok(parsed) = Bic::parse(value) {
                assert!(Bic::parse(parsed.as_str()).is_ok());
            }
            if let Ok(parsed) = Lei::parse(value) {
                assert!(Lei::parse(parsed.as_str()).is_ok());
            }
            let _ = Currency::parse(value);
            let _ = CountryCode::parse(value);
            let _ = Money::parse(value, auxiliary);
            let _ = IsoDate::parse(value);
            let _ = IsoDateTime::parse(value);
            let _ = AccountIdentifier::parse(value);
            let _ = ClearingIdentifier::parse(value, auxiliary);
        });
        assert!(outcome.is_ok());
    }
}
