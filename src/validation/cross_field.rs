//! Typed, reusable cross-field rule combinators.
//!
//! Combinators operate on short-lived paths projected from canonical generated
//! structs. They never own or reproduce an ISO 20022 message model.

use super::{
    FieldPath, Rule, RuleDescriptor, Severity, ValidationContext, ValidationIssue, ValidationTarget,
};
use crate::helpers::Money;
use core::ops::RangeInclusive;

fn issue(descriptor: &'static RuleDescriptor, path: &'static str) -> ValidationIssue {
    ValidationIssue::new(
        descriptor,
        Severity::Error,
        FieldPath::new(path),
        descriptor.reason,
    )
}

/// A typed condition used by conditional-presence rules.
#[derive(Debug, Clone, Copy)]
pub enum FieldPredicate {
    Present(&'static str),
    Equals {
        path: &'static str,
        expected: &'static str,
    },
}

impl FieldPredicate {
    pub const fn present(path: &'static str) -> Self {
        Self::Present(path)
    }

    pub const fn equals(path: &'static str, expected: &'static str) -> Self {
        Self::Equals { path, expected }
    }

    fn evaluate(self, target: &ValidationTarget<'_>) -> bool {
        match self {
            Self::Present(path) => target.present(path),
            Self::Equals { path, expected } => target.values(path).any(|value| value == expected),
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::Present(path) | Self::Equals { path, .. } => path,
        }
    }
}

/// Require one field only when a typed predicate holds.
pub struct RequiredIfRule {
    descriptor: &'static RuleDescriptor,
    condition: FieldPredicate,
    required: &'static str,
}

impl RequiredIfRule {
    pub const fn new(
        descriptor: &'static RuleDescriptor,
        condition: FieldPredicate,
        required: &'static str,
    ) -> Self {
        Self {
            descriptor,
            condition,
            required,
        }
    }
}

impl Rule for RequiredIfRule {
    fn descriptor(&self) -> &'static RuleDescriptor {
        self.descriptor
    }

    fn field_paths(&self) -> Vec<&'static str> {
        vec![self.condition.path(), self.required]
    }

    fn evaluate(
        &self,
        target: &ValidationTarget<'_>,
        _: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue> {
        if self.condition.evaluate(target) && !target.present(self.required) {
            vec![issue(self.descriptor, self.required)]
        } else {
            Vec::new()
        }
    }
}

/// Reject simultaneous presence of two or more configured alternatives.
pub struct MutualExclusionRule {
    descriptor: &'static RuleDescriptor,
    paths: Vec<&'static str>,
}

impl MutualExclusionRule {
    pub fn new(descriptor: &'static RuleDescriptor, paths: &[&'static str]) -> Self {
        Self {
            descriptor,
            paths: paths.to_vec(),
        }
    }
}

impl Rule for MutualExclusionRule {
    fn descriptor(&self) -> &'static RuleDescriptor {
        self.descriptor
    }

    fn field_paths(&self) -> Vec<&'static str> {
        self.paths.clone()
    }

    fn evaluate(
        &self,
        target: &ValidationTarget<'_>,
        _: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue> {
        let present: Vec<_> = self
            .paths
            .iter()
            .copied()
            .filter(|path| target.present(path))
            .collect();
        if present.len() > 1 {
            present
                .into_iter()
                .map(|path| issue(self.descriptor, path))
                .collect()
        } else {
            Vec::new()
        }
    }
}

/// Require at least one member of a configured group.
pub struct AtLeastOneRule {
    descriptor: &'static RuleDescriptor,
    paths: Vec<&'static str>,
}

impl AtLeastOneRule {
    pub fn new(descriptor: &'static RuleDescriptor, paths: &[&'static str]) -> Self {
        Self {
            descriptor,
            paths: paths.to_vec(),
        }
    }
}

impl Rule for AtLeastOneRule {
    fn descriptor(&self) -> &'static RuleDescriptor {
        self.descriptor
    }

    fn field_paths(&self) -> Vec<&'static str> {
        self.paths.clone()
    }

    fn evaluate(
        &self,
        target: &ValidationTarget<'_>,
        _: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue> {
        if self.paths.iter().any(|path| target.present(path)) {
            Vec::new()
        } else {
            self.paths
                .first()
                .map(|path| vec![issue(self.descriptor, path)])
                .unwrap_or_default()
        }
    }
}

/// When a trigger collection is non-empty, constrain another collection's cardinality.
pub struct CardinalityDependencyRule {
    descriptor: &'static RuleDescriptor,
    trigger: &'static str,
    dependent: &'static str,
    expected: CardinalityExpectation,
}

