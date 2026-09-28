//! Bounded SWIFT MT block and text-field parsing.

mod parser;

pub use parser::{
    MtBlock, MtDocument, MtField, MtLimit, MtParseError, MtParseErrorKind, MtParseLimits,
    SourceSpan,
};
