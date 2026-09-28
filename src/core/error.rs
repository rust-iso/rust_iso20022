//! Error type for the hand-written core.

/// Errors raised when parsing identifiers or (de)serializing MX messages.
#[derive(Clone, PartialEq, Eq)]
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

impl core::fmt::Debug for Error {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(match self {
            Self::InvalidMxId(_) => "Error::InvalidMxId([REDACTED])",
            Self::UnknownBusinessArea(_) => "Error::UnknownBusinessArea([REDACTED])",
            Self::Deserialize(_) => "Error::Deserialize([REDACTED])",
            Self::Serialize(_) => "Error::Serialize([REDACTED])",
        })
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::InvalidMxId(_) => f.write_str("invalid ISO 20022 message id"),
            Error::UnknownBusinessArea(_) => f.write_str("unknown business area"),
            Error::Deserialize(_) => f.write_str("XML deserialization failed"),
            Error::Serialize(_) => f.write_str("XML serialization failed"),
        }
    }
}

impl std::error::Error for Error {}

/// Convenience alias.
pub type Result<T> = core::result::Result<T, Error>;
