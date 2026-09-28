//! SEPA source identity and release eligibility.
//!
//! This module deliberately contains no payment-message DTO. It records the
//! rulebook and implementation-guideline documents needed to select an exact
//! SCT or SCT Inst release. A release with missing source-content digests is
//! discoverable but cannot be advertised as production-ready.

use core::fmt;
use std::collections::BTreeSet;

use super::ProfileDate;

/// EPC credit-transfer schemes supported by the shared source framework.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SepaScheme {
    Sct,
    SctInst,
}

impl SepaScheme {
    pub const fn profile_name(self) -> &'static str {
        match self {
            Self::Sct => "sepa-sct",
            Self::SctInst => "sepa-sct-inst",
        }
    }
}

/// The distinct source roles that make up an EPC release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DocumentKind {
    Rulebook,
    CustomerToPspGuideline,
    InterPspGuideline,
}

/// Publication state; consultation material is never an effective release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DocumentStatus {
    Consultation,
    Published,
    Effective,
    Retired,
}

/// One exact public EPC source document without copied source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDocument {
    scheme: SepaScheme,
    document_id: String,
    edition: String,
    kind: DocumentKind,
    status: DocumentStatus,
    issued: ProfileDate,
    effective_from: Option<ProfileDate>,
    source_url: String,
    content_sha256: Option<String>,
}

impl SourceDocument {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        scheme: SepaScheme,
        document_id: &str,
        edition: &str,
        kind: DocumentKind,
        status: DocumentStatus,
        issued: &str,
        effective_from: Option<&str>,
        source_url: &str,
        content_sha256: Option<&str>,
    ) -> Result<Self, SepaReleaseError> {
        if document_id.is_empty() || edition.is_empty() || source_url.is_empty() {
            return Err(SepaReleaseError::InvalidSourceIdentity);
        }
        let issued = ProfileDate::parse(issued).map_err(|_| SepaReleaseError::InvalidDate)?;
        let effective_from = effective_from
            .map(ProfileDate::parse)
            .transpose()
            .map_err(|_| SepaReleaseError::InvalidDate)?;
        if content_sha256.is_some_and(|digest| {
            digest.len() != 64
                || !digest
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        }) {
            return Err(SepaReleaseError::InvalidDigest);
        }
        Ok(Self {
            scheme,
            document_id: document_id.to_owned(),
            edition: edition.to_owned(),
            kind,
            status,
            issued,
            effective_from,
            source_url: source_url.to_owned(),
            content_sha256: content_sha256.map(str::to_owned),
        })
    }

    pub const fn scheme(&self) -> SepaScheme {
        self.scheme
    }

    pub fn document_id(&self) -> &str {
        &self.document_id
    }

    pub fn edition(&self) -> &str {
        &self.edition
    }

    pub const fn kind(&self) -> DocumentKind {
        self.kind
    }

    pub const fn status(&self) -> DocumentStatus {
        self.status
    }

    pub const fn issued(&self) -> ProfileDate {
        self.issued
    }

    pub const fn effective_from(&self) -> Option<ProfileDate> {
        self.effective_from
    }

    pub fn source_url(&self) -> &str {
        &self.source_url
    }

    pub fn content_sha256(&self) -> Option<&str> {
        self.content_sha256.as_deref()
    }
}

/// Exact rulebook plus its separately versioned implementation guidelines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SepaRelease {
    scheme: SepaScheme,
    rulebook: SourceDocument,
    guidelines: Vec<SourceDocument>,
}

impl SepaRelease {
    pub fn new(
        scheme: SepaScheme,
        rulebook: SourceDocument,
        mut guidelines: Vec<SourceDocument>,
    ) -> Result<Self, SepaReleaseError> {
        if rulebook.scheme != scheme || guidelines.iter().any(|source| source.scheme != scheme) {
            return Err(SepaReleaseError::SchemeMismatch);
        }
        if rulebook.kind != DocumentKind::Rulebook {
            return Err(SepaReleaseError::RulebookKindRequired);
        }
        if rulebook.status != DocumentStatus::Effective || rulebook.effective_from.is_none() {
            return Err(SepaReleaseError::RulebookNotEffective);
        }
        if guidelines.is_empty() {
            return Err(SepaReleaseError::MissingGuidelines);
        }
        if guidelines
            .iter()
            .any(|source| source.kind == DocumentKind::Rulebook)
        {
            return Err(SepaReleaseError::GuidelineKindRequired);
        }
        if guidelines.iter().any(|source| {
            source.status != DocumentStatus::Effective || source.effective_from.is_none()
        }) {
            return Err(SepaReleaseError::GuidelineNotEffective);
        }
        let mut identities = BTreeSet::new();
        if guidelines
            .iter()
            .any(|source| !identities.insert(source.document_id.as_str()))
        {
            return Err(SepaReleaseError::DuplicateDocument);
        }
        guidelines.sort_by(|left, right| left.document_id.cmp(&right.document_id));
        Ok(Self {
            scheme,
            rulebook,
            guidelines,
        })
    }

    pub const fn scheme(&self) -> SepaScheme {
        self.scheme
    }

