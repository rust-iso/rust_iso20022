//! Bounded transient SWIFT MT syntax and complete mapping-accountability types.
//!
//! [`mt::MtDocument`] is migration input, not a second ISO 20022 model.

pub mod mt;
pub mod report;

#[cfg(feature = "mt103")]
pub mod mt103;
#[cfg(feature = "mt202")]
pub mod mt202;
#[cfg(feature = "mt940")]
pub mod mt940;
