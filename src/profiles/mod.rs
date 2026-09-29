//! Exact, source-addressed profile releases over the shared validation engine.
//!
//! A profile name or year is never sufficient to dispatch L3 validation. The
//! registry accepts only a complete [`crate::profiles::ProfileReleaseKey`] and does not contain
//! message data or duplicate the generated ISO 20022 model.

/// Opt-in deterministic checks pending exact profile source artifacts.
pub mod provisional;
/// Usable public-reference bundles that remain separate from authoritative
/// release registration.
pub mod public_reference;
mod registry;
mod release;
/// SEPA rulebook/implementation-guideline source identities and release gates.
pub mod sepa;

pub use registry::{ProfileDispatchError, ProfileRegistry, ProfileRegistryError, ProfileRelease};
pub use release::{
    ProfileDate, ProfileDateParseError, ProfileReleaseKey, ProfileScheme, ProfileSchemeParseError,
    ReleaseKeyParseError,
};
