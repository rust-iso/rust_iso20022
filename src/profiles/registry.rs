use core::fmt;
use std::collections::BTreeSet;

use super::{ProfileDate, ProfileReleaseKey, ProfileScheme};
use crate::validation::{RuleSet, ValidationLayer};

/// One immutable profile release and its exact support/validation artifacts.
pub struct ProfileRelease<'a> {
    key: ProfileReleaseKey,
    supported_messages: Vec<String>,
    rule_set: Option<RuleSet<'a>>,
    unavailable_reason: Option<&'static str>,
    fixture_index_digest: &'static str,
    evidence_ref: &'static str,
}

impl fmt::Debug for ProfileRelease<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProfileRelease")
            .field("key", &self.key)
            .field("supported_messages", &self.supported_messages)
            .field("rules_available", &self.rule_set.is_some())
            .field("fixture_index_digest", &self.fixture_index_digest)
            .field("evidence_ref", &self.evidence_ref)
            .finish()
    }
}

impl<'a> ProfileRelease<'a> {
    pub fn available(
        key: ProfileReleaseKey,
        supported_messages: &[&str],
        rule_set: RuleSet<'a>,
        fixture_index_digest: &'static str,
        evidence_ref: &'static str,
    ) -> Result<Self, ProfileRegistryError> {
        if rule_set.layer != ValidationLayer::Profile
            || rule_set.name != key.scheme().as_str()
            || rule_set.version != Some(key.rule_set_version().as_str())
        {
            return Err(ProfileRegistryError::RuleSetIdentityMismatch);
        }
        Self::new(
            key,
            supported_messages,
            Some(rule_set),
            None,
            fixture_index_digest,
            evidence_ref,
        )
    }

    pub fn unavailable(
        key: ProfileReleaseKey,
        supported_messages: &[&str],
        reason: &'static str,
        fixture_index_digest: &'static str,
        evidence_ref: &'static str,
    ) -> Result<Self, ProfileRegistryError> {
        if reason.is_empty() {
            return Err(ProfileRegistryError::MissingArtifactReference);
        }
        Self::new(
            key,
            supported_messages,
            None,
            Some(reason),
            fixture_index_digest,
            evidence_ref,
        )
    }

    fn new(
        key: ProfileReleaseKey,
        supported_messages: &[&str],
        rule_set: Option<RuleSet<'a>>,
        unavailable_reason: Option<&'static str>,
        fixture_index_digest: &'static str,
        evidence_ref: &'static str,
    ) -> Result<Self, ProfileRegistryError> {
        if supported_messages.is_empty() {
            return Err(ProfileRegistryError::EmptySupportMatrix);
        }
        if fixture_index_digest.is_empty() || evidence_ref.is_empty() {
            return Err(ProfileRegistryError::MissingArtifactReference);
        }
        let mut unique = BTreeSet::new();
        for message in supported_messages {
            if crate::catalogue::lookup_descriptor(message).is_none() {
                return Err(ProfileRegistryError::UnknownMessage {
                    message_id: (*message).to_owned(),
                });
            }
            if !unique.insert(*message) {
                return Err(ProfileRegistryError::DuplicateMessage {
                    message_id: (*message).to_owned(),
                });
            }
        }
        Ok(Self {
            key,
            supported_messages: unique.into_iter().map(str::to_owned).collect(),
            rule_set,
            unavailable_reason,
            fixture_index_digest,
            evidence_ref,
        })
    }

    pub fn key(&self) -> &ProfileReleaseKey {
        &self.key
    }

    pub fn supported_messages(&self) -> &[String] {
        &self.supported_messages
    }

    pub fn fixture_index_digest(&self) -> &str {
        self.fixture_index_digest
    }

    pub fn evidence_ref(&self) -> &str {
        self.evidence_ref
    }
}

/// Immutable, sorted, append-only-by-construction release registry.
#[derive(Debug)]
pub struct ProfileRegistry<'a> {
    releases: Vec<ProfileRelease<'a>>,
}

