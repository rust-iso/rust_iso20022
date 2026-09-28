use rust_iso20022::helpers::{
    AccountIdentifier, Bic, BicFi, ClearingIdentifier, CountryCode, Currency, Iban, IsoDate,
    IsoDateTime, Lei, Money, ValueConstraint,
};

#[test]
fn iban_valid_invalid_and_boundaries() {
    for value in ["DE89370400440532013000", "GB82WEST12345698765432"] {
        assert_eq!(Iban::parse(value).expect("valid IBAN").as_str(), value);
    }
    let checksum = Iban::parse("DE88370400440532013000").expect_err("checksum");
    assert_eq!(checksum.rule_id().as_str(), "ISO20022-L2-IBAN-CHECKSUM");
    assert_eq!(checksum.constraint(), ValueConstraint::Checksum);
    assert_eq!(
        Iban::parse("DE89 3704 0044 0532 0130 00")
            .expect_err("spaces are not canonical")
            .constraint(),
        ValueConstraint::CharacterSet
    );
    assert_eq!(
        Iban::parse("DE89")
            .expect_err("country length")
            .constraint(),
        ValueConstraint::Length
    );
    assert_eq!(
        Iban::parse("DE89A70400440532013000")
            .expect_err("German BBAN is numeric")
            .rule_id()
            .as_str(),
        "ISO20022-L2-IBAN-BBAN-FORMAT"
    );
    assert!(
        !Iban::parse("DE89370400440532013000")
            .unwrap()
            .account_existence_verified()
    );
}

#[test]
fn bic_and_bicfi_have_typed_syntax_failures() {
    for value in ["DEUTDEFF", "DEUTDEFF500"] {
        assert_eq!(Bic::parse(value).expect("valid BIC").as_str(), value);
        assert_eq!(BicFi::parse(value).expect("valid BICFI").as_str(), value);
    }
    for value in ["DEUTDE", "deutDEFF", "DEUTZZFF", "DEUTDE!F"] {
        assert!(Bic::parse(value).is_err(), "{value}");
    }
    assert!(
        !Bic::parse("DEUTDEFF")
            .unwrap()
            .directory_existence_verified()
    );
    assert!(
        !BicFi::parse("DEUTDEFF")
            .unwrap()
            .directory_existence_verified()
    );
}

#[test]
fn lei_checksum_and_boundaries() {
    assert_eq!(
        Lei::parse("5493001KJTIIGC8Y1R12")
            .expect("valid LEI")
            .as_str(),
        "5493001KJTIIGC8Y1R12"
    );
    assert_eq!(
        Lei::parse("5493001KJTIIGC8Y1R13")
            .expect_err("checksum")
            .constraint(),
        ValueConstraint::Checksum
    );
    assert_eq!(
        Lei::parse("5493001KJTIIGC8Y1R1")
            .expect_err("length")
            .constraint(),
        ValueConstraint::Length
    );
    assert!(
        !Lei::parse("5493001KJTIIGC8Y1R12")
            .unwrap()
            .directory_existence_verified()
    );
}

#[test]
fn currency_country_and_money_precision() {
    assert_eq!(Currency::parse("EUR").unwrap().minor_units(), Some(2));
    assert_eq!(Currency::parse("JPY").unwrap().minor_units(), Some(0));
    assert_eq!(Currency::parse("BHD").unwrap().minor_units(), Some(3));
    assert_eq!(Currency::parse("XAU").unwrap().minor_units(), None);
    assert_eq!(Currency::parse("XCG").unwrap().minor_units(), Some(2));
    assert_eq!(Currency::parse("ZWG").unwrap().minor_units(), Some(2));
    assert!(Currency::parse("ANG").is_err());
    assert!(Currency::parse("ZWL").is_err());
    assert!(Currency::parse("ZZZ").is_err());

    assert_eq!(CountryCode::parse("DE").unwrap().as_str(), "DE");
    assert_eq!(CountryCode::parse("US").unwrap().as_str(), "US");
    assert!(CountryCode::parse("ZZ").is_err());

    assert!(Money::parse("EUR", "0.01").is_ok());
    assert!(Money::parse("BHD", "1.234").is_ok());
    assert_eq!(
        Money::parse("JPY", "1.00")
            .expect_err("JPY precision")
            .constraint(),
        ValueConstraint::Precision
    );
    assert_eq!(
        Money::parse("EUR", "1e3")
            .expect_err("exponent notation")
            .constraint(),
        ValueConstraint::Format
    );
    assert!(Money::parse("EUR", "1.").is_err());
    assert!(Money::parse("EUR", "123456789012345678.00").is_err());
}

#[test]
fn helpers_convert_to_generated_leaf_storage_without_a_second_model() {
    let iban: String = Iban::parse("DE89370400440532013000").unwrap().into();
    let bic: String = BicFi::parse("DEUTDEFF").unwrap().into();
    let lei: String = Lei::parse("5493001KJTIIGC8Y1R12").unwrap().into();
    assert_eq!(iban, "DE89370400440532013000");
    assert_eq!(bic, "DEUTDEFF");
    assert_eq!(lei, "5493001KJTIIGC8Y1R12");
}

#[test]
fn date_and_datetime_calendar_boundaries() {
    assert!(IsoDate::parse("2024-02-29").is_ok());
    assert!(IsoDate::parse("2023-02-29").is_err());
    assert!(IsoDate::parse("2026-13-01").is_err());
    assert!(IsoDateTime::parse("2026-09-28T23:59:59Z").is_ok());
    assert!(IsoDateTime::parse("2026-09-28T23:59:59.123+08:00").is_ok());
    assert!(IsoDateTime::parse("2026-09-28 23:59:59").is_err());
    assert!(IsoDateTime::parse("2026-09-28T24:00:00Z").is_err());
}

#[test]
fn account_and_clearing_identifiers_enforce_bounds() {
    assert!(AccountIdentifier::parse("GB29NWBK60161331926819").is_ok());
    assert_eq!(
        AccountIdentifier::parse("")
            .expect_err("empty")
            .constraint(),
        ValueConstraint::Length
    );
    assert!(AccountIdentifier::parse(&"A".repeat(34)).is_ok());
    assert!(AccountIdentifier::parse(&"A".repeat(35)).is_err());
    assert!(AccountIdentifier::parse("ABC\n123").is_err());

    let clearing = ClearingIdentifier::parse("USABA", "021000021").expect("valid clearing id");
    assert_eq!(clearing.scheme(), "USABA");
    assert_eq!(clearing.member_id(), "021000021");
    assert!(ClearingIdentifier::parse("", "021000021").is_err());
    assert!(ClearingIdentifier::parse("USABA", &"1".repeat(36)).is_err());
}
