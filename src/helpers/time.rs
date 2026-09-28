use super::{ValueConstraint, ValueError};

/// Calendar-valid ISO date (`YYYY-MM-DD`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IsoDate(String);

impl IsoDate {
    pub fn parse(value: &str) -> Result<Self, ValueError> {
        validate_date_bytes(value.as_bytes())?;
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<IsoDate> for String {
    fn from(value: IsoDate) -> Self {
        value.0
    }
}

/// ISO date-time with an explicit UTC designator or numeric offset.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IsoDateTime(String);

impl IsoDateTime {
    pub fn parse(value: &str) -> Result<Self, ValueError> {
        let bytes = value.as_bytes();
        if bytes.len() < 20 || bytes.get(10) != Some(&b'T') {
            return Err(format_error());
        }
        validate_date_bytes(&bytes[..10])?;
        let hour = number(&bytes[11..13])?;
        let minute = number(&bytes[14..16])?;
        let second = number(&bytes[17..19])?;
        if bytes.get(13) != Some(&b':')
            || bytes.get(16) != Some(&b':')
            || hour > 23
            || minute > 59
            || second > 59
        {
            return Err(format_error());
        }

        let suffix = &bytes[19..];
        let zone = if suffix == b"Z" {
            &b"Z"[..]
        } else {
            let zone_start = suffix
                .iter()
                .position(|byte| matches!(byte, b'+' | b'-'))
                .ok_or_else(format_error)?;
            let fraction = &suffix[..zone_start];
            if !fraction.is_empty()
                && (!fraction.starts_with(b".")
                    || fraction.len() == 1
                    || !fraction[1..].iter().all(u8::is_ascii_digit))
            {
                return Err(format_error());
            }
            &suffix[zone_start..]
        };
        if zone != b"Z" {
            if zone.len() != 6 || zone.get(3) != Some(&b':') {
                return Err(format_error());
            }
            let zone_hour = number(&zone[1..3])?;
            let zone_minute = number(&zone[4..6])?;
            if zone_hour > 14 || zone_minute > 59 || (zone_hour == 14 && zone_minute != 0) {
                return Err(format_error());
            }
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<IsoDateTime> for String {
    fn from(value: IsoDateTime) -> Self {
        value.0
    }
}

fn validate_date_bytes(value: &[u8]) -> Result<(), ValueError> {
    if value.len() != 10 || value.get(4) != Some(&b'-') || value.get(7) != Some(&b'-') {
        return Err(format_error());
    }
    let year = number(&value[..4])?;
    let month = number(&value[5..7])?;
    let day = number(&value[8..10])?;
    let maximum = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => return Err(range_error()),
    };
    if day == 0 || day > maximum {
        return Err(range_error());
    }
    Ok(())
}

fn number(value: &[u8]) -> Result<u32, ValueError> {
    if !value.iter().all(u8::is_ascii_digit) {
        return Err(format_error());
    }
    Ok(value
        .iter()
        .fold(0, |number, digit| number * 10 + u32::from(*digit - b'0')))
}

const fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

const fn format_error() -> ValueError {
    ValueError::new("ISO20022-L2-DATE-TIME-FORMAT", ValueConstraint::Format)
}

const fn range_error() -> ValueError {
    ValueError::new("ISO20022-L2-DATE-TIME-RANGE", ValueConstraint::Range)
}
