//! Thin mappings from core SDK results to CLI output DTOs.

mod catalogue;
mod compare;
mod explain;
mod inspect;
mod serialize;
mod validate;

#[allow(unused_imports)]
pub use catalogue::{CatalogueData, VersionsData, catalog, versions};
#[allow(unused_imports)]
pub use compare::compare;
#[allow(unused_imports)]
pub use explain::{explain, explain_from_rules};
#[allow(unused_imports)]
pub use inspect::{InspectData, inspect};
#[allow(unused_imports)]
pub use serialize::{JsonData, XmlData, to_json, to_xml};
#[allow(unused_imports)]
pub use validate::{ValidateData, validate};

use crate::output::ExitCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandError {
    pub code: &'static str,
    pub message: &'static str,
    pub exit: ExitCode,
}

impl CommandError {
    pub const fn unavailable() -> Self {
        Self {
            code: "feature_unavailable",
            message: "requested serialization feature or message model is unavailable",
            exit: ExitCode::Unavailable,
        }
    }

    pub const fn parse(code: &'static str, message: &'static str) -> Self {
        Self {
            code,
            message,
            exit: ExitCode::ParseFailure,
        }
    }

    pub const fn rule_not_found() -> Self {
        Self {
            code: "rule_not_found",
            message: "validation rule is not registered",
            exit: ExitCode::Unavailable,
        }
    }

    pub const fn internal() -> Self {
        Self {
            code: "internal_failure",
            message: "internal validation registry failure",
            exit: ExitCode::InternalFailure,
        }
    }

    pub const fn unknown_message() -> Self {
        Self {
            code: "unknown_message",
            message: "unknown ISO 20022 message",
            exit: ExitCode::Unavailable,
        }
    }

    pub const fn unrelated_families() -> Self {
        Self {
            code: "unrelated_families",
            message: "message versions belong to different families",
            exit: ExitCode::Usage,
        }
    }
}
