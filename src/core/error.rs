//! Error type for the hand-written core.

/// Errors raised when parsing identifiers or (de)serializing MX messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// A namespace or identifier could not be parsed into an [`crate::MxId`].
    InvalidMxId(String),
    /// The 4-letter business-area code is not a known ISO 20022 area.
    UnknownBusinessArea(String),
    /// XML deserialization failed (message from the underlying `yaserde`).
    Deserialize(String),
    /// XML serialization failed (message from the underlying `yaserde`).
    Serialize(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::InvalidMxId(s) => write!(f, "invalid ISO 20022 message id: {s}"),
            Error::UnknownBusinessArea(s) => write!(f, "unknown business area: {s}"),
            Error::Deserialize(s) => write!(f, "XML deserialization failed: {s}"),
            Error::Serialize(s) => write!(f, "XML serialization failed: {s}"),
        }
    }
}

impl std::error::Error for Error {}

/// Convenience alias.
pub type Result<T> = core::result::Result<T, Error>;