impl<'a> ProfileRegistry<'a> {
    pub fn new(mut releases: Vec<ProfileRelease<'a>>) -> Result<Self, ProfileRegistryError> {
        releases.sort_by(|left, right| {
            left.key
                .scheme()
                .cmp(right.key.scheme())
                .then_with(|| left.key.effective_from().cmp(&right.key.effective_from()))
                .then_with(|| left.key.cmp(&right.key))
        });
        for pair in releases.windows(2) {
            let left = &pair[0];
            let right = &pair[1];
            if left.key == right.key {
                return Err(ProfileRegistryError::DuplicateRelease);
            }
            if left.key.scheme() == right.key.scheme()
                && right.key.effective_from() <= left.key.effective_until()
            {
                return Err(ProfileRegistryError::OverlappingEffectiveRange);
            }
        }
        Ok(Self { releases })
    }

    pub fn releases(&self) -> &[ProfileRelease<'a>] {
        &self.releases
    }

    pub fn exact(
        &self,
        key: &ProfileReleaseKey,
    ) -> Result<&ProfileRelease<'a>, ProfileDispatchError> {
        self.releases
            .iter()
            .find(|release| &release.key == key)
            .ok_or_else(|| ProfileDispatchError::UnknownRelease {
                key: key.to_string(),
            })
    }

    pub fn resolve_as_of(
        &self,
        scheme: &ProfileScheme,
        date: ProfileDate,
    ) -> Result<&ProfileRelease<'a>, ProfileDispatchError> {
        self.releases
            .iter()
            .rev()
            .find(|release| release.key.scheme() == scheme && release.key.effective_on(date))
            .ok_or_else(|| ProfileDispatchError::NoReleaseAsOf {
                scheme: scheme.clone(),
                date,
            })
    }

    pub fn supports(
        &self,
        key: &ProfileReleaseKey,
        message_id: &str,
    ) -> Result<(), ProfileDispatchError> {
        let release = self.exact(key)?;
        if release
            .supported_messages
            .binary_search_by(|candidate| candidate.as_str().cmp(message_id))
            .is_err()
        {
            return Err(ProfileDispatchError::UnsupportedMessage {
                key: key.to_string(),
                message_id: message_id.to_owned(),
            });
        }
        Ok(())
    }

    pub fn rule_set_for(
        &self,
        key: &ProfileReleaseKey,
        message_id: &str,
    ) -> Result<&RuleSet<'a>, ProfileDispatchError> {
        self.supports(key, message_id)?;
        let release = self.exact(key)?;
        release
            .rule_set
            .as_ref()
            .ok_or(ProfileDispatchError::RulesUnavailable {
                reason: release
                    .unavailable_reason
                    .unwrap_or("profile rules unavailable"),
            })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileRegistryError {
    EmptySupportMatrix,
    UnknownMessage { message_id: String },
    DuplicateMessage { message_id: String },
    DuplicateRelease,
    OverlappingEffectiveRange,
    RuleSetIdentityMismatch,
    MissingArtifactReference,
}

impl fmt::Display for ProfileRegistryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySupportMatrix => formatter.write_str("profile support matrix is empty"),
            Self::UnknownMessage { message_id } => {
                write!(formatter, "unknown supported ISO message: {message_id}")
            }
            Self::DuplicateMessage { message_id } => {
                write!(formatter, "duplicate supported ISO message: {message_id}")
            }
            Self::DuplicateRelease => formatter.write_str("duplicate profile release key"),
            Self::OverlappingEffectiveRange => {
                formatter.write_str("profile effective ranges overlap")
            }
            Self::RuleSetIdentityMismatch => {
                formatter.write_str("L3 rule-set identity does not match profile release")
            }
            Self::MissingArtifactReference => {
                formatter.write_str("profile artifact reference is missing")
            }
        }
    }
}

impl std::error::Error for ProfileRegistryError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileDispatchError {
    UnknownRelease {
        key: String,
    },
    NoReleaseAsOf {
        scheme: ProfileScheme,
        date: ProfileDate,
    },
    UnsupportedMessage {
        key: String,
        message_id: String,
    },
    RulesUnavailable {
        reason: &'static str,
    },
}

impl fmt::Display for ProfileDispatchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownRelease { key } => write!(formatter, "unknown profile release: {key}"),
            Self::NoReleaseAsOf { scheme, date } => {
                write!(formatter, "no {scheme} release effective on {date}")
            }
            Self::UnsupportedMessage { key, message_id } => {
                write!(formatter, "{message_id} is unsupported by {key}")
            }
            Self::RulesUnavailable { reason } => {
                write!(formatter, "profile rules unavailable: {reason}")
            }
        }
    }
}

impl std::error::Error for ProfileDispatchError {}
