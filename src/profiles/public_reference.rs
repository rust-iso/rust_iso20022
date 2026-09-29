//! Public-reference profile bundles.
//!
//! These bundles make independently implemented, publicly documented checks
//! usable without pretending that a restricted scheme rulebook has been
//! reproduced. They are intentionally separate from the authoritative release
//! registry: callers must opt in and should surface the `provisional` status
//! to users and downstream systems.

use super::provisional::{
    cbpr_plus_provisional_rules, sepa_sct_inst_provisional_rules, sepa_sct_provisional_rules,
};
use crate::validation::{RuleRegistry, RuleSet};

/// Provenance classification for a profile bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceStatus {
    /// Independently implemented from public standards and metadata; not a
    /// claim of network or bank acceptance.
    PublicReference,
    /// Reserved for future bundles backed by exact, distributable source
    /// artifacts. Authoritative bundles are still kept in the release registry.
    Authoritative,
}

/// A usable profile rule bundle with explicit provenance and release labels.
///
/// This type contains validation rules and metadata only. It does not define
/// a second ISO 20022 message model; callers continue to validate generated
/// structs through the shared [`crate::validation::ValidationTarget`].
pub struct PublicReferenceProfile<'a> {
    scheme: &'static str,
    release: &'static str,
    guideline: &'static str,
    status: ReferenceStatus,
    source_references: &'static [&'static str],
    rules: RuleSet<'a>,
}

impl<'a> PublicReferenceProfile<'a> {
    pub(crate) fn new(
        scheme: &'static str,
        release: &'static str,
        guideline: &'static str,
        source_references: &'static [&'static str],
        rules: RuleSet<'a>,
    ) -> Self {
        Self {
            scheme,
            release,
            guideline,
            status: ReferenceStatus::PublicReference,
            source_references,
            rules,
        }
    }

    pub const fn scheme(&self) -> &'static str {
        self.scheme
    }

    pub const fn release(&self) -> &'static str {
        self.release
    }

    pub const fn guideline(&self) -> &'static str {
        self.guideline
    }

    pub const fn status(&self) -> ReferenceStatus {
        self.status
    }

    pub const fn source_references(&self) -> &'static [&'static str] {
        self.source_references
    }

    /// Consume the bundle and construct the normal explainable rule registry.
    pub fn into_registry(self) -> RuleRegistry<'a> {
        RuleRegistry::from_rule_sets(&[self.rules])
            .expect("public-reference rule identifiers are unique")
    }

    /// Borrow the underlying rule set for adapters that already manage a
    /// registry lifecycle.
    pub fn rules(&self) -> &RuleSet<'a> {
        &self.rules
    }
}

static CBPR_SOURCES: &[&str] = &[
    "https://www.swift.com/standards/standards-releases",
    "docs/profile-references.md#cbpr-plus",
];

static SCT_SOURCES: &[&str] = &[
    "https://www.europeanpaymentscouncil.eu/what-we-do/epc-payment-schemes/sepa-credit-transfer-sct/sepa-credit-transfer-rulebook-and",
    "docs/profile-references.md#sepa",
];

static SCT_INST_SOURCES: &[&str] = &[
    "https://www.europeanpaymentscouncil.eu/what-we-do/epc-payment-schemes/sepa-instant-credit-transfer/sepa-instant-credit-transfer-rulebook",
    "docs/profile-references.md#sepa",
];

/// Public-reference CBPR+ candidate checks labelled `provisional`.
pub fn cbpr_plus_public_reference() -> PublicReferenceProfile<'static> {
    PublicReferenceProfile::new(
        "cbpr-plus",
        "SR2026",
        "public-reference",
        CBPR_SOURCES,
        cbpr_plus_provisional_rules(),
    )
}

/// Public-reference SCT candidate checks labelled `provisional`.
pub fn sepa_sct_public_reference() -> PublicReferenceProfile<'static> {
    PublicReferenceProfile::new(
        "sepa-sct",
        "2025-v1.1",
        "public-reference",
        SCT_SOURCES,
        sepa_sct_provisional_rules(),
    )
}

/// Public-reference SCT Inst candidate checks labelled `provisional`.
pub fn sepa_sct_inst_public_reference() -> PublicReferenceProfile<'static> {
    PublicReferenceProfile::new(
        "sepa-sct-inst",
        "2025-v1.1",
        "public-reference",
        SCT_INST_SOURCES,
        sepa_sct_inst_provisional_rules(),
    )
}
