use super::{ValueConstraint, ValueError, reference_data};

/// Reviewed ISO 4217 alpha code and minor-unit metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Currency {
    code: &'static str,
    minor_units: Option<u8>,
}

impl Currency {
    pub fn parse(value: &str) -> Result<Self, ValueError> {
        currency(value).ok_or_else(|| {
            ValueError::new(
                "ISO20022-L2-CURRENCY-CODE",
                ValueConstraint::UnsupportedCode,
            )
        })
    }

    pub const fn as_str(self) -> &'static str {
        self.code
    }

    pub const fn minor_units(self) -> Option<u8> {
        self.minor_units
    }
}

impl From<Currency> for String {
    fn from(value: Currency) -> Self {
        value.code.to_owned()
    }
}

/// ISO 3166-1 alpha-2 country code backed by `rust_iso3166`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CountryCode(&'static str);

impl CountryCode {
    pub fn parse(value: &str) -> Result<Self, ValueError> {
        reference_data::COUNTRY_CODES
            .binary_search(&value)
            .map(|index| Self(reference_data::COUNTRY_CODES[index]))
            .map_err(|_| {
                ValueError::new("ISO20022-L2-COUNTRY-CODE", ValueConstraint::UnsupportedCode)
            })
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl From<CountryCode> for String {
    fn from(value: CountryCode) -> Self {
        value.0.to_owned()
    }
}

fn currency(value: &str) -> Option<Currency> {
    let index = reference_data::CURRENCIES
        .binary_search_by_key(&value, |record| record.code)
        .ok()?;
    let record = reference_data::CURRENCIES[index];
    Some(Currency {
        code: record.code,
        minor_units: record.minor_units,
    })
}