enum CardinalityExpectation {
    Range(RangeInclusive<usize>),
    DeclaredAt(&'static str),
}

impl CardinalityDependencyRule {
    pub const fn new(
        descriptor: &'static RuleDescriptor,
        trigger: &'static str,
        dependent: &'static str,
        expected: RangeInclusive<usize>,
    ) -> Self {
        Self {
            descriptor,
            trigger,
            dependent,
            expected: CardinalityExpectation::Range(expected),
        }
    }

    /// Require the collection count to equal a decimal count declared at
    /// another generated-field path.
    pub const fn matches_declared(
        descriptor: &'static RuleDescriptor,
        collection: &'static str,
        declared: &'static str,
    ) -> Self {
        Self {
            descriptor,
            trigger: collection,
            dependent: collection,
            expected: CardinalityExpectation::DeclaredAt(declared),
        }
    }
}

impl Rule for CardinalityDependencyRule {
    fn descriptor(&self) -> &'static RuleDescriptor {
        self.descriptor
    }

    fn field_paths(&self) -> Vec<&'static str> {
        let mut paths = vec![self.trigger, self.dependent];
        if let CardinalityExpectation::DeclaredAt(declared) = &self.expected {
            paths.push(*declared);
        }
        paths.sort_unstable();
        paths.dedup();
        paths
    }

    fn evaluate(
        &self,
        target: &ValidationTarget<'_>,
        _: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue> {
        match &self.expected {
            CardinalityExpectation::Range(expected) => {
                let count = target.count(self.dependent);
                if target.count(self.trigger) > 0 && !expected.contains(&count) {
                    vec![issue(self.descriptor, self.dependent)]
                } else {
                    Vec::new()
                }
            }
            CardinalityExpectation::DeclaredAt(declared) => {
                let declared_count = target.text(declared).and_then(|value| value.parse().ok());
                if declared_count != Some(target.count(self.dependent)) {
                    vec![issue(self.descriptor, declared)]
                } else {
                    Vec::new()
                }
            }
        }
    }
}

/// Apply a typed relation to two present scalar fields.
pub struct FieldRelationshipRule {
    descriptor: &'static RuleDescriptor,
    left: &'static str,
    right: &'static str,
    relation: fn(&str, &str) -> bool,
}

impl FieldRelationshipRule {
    pub const fn new(
        descriptor: &'static RuleDescriptor,
        left: &'static str,
        right: &'static str,
        relation: fn(&str, &str) -> bool,
    ) -> Self {
        Self {
            descriptor,
            left,
            right,
            relation,
        }
    }
}

impl Rule for FieldRelationshipRule {
    fn descriptor(&self) -> &'static RuleDescriptor {
        self.descriptor
    }

    fn field_paths(&self) -> Vec<&'static str> {
        vec![self.left, self.right]
    }

    fn evaluate(
        &self,
        target: &ValidationTarget<'_>,
        _: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue> {
        match (target.text(self.left), target.text(self.right)) {
            (Some(left), Some(right)) if !(self.relation)(left, right) => {
                vec![issue(self.descriptor, self.right)]
            }
            _ => Vec::new(),
        }
    }
}

/// Validate a currency and amount as one relationship.
pub struct CurrencyAmountRule {
    descriptor: &'static RuleDescriptor,
    currency: &'static str,
    amount: &'static str,
}

impl CurrencyAmountRule {
    pub const fn new(
        descriptor: &'static RuleDescriptor,
        currency: &'static str,
        amount: &'static str,
    ) -> Self {
        Self {
            descriptor,
            currency,
            amount,
        }
    }
}

impl Rule for CurrencyAmountRule {
    fn descriptor(&self) -> &'static RuleDescriptor {
        self.descriptor
    }

    fn field_paths(&self) -> Vec<&'static str> {
        vec![self.currency, self.amount]
    }

    fn evaluate(
        &self,
        target: &ValidationTarget<'_>,
        _: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue> {
        target
            .values(self.currency)
            .zip(target.values(self.amount))
            .filter(|(currency, amount)| Money::parse(currency, amount).is_err())
            .map(|_| issue(self.descriptor, self.amount))
            .collect()
    }
}

/// Require an agent whenever its related account is present.
pub struct AgentAccountRule {
    descriptor: &'static RuleDescriptor,
    account: &'static str,
    agent: &'static str,
}

impl AgentAccountRule {
    pub const fn new(
        descriptor: &'static RuleDescriptor,
        account: &'static str,
        agent: &'static str,
    ) -> Self {
        Self {
            descriptor,
            account,
            agent,
        }
    }
}

impl Rule for AgentAccountRule {
    fn descriptor(&self) -> &'static RuleDescriptor {
        self.descriptor
    }

    fn field_paths(&self) -> Vec<&'static str> {
        vec![self.account, self.agent]
    }

    fn evaluate(
        &self,
        target: &ValidationTarget<'_>,
        _: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue> {
        if target.present(self.account) && !target.present(self.agent) {
            vec![issue(self.descriptor, self.agent)]
        } else {
            Vec::new()
        }
    }
}
