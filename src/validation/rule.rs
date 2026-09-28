use super::{RuleId, ValidationContext, ValidationIssue, ValidationLayer, ValidationTarget};
use std::fmt;

/// Immutable explanation metadata coupled to an executable rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RuleDescriptor {
    pub id: RuleId,
    pub layer: ValidationLayer,
    pub title: &'static str,
    pub reason: &'static str,
    pub source: Option<&'static str>,
    pub profile: Option<&'static str>,
    pub profile_version: Option<&'static str>,
    pub affected_messages: &'static [&'static str],
}

/// Additive explanation view over the descriptor owned by an executable rule.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RuleExplanation {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub descriptor: &'static RuleDescriptor,
    pub field_paths: Vec<&'static str>,
}

/// One reusable validation rule over a view of a canonical generated message.
pub trait Rule: Send + Sync {
    fn descriptor(&self) -> &'static RuleDescriptor;

    /// Logical generated-schema paths read or constrained by this rule.
    ///
    /// The default preserves source compatibility for external rule
    /// implementations while allowing adapters to expose richer traceability.
    fn field_paths(&self) -> Vec<&'static str> {
        Vec::new()
    }

    fn evaluate(
        &self,
        target: &ValidationTarget<'_>,
        context: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue>;
}

/// A versioned group of executable rules for one validation layer.
pub struct RuleSet<'a> {
    pub name: &'static str,
    pub version: Option<&'static str>,
    pub layer: ValidationLayer,
    pub(crate) rules: Vec<&'a dyn Rule>,
}

impl fmt::Debug for RuleSet<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RuleSet")
            .field("name", &self.name)
            .field("version", &self.version)
            .field("layer", &self.layer)
            .field("rule_count", &self.rules.len())
            .finish()
    }
}

/// A rule set cannot mix layers or silently omit a profile release.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleSetError {
    MissingProfileVersion,
    WrongLayer { rule_id: RuleId },
    WrongProfile { rule_id: RuleId },
    WrongProfileVersion { rule_id: RuleId },
}

impl fmt::Display for RuleSetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingProfileVersion => {
                formatter.write_str("profile rule set requires an explicit version")
            }
            Self::WrongLayer { rule_id } => {
                write!(formatter, "rule {rule_id} belongs to another layer")
            }
            Self::WrongProfile { rule_id } => {
                write!(formatter, "rule {rule_id} belongs to another profile")
            }
            Self::WrongProfileVersion { rule_id } => {
                write!(
                    formatter,
                    "rule {rule_id} belongs to another profile version"
                )
            }
        }
    }
}

impl std::error::Error for RuleSetError {}

impl<'a> RuleSet<'a> {
    pub fn new(
        name: &'static str,
        version: Option<&'static str>,
        layer: ValidationLayer,
        rules: &[&'a dyn Rule],
    ) -> Result<Self, RuleSetError> {
        if layer == ValidationLayer::Profile && version.is_none() {
            return Err(RuleSetError::MissingProfileVersion);
        }
        for rule in rules {
            let descriptor = rule.descriptor();
            if descriptor.layer != layer {
                return Err(RuleSetError::WrongLayer {
                    rule_id: descriptor.id,
                });
            }
            if layer == ValidationLayer::Profile && descriptor.profile != Some(name) {
                return Err(RuleSetError::WrongProfile {
                    rule_id: descriptor.id,
                });
            }
            if layer == ValidationLayer::Profile && descriptor.profile_version != version {
                return Err(RuleSetError::WrongProfileVersion {
                    rule_id: descriptor.id,
                });
            }
        }
        Ok(Self {
            name,
            version,
            layer,
            rules: rules.to_vec(),
        })
    }
}
