#![no_main]

use libfuzzer_sys::fuzz_target;
use rust_iso20022::helpers::{
    AccountIdentifier, Bic, ClearingIdentifier, CountryCode, Currency, Iban, IsoDate, IsoDateTime,
    Lei, Money,
};

fuzz_target!(|data: &[u8]| {
    let Ok(value) = std::str::from_utf8(data) else {
        return;
    };
    let mut fields = value.split('|');
    let identifier = fields.next().unwrap_or(value);
    let auxiliary = fields.next().unwrap_or(identifier);

    if let Ok(parsed) = Iban::parse(identifier) {
        assert!(Iban::parse(parsed.as_str()).is_ok());
    }
    if let Ok(parsed) = Bic::parse(identifier) {
        assert!(Bic::parse(parsed.as_str()).is_ok());
    }
    if let Ok(parsed) = Lei::parse(identifier) {
        assert!(Lei::parse(parsed.as_str()).is_ok());
    }
    let _ = Currency::parse(identifier);
    let _ = CountryCode::parse(identifier);
    let _ = Money::parse(identifier, auxiliary);
    let _ = IsoDate::parse(identifier);
    let _ = IsoDateTime::parse(identifier);
    let _ = AccountIdentifier::parse(identifier);
    let _ = ClearingIdentifier::parse(identifier, auxiliary);
});
