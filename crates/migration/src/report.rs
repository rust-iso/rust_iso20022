//! Complete, value-free source-field mapping accountability.

use crate::mt::{MtDocument, SourceSpan};
use core::fmt;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingClassification {
    Exact,
    Derived,
    Lossy,
    Ambiguous,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SourceFieldRef {
    pub index: usize,
    pub tag: String,
    pub occurrence: usize,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MappingEntry {
    pub source: SourceFieldRef,
    pub target_path: Option<String>,
    pub classification: MappingClassification,
    pub note: String,
}

impl MappingEntry {
    pub fn new(
        source: SourceFieldRef,
        target_path: Option<String>,
        classification: MappingClassification,
        note: impl Into<String>,
    ) -> Self {
        Self {
            source,
            target_path,
            classification,
            note: note.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MappingWarning {
    pub code: String,
    pub message: String,
    pub source_index: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MappingReport {
    entries: Vec<MappingEntry>,
    warnings: Vec<MappingWarning>,
    unmapped_fields: Vec<SourceFieldRef>,
}

impl MappingReport {
    pub fn build(
        document: &MtDocument,
        mut entries: Vec<MappingEntry>,
        mut warnings: Vec<MappingWarning>,
    ) -> Result<Self, MappingReportError> {
        entries.sort_by_key(|entry| entry.source.index);
        warnings.sort_by(|a, b| (a.source_index, &a.code).cmp(&(b.source_index, &b.code)));
        let mut seen = vec![false; document.fields().len()];
        for entry in &entries {
            let field = document.fields().get(entry.source.index).ok_or(
                MappingReportError::UnknownSource {
                    index: entry.source.index,
                },
            )?;
            if seen[entry.source.index] {
                return Err(MappingReportError::DuplicateSource {
                    index: entry.source.index,
                });
            }
            if field.tag() != entry.source.tag
                || field.occurrence() != entry.source.occurrence
                || field.span() != entry.source.span
            {
                return Err(MappingReportError::SourceMismatch {
                    index: entry.source.index,
                });
            }
            seen[entry.source.index] = true;
        }
        if let Some(index) = seen.iter().position(|present| !present) {
            return Err(MappingReportError::MissingSource { index });
        }
        let unmapped_fields = entries
            .iter()
            .filter(|entry| {
                matches!(
                    entry.classification,
                    MappingClassification::Ambiguous | MappingClassification::Unsupported
                )
            })
            .map(|entry| entry.source.clone())
            .collect();
        Ok(Self {
            entries,
            warnings,
            unmapped_fields,
        })
    }
    pub fn entries(&self) -> &[MappingEntry] {
        &self.entries
    }
    pub fn warnings(&self) -> &[MappingWarning] {
        &self.warnings
    }
    pub fn unmapped_fields(&self) -> &[SourceFieldRef] {
        &self.unmapped_fields
    }
}

impl SourceFieldRef {
    pub fn from_document(document: &MtDocument, index: usize) -> Option<Self> {
        let field = document.fields().get(index)?;
        Some(Self {
            index,
            tag: field.tag().to_owned(),
            occurrence: field.occurrence(),
            span: field.span(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MappingReportError {
    MissingSource { index: usize },
    DuplicateSource { index: usize },
    UnknownSource { index: usize },
    SourceMismatch { index: usize },
}

impl fmt::Display for MappingReportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .write_str("mapping report does not account for every parsed source field exactly once")
    }
}
impl std::error::Error for MappingReportError {}
