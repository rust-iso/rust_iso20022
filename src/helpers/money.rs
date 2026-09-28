use super::{Currency, ValueConstraint, ValueError};

/// Validated currency/decimal pair for ergonomic conversion to generated
/// amount fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Money {
    currency: Currency,
    amount: String,
}

impl Money {
    pub fn parse(currency: &str, amount: &str) -> Result<Self, ValueError> {
        let currency = Currency::parse(currency)?;
        let (whole, fraction) = match amount.split_once('.') {
            Some((_whole, "")) => return Err(format_error()),
            Some(parts) => parts,
            None => (amount, ""),
        };
        if whole.is_empty()
            || !whole.bytes().all(|byte| byte.is_ascii_digit())
            || !fraction.bytes().all(|byte| byte.is_ascii_digit())
            || amount.matches('.').count() > 1
        {
            return Err(format_error());
        }
        let significant_whole = whole.trim_start_matches('0').len();
        let total_digits = significant_whole.saturating_add(fraction.len()).max(1);
        if total_digits > 18 {
            return Err(ValueError::new(
                "ISO20022-L2-AMOUNT-RANGE",
                ValueConstraint::Range,
            ));
        }
        if let Some(minor_units) = currency.minor_units() {
            if fraction.len() > usize::from(minor_units) {
                return Err(ValueError::new(
                    "ISO20022-L2-AMOUNT-PRECISION",
                    ValueConstraint::Precision,
                ));
            }
        }
        Ok(Self {
            currency,
            amount: amount.to_owned(),
        })
    }

    pub const fn currency(&self) -> Currency {
        self.currency
    }

    pub fn amount(&self) -> &str {
        &self.amount
    }

    /// Split into values that can be placed directly into generated amount
    /// fields (`value` plus the generated currency-code newtype).
    pub fn into_parts(self) -> (Currency, String) {
        (self.currency, self.amount)
    }
}

const fn format_error() -> ValueError {
    ValueError::new("ISO20022-L2-AMOUNT-FORMAT", ValueConstraint::Format)
}
