use std::cmp::Ordering;
use std::fmt;

/// A stable validation layer in execution order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ValidationLayer {
    /// Well-formed XML and, when available, XSD validation.
    SyntaxSchema,
    /// ISO 20022 semantic rules independent of a market profile.
    IsoSemantic,
    /// Versioned market or network profile rules.
    Profile,
}

/// Issue severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Severity {
    Error,
    Warning,
}

/// Stable identifier for an executable validation rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RuleId(&'static str);

impl RuleId {
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for RuleId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for RuleId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0)
    }
}

/// Logical schema path. It identifies a field but never stores its value.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FieldPath(String);

impl FieldPath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for FieldPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

/// One structured validation finding.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ValidationIssue {
    pub code: RuleId,
    pub severity: Severity,
    pub path: FieldPath,
    pub message: String,
    pub rule_id: RuleId,
    pub layer: ValidationLayer,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub profile: Option<&'static str>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub profile_version: Option<&'static str>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub source: Option<&'static str>,
}

impl ValidationIssue {
    pub fn new(
        descriptor: &super::RuleDescriptor,
        severity: Severity,
        path: FieldPath,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: descriptor.id,
            severity,
            path,
            message: message.into(),
            rule_id: descriptor.id,
            layer: descriptor.layer,
            profile: descriptor.profile,
            profile_version: descriptor.profile_version,
            source: descriptor.source,
        }
    }

    pub fn standalone(
        rule_id: &'static str,
        layer: ValidationLayer,
        severity: Severity,
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        let rule_id = RuleId::new(rule_id);
        Self {
            code: rule_id,
            severity,
            path: FieldPath::new(path),
            message: message.into(),
            rule_id,
            layer,
            profile: None,
            profile_version: None,
            source: None,
        }
    }
}

/// Explicit reason that a requested validation layer could not run.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LayerUnavailable {
    pub layer: ValidationLayer,
    pub reason: String,
}

/// Aggregated validation output. Validity and partitions are derived, never
/// independently mutable.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ValidationReport {
    issues: Vec<ValidationIssue>,
    unavailable_layers: Vec<LayerUnavailable>,
}

impl ValidationReport {
    pub const fn new() -> Self {
        Self {
            issues: Vec::new(),
            unavailable_layers: Vec::new(),
        }
    }

    pub fn valid(&self) -> bool {
        self.complete() && self.errors().is_empty()
    }

    pub fn complete(&self) -> bool {
        self.unavailable_layers.is_empty()
    }

    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }

    pub fn errors(&self) -> Vec<&ValidationIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.severity == Severity::Error)
            .collect()
    }

    pub fn warnings(&self) -> Vec<&ValidationIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.severity == Severity::Warning)
            .collect()
    }

    pub fn unavailable_layers(&self) -> &[LayerUnavailable] {
        &self.unavailable_layers
    }

    pub fn push(&mut self, issue: ValidationIssue) {
        self.issues.push(issue);
    }

    pub fn extend(&mut self, issues: impl IntoIterator<Item = ValidationIssue>) {
        self.issues.extend(issues);
    }

    pub fn mark_unavailable(&mut self, layer: ValidationLayer, reason: impl Into<String>) {
        if !self
            .unavailable_layers
            .iter()
            .any(|unavailable| unavailable.layer == layer)
        {
            self.unavailable_layers.push(LayerUnavailable {
                layer,
                reason: reason.into(),
            });
            self.unavailable_layers.sort_by_key(|entry| entry.layer);
        }
    }

    pub fn sort(&mut self) {
        self.issues.sort_by(compare_issues);
    }
}

fn compare_issues(left: &ValidationIssue, right: &ValidationIssue) -> Ordering {
    left.layer
        .cmp(&right.layer)
        .then_with(|| left.severity.cmp(&right.severity))
        .then_with(|| left.rule_id.cmp(&right.rule_id))
        .then_with(|| left.path.cmp(&right.path))
        .then_with(|| left.message.cmp(&right.message))
}

#[cfg(feature = "serde")]
impl serde::Serialize for ValidationReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let errors = self.errors();
        let warnings = self.warnings();
        let mut state = serializer.serialize_struct("ValidationReport", 5)?;
        state.serialize_field("valid", &self.valid())?;
        state.serialize_field("complete", &self.complete())?;
        state.serialize_field("errors", &errors)?;
        state.serialize_field("warnings", &warnings)?;
        state.serialize_field("unavailable_layers", &self.unavailable_layers)?;
        state.end()
    }
}
