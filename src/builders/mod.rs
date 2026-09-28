//! Additive builders that return canonical generated message structs.
//!
//! Builders store only the small set of typed inputs needed during
//! construction. They are not message models and are consumed by `build`.

#[cfg(feature = "model-camt")]
mod camt;
mod error;
#[cfg(feature = "model-pacs")]
mod pacs;
#[cfg(feature = "model-pain")]
mod pain;

#[cfg(feature = "model-camt")]
pub use camt::{Camt052Builder, Camt053Builder, Camt054Builder, CamtReportId};
pub use error::{BuilderError, BuilderErrorKind};
#[cfg(feature = "model-pacs")]
pub use pacs::{Pacs002Builder, Pacs008Builder, Pacs009Builder};
#[cfg(feature = "model-pain")]
pub use pain::{Pain001Builder, Pain002Builder};

#[cfg(any(feature = "model-camt", feature = "model-pacs", feature = "model-pain"))]
pub(crate) fn required<T>(value: Option<T>, field: &'static str) -> Result<T, BuilderError> {
    value.ok_or_else(|| BuilderError::missing(field))
}

#[cfg(any(feature = "model-camt", feature = "model-pacs", feature = "model-pain"))]
pub(crate) fn bounded_text(value: &str, field: &'static str) -> Result<(), BuilderError> {
    if value.is_empty() || value.len() > 35 {
        return Err(BuilderError::new(
            "BUILDER-FIELD-LENGTH",
            field,
            BuilderErrorKind::Constraint,
        ));
    }
    if value.chars().any(char::is_control) {
        return Err(BuilderError::new(
            "BUILDER-FIELD-CHARSET",
            field,
            BuilderErrorKind::Constraint,
        ));
    }
    Ok(())
}
