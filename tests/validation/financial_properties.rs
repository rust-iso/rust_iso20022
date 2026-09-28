use proptest::prelude::*;
use rust_iso20022::helpers::{Iban, IsoDate, IsoDateTime, Lei, Money};

proptest! {
    #[test]
    fn arbitrary_utf8_never_panics_iban_lei_or_money(value in any::<String>()) {
        let _ = Iban::parse(&value);
        let _ = Lei::parse(&value);
        let _ = Money::parse("EUR", &value);
        let _ = IsoDate::parse(&value);
        let _ = IsoDateTime::parse(&value);
    }

    #[test]
    fn changing_a_valid_iban_check_digit_is_rejected(digit in b'0'..=b'9') {
        let mut value = b"DE89370400440532013000".to_vec();
        prop_assume!(digit != value[3]);
        value[3] = digit;
        let changed = String::from_utf8(value).unwrap();
        prop_assert!(Iban::parse(&changed).is_err());
    }

    #[test]
    fn extra_eur_fraction_digits_are_rejected(extra in 0u32..1_000_000) {
        let amount = format!("1.00{extra}");
        prop_assert!(Money::parse("EUR", &amount).is_err());
    }
}

#[test]
fn lei_checksum_single_character_mutations_fail() {
    let valid = b"5493001KJTIIGC8Y1R12";
    for index in 0..valid.len() {
        let mut changed = valid.to_vec();
        changed[index] = if changed[index] == b'A' { b'B' } else { b'A' };
        let changed = String::from_utf8(changed).unwrap();
        assert!(Lei::parse(&changed).is_err(), "mutation {index}");
    }
}
