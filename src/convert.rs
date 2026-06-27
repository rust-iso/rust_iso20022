//! Typed conversions for ISO 20022 scalar values (`convert` feature).
//!
//! Generated scalar values (amounts, rates, dates, date-times) are stored as
//! lossless `String`s — exact text with no float rounding, which is the safe
//! representation for money. prowide exposes the same values as `BigDecimal` /
//! `LocalDate` / `OffsetDateTime`; these helpers provide the equivalent typed
//! access on demand, using `rust_decimal` and `chrono`.
//!
//! ```
//! # use rust_iso20022::convert::*;
//! assert_eq!(to_decimal("1234.56").unwrap().to_string(), "1234.56");
//! assert_eq!(to_date("2026-06-27").unwrap().to_string(), "2026-06-27");
//! assert!(to_datetime("2026-06-27T10:30:00Z").is_some());
//! ```

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;

/// Parse an ISO 20022 amount/rate (`xs:decimal`) into a [`Decimal`].
pub fn to_decimal(s: &str) -> Option<Decimal> {
    s.trim().parse().ok()
}

/// Parse an ISO 20022 `ISODate` (`YYYY-MM-DD`, optionally with a trailing
/// timezone) into a [`NaiveDate`].
pub fn to_date(s: &str) -> Option<NaiveDate> {
    let s = s.trim();
    // Tolerate a trailing timezone on date values (`2026-06-27+02:00`, `…Z`).
    let date_part = &s[..s.len().min(10)];
    NaiveDate::parse_from_str(date_part, "%Y-%m-%d").ok()
}

/// Parse an ISO 20022 `ISODateTime` into a timezone-aware [`DateTime`]. Falls
/// back to a naive parse (assuming UTC) when no offset is present.
pub fn to_datetime(s: &str) -> Option<DateTime<FixedOffset>> {
    let s = s.trim();
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt);
    }
    // No offset: parse naive and treat as UTC.
    for fmt in ["%Y-%m-%dT%H:%M:%S%.f", "%Y-%m-%dT%H:%M:%S"] {
        if let Ok(naive) = NaiveDateTime::parse_from_str(s, fmt) {
            return Some(DateTime::from_naive_utc_and_offset(
                naive,
                FixedOffset::east_opt(0).unwrap(),
            ));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions() {
        assert_eq!(to_decimal("  7866240.23491 ").unwrap().to_string(), "7866240.23491");
        assert!(to_decimal("not-a-number").is_none());
        assert_eq!(to_date("2026-06-27").unwrap().to_string(), "2026-06-27");
        assert_eq!(to_date("2026-06-27+02:00").unwrap().to_string(), "2026-06-27");
        assert!(to_datetime("2026-06-27T10:30:00Z").is_some());
        assert!(to_datetime("2026-06-27T10:30:00").is_some());
        assert!(to_datetime("2026-06-27T10:30:00.123+01:00").is_some());
    }
}