    pub const fn rulebook(&self) -> &SourceDocument {
        &self.rulebook
    }

    pub fn guidelines(&self) -> &[SourceDocument] {
        &self.guidelines
    }

    pub fn is_effective(&self) -> bool {
        self.rulebook.status == DocumentStatus::Effective
            && self
                .guidelines
                .iter()
                .all(|source| source.status == DocumentStatus::Effective)
    }

    /// True only when exact effective documents also have verified bytes.
    pub fn is_production_ready(&self) -> bool {
        self.is_effective() && self.missing_digests().is_empty()
    }

    pub fn missing_digests(&self) -> Vec<&str> {
        core::iter::once(&self.rulebook)
            .chain(self.guidelines.iter())
            .filter(|source| source.content_sha256.is_none())
            .map(|source| source.document_id.as_str())
            .collect()
    }
}

/// Typed reasons a source set cannot identify an effective SEPA release.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SepaReleaseError {
    InvalidSourceIdentity,
    InvalidDate,
    InvalidDigest,
    SchemeMismatch,
    RulebookKindRequired,
    RulebookNotEffective,
    MissingGuidelines,
    GuidelineKindRequired,
    GuidelineNotEffective,
    DuplicateDocument,
}

impl fmt::Display for SepaReleaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidSourceIdentity => "SEPA source identity is incomplete",
            Self::InvalidDate => "SEPA source date is invalid",
            Self::InvalidDigest => "SEPA source digest is not lowercase SHA-256",
            Self::SchemeMismatch => "SEPA source belongs to a different scheme",
            Self::RulebookKindRequired => "SEPA release requires a rulebook source",
            Self::RulebookNotEffective => "SEPA rulebook is not an effective release",
            Self::MissingGuidelines => "SEPA release has no implementation guideline",
            Self::GuidelineKindRequired => "SEPA guideline source has the wrong role",
            Self::GuidelineNotEffective => "SEPA guideline is not effective",
            Self::DuplicateDocument => "SEPA source document is duplicated",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for SepaReleaseError {}

/// Publicly identified current SCT release. Source bytes are intentionally not
/// marked verified until their exact SHA-256 values can be recorded.
pub fn public_sct_2025() -> Result<SepaRelease, SepaReleaseError> {
    SepaRelease::new(
        SepaScheme::Sct,
        source(
            SepaScheme::Sct,
            "EPC125-05",
            "2025-v1.1",
            DocumentKind::Rulebook,
            "2025-10-05",
            "https://www.europeanpaymentscouncil.eu/document-library/rulebooks/2025-sepa-credit-transfer-rulebook-version-11",
        )?,
        vec![
            source(
                SepaScheme::Sct,
                "EPC132-08",
                "2025-v1.0",
                DocumentKind::CustomerToPspGuideline,
                "2025-10-06",
                "https://www.europeanpaymentscouncil.eu/document-library/implementation-guidelines/sepa-credit-transfer-customer-psp-implementation-1",
            )?,
            source(
                SepaScheme::Sct,
                "EPC115-06",
                "2025-v1.0",
                DocumentKind::InterPspGuideline,
                "2025-10-06",
                "https://www.europeanpaymentscouncil.eu/document-library/implementation-guidelines/sepa-credit-transfer-inter-psp-implementation-1",
            )?,
        ],
    )
}

/// Publicly identified current SCT Inst release. Source bytes are intentionally
/// not marked verified until their exact SHA-256 values can be recorded.
pub fn public_sct_inst_2025() -> Result<SepaRelease, SepaReleaseError> {
    SepaRelease::new(
        SepaScheme::SctInst,
        source(
            SepaScheme::SctInst,
            "EPC004-16",
            "2025-v1.1",
            DocumentKind::Rulebook,
            "2025-10-06",
            "https://www.europeanpaymentscouncil.eu/document-library/rulebooks/2025-sepa-instant-credit-transfer-rulebook-version-11",
        )?,
        vec![
            source(
                SepaScheme::SctInst,
                "EPC121-16",
                "2025-v1.0",
                DocumentKind::CustomerToPspGuideline,
                "2025-10-06",
                "https://www.europeanpaymentscouncil.eu/document-library/implementation-guidelines/sepa-instant-credit-transfer-customer-psp-2",
            )?,
            source(
                SepaScheme::SctInst,
                "EPC122-16",
                "2025-v1.0",
                DocumentKind::InterPspGuideline,
                "2025-10-06",
                "https://www.europeanpaymentscouncil.eu/document-library/implementation-guidelines/sepa-instant-credit-transfer-inter-psp-implementation-2",
            )?,
        ],
    )
}

fn source(
    scheme: SepaScheme,
    document_id: &str,
    edition: &str,
    kind: DocumentKind,
    issued: &str,
    source_url: &str,
) -> Result<SourceDocument, SepaReleaseError> {
    SourceDocument::new(
        scheme,
        document_id,
        edition,
        kind,
        DocumentStatus::Effective,
        issued,
        Some("2025-10-05"),
        source_url,
        None,
    )
}
