use super::{CountryCode, ValueConstraint, ValueError, reference_data};

/// Validated canonical IBAN. Registry membership controls the national length;
/// MOD-97 validates the checksum. It does not prove account existence.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Iban(String);

impl Iban {
    pub fn parse(value: &str) -> Result<Self, ValueError> {
        if !value
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
        {
            return Err(ValueError::new(
                "ISO20022-L2-IBAN-CHARSET",
                ValueConstraint::CharacterSet,
            ));
        }
        if value.len() < 15 || value.len() > 34 {
            return Err(ValueError::new(
                "ISO20022-L2-IBAN-LENGTH",
                ValueConstraint::Length,
            ));
        }
        let country = value
            .get(..2)
            .ok_or_else(|| ValueError::new("ISO20022-L2-IBAN-LENGTH", ValueConstraint::Length))?;
        if !country.bytes().all(|byte| byte.is_ascii_uppercase())
            || !value.as_bytes()[2..4].iter().all(u8::is_ascii_digit)
        {
            return Err(ValueError::new(
                "ISO20022-L2-IBAN-FORMAT",
                ValueConstraint::Format,
            ));
        }
        let format = iban_format(country).ok_or_else(|| {
            ValueError::new("ISO20022-L2-IBAN-COUNTRY", ValueConstraint::UnsupportedCode)
        })?;
        if value.len() != usize::from(format.total_length) {
            return Err(ValueError::new(
                "ISO20022-L2-IBAN-LENGTH",
                ValueConstraint::Length,
            ));
        }
        if !value.as_bytes()[4..]
            .iter()
            .zip(format.bban_pattern.bytes())
            .all(|(byte, class)| match class {
                b'n' => byte.is_ascii_digit(),
                b'a' => byte.is_ascii_uppercase(),
                b'c' => byte.is_ascii_uppercase() || byte.is_ascii_digit(),
                _ => false,
            })
        {
            return Err(ValueError::new(
                "ISO20022-L2-IBAN-BBAN-FORMAT",
                ValueConstraint::Format,
            ));
        }
        if mod97(
            value.as_bytes()[4..]
                .iter()
                .chain(value.as_bytes()[..4].iter())
                .copied(),
        ) != 1
        {
            return Err(ValueError::new(
                "ISO20022-L2-IBAN-CHECKSUM",
                ValueConstraint::Checksum,
            ));
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Syntax, national structure and checksum validation do not prove that an
    /// account exists or is reachable.
    pub const fn account_existence_verified(&self) -> bool {
        false
    }
}

impl From<Iban> for String {
    fn from(value: Iban) -> Self {
        value.0
    }
}

/// Syntactically valid ISO 9362 BIC. No proprietary directory lookup occurs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bic(String);

impl Bic {
    pub fn parse(value: &str) -> Result<Self, ValueError> {
        if value.len() != 8 && value.len() != 11 {
            return Err(ValueError::new(
                "ISO20022-L2-BIC-LENGTH",
                ValueConstraint::Length,
            ));
        }
        let bytes = value.as_bytes();
        let valid = bytes[..4].iter().all(u8::is_ascii_uppercase)
            && bytes[4..6].iter().all(u8::is_ascii_uppercase)
            && bytes[6..]
                .iter()
                .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit());
        if !valid {
            return Err(ValueError::new(
                "ISO20022-L2-BIC-FORMAT",
                ValueConstraint::CharacterSet,
            ));
        }
        CountryCode::parse(&value[4..6]).map_err(|_| {
            ValueError::new("ISO20022-L2-BIC-COUNTRY", ValueConstraint::UnsupportedCode)
        })?;
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub const fn directory_existence_verified(&self) -> bool {
        false
    }
}

/// Financial-institution BIC syntax wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BicFi(Bic);

impl BicFi {
    pub fn parse(value: &str) -> Result<Self, ValueError> {
        Bic::parse(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub const fn directory_existence_verified(&self) -> bool {
        false
    }
}

impl From<Bic> for String {
    fn from(value: Bic) -> Self {
        value.0
    }
}

impl From<BicFi> for String {
    fn from(value: BicFi) -> Self {
        value.0.0
    }
}

/// ISO 17442 Legal Entity Identifier with MOD-97 checksum validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lei(String);

impl Lei {
    pub fn parse(value: &str) -> Result<Self, ValueError> {
        if value.len() != 20 {
            return Err(ValueError::new(
                "ISO20022-L2-LEI-LENGTH",
                ValueConstraint::Length,
            ));
        }
        if !value
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
        {
            return Err(ValueError::new(
                "ISO20022-L2-LEI-CHARSET",
                ValueConstraint::CharacterSet,
            ));
        }
        if mod97(value.bytes()) != 1 {
            return Err(ValueError::new(
                "ISO20022-L2-LEI-CHECKSUM",
                ValueConstraint::Checksum,
            ));
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// A valid checksum does not assert that the LEI is issued, active, or in
    /// good standing in the GLEIF directory.
    pub const fn directory_existence_verified(&self) -> bool {
        false
    }
}

impl From<Lei> for String {
    fn from(value: Lei) -> Self {
        value.0
    }
}

fn mod97(bytes: impl IntoIterator<Item = u8>) -> u32 {
    let mut remainder = 0u32;
    for byte in bytes {
        if byte.is_ascii_digit() {
            remainder = (remainder * 10 + u32::from(byte - b'0')) % 97;
        } else if byte.is_ascii_uppercase() {
            let value = u32::from(byte - b'A') + 10;
            remainder = (remainder * 100 + value) % 97;
        } else {
            return u32::MAX;
        }
    }
    remainder
}

fn iban_format(country: &str) -> Option<reference_data::IbanFormat> {
    let index = reference_data::IBAN_FORMATS
        .binary_search_by_key(&country, |format| format.country)
        .ok()?;
    Some(reference_data::IBAN_FORMATS[index])
}
