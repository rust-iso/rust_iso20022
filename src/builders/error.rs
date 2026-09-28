use core::fmt;

/// Stable category for a builder failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuilderErrorKind {
    Missing,
    Constraint,
    Validation,
    Internal,
}

/// Typed, value-free error returned by high-level builders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuilderError {
    code: &'static str,
    field: &'static str,
    kind: BuilderErrorKind,
}

impl BuilderError {
    #[cfg(any(feature = "model-camt", feature = "model-pacs", feature = "model-pain"))]
    pub(crate) const fn new(
        code: &'static str,
        field: &'static str,
        kind: BuilderErrorKind,
    ) -> Self {
        Self { code, field, kind }
    }

    #[cfg(any(feature = "model-camt", feature = "model-pacs", feature = "model-pain"))]
    pub(crate) const fn missing(field: &'static str) -> Self {
        Self::new("BUILDER-MISSING-FIELD", field, BuilderErrorKind::Missing)
    }

    pub const fn code(&self) -> &'static str {
        self.code
    }

    pub const fn field(&self) -> &'static str {
        self.field
    }

    pub const fn kind(&self) -> BuilderErrorKind {
        self.kind
    }
}

impl fmt::Display for BuilderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} at {}", self.code, self.field)
    }
}

impl std::error::Error for BuilderError {}
