//! Validated ergonomic values that convert to generated fields.
//!
//! Helpers own scalar values only. They do not duplicate message structures and
//! cannot become a second canonical ISO 20022 message model.

mod account;
mod currency;
mod identifiers;
mod money;
mod reference_data;
mod time;

pub use account::{AccountIdentifier, ClearingIdentifier};
pub use currency::{CountryCode, Currency};
pub use identifiers::{Bic, BicFi, Iban, Lei};
pub use money::Money;
pub use reference_data::REFERENCE_DATA_MANIFEST_SHA256;
pub use time::{IsoDate, IsoDateTime};

use crate::validation::RuleId;
use core::fmt;

/// Machine-readable category for a failed scalar constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueConstraint {
    Length,
    CharacterSet,
    Format,
    Checksum,
    UnsupportedCode,
    Precision,
    Range,
}

/// Typed, value-free helper validation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValueError {
    rule_id: RuleId,
    constraint: ValueConstraint,
}

impl ValueError {
    pub(crate) const fn new(rule_id: &'static str, constraint: ValueConstraint) -> Self {
        Self {
            rule_id: RuleId::new(rule_id),
            constraint,
        }
    }

    pub const fn rule_id(self) -> RuleId {
        self.rule_id
    }

    pub const fn constraint(self) -> ValueConstraint {
        self.constraint
    }
}

impl fmt::Display for ValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "value failed {} ({:?})",
            self.rule_id, self.constraint
        )
    }
}

impl std::error::Error for ValueError {}
