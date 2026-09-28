use std::collections::BTreeSet;
use std::fmt;

use super::{
    Rule, RuleDescriptor, RuleExplanation, RuleId, RuleSet, ValidationContext, ValidationLayer,
    ValidationReport, ValidationTarget,
};

/// Duplicate IDs would make explanation diverge from execution, so registry
/// construction rejects them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DuplicateRuleId {
    pub rule_id: RuleId,
}

impl fmt::Display for DuplicateRuleId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "duplicate validation rule id: {}", self.rule_id)
    }
}

impl std::error::Error for DuplicateRuleId {}

/// The single source for both rule execution and `explain` lookup.
pub struct RuleRegistry<'a> {
    rules: Vec<&'a dyn Rule>,
}

impl fmt::Debug for RuleRegistry<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RuleRegistry")
            .field("rule_count", &self.rules.len())
            .finish()
    }
}

impl<'a> RuleRegistry<'a> {
    pub fn new(rules: &[&'a dyn Rule]) -> Result<Self, DuplicateRuleId> {
        let mut ids = BTreeSet::new();
        for rule in rules {
            let rule_id = rule.descriptor().id;
            if !ids.insert(rule_id) {
                return Err(DuplicateRuleId { rule_id });
            }
        }
        let mut rules = rules.to_vec();
        rules.sort_by_key(|rule| rule.descriptor().id);
        Ok(Self { rules })
    }

    pub fn from_rule_sets(rule_sets: &[RuleSet<'a>]) -> Result<Self, DuplicateRuleId> {
        let rules: Vec<_> = rule_sets
            .iter()
            .flat_map(|rule_set| rule_set.rules.iter().copied())
            .collect();
        Self::new(&rules)
    }

    pub fn explain(&self, rule_id: RuleId) -> Option<&'static RuleDescriptor> {
        self.rules
            .binary_search_by_key(&rule_id, |rule| rule.descriptor().id)
            .ok()
            .map(|index| self.rules[index].descriptor())
    }

    /// Look up the exact descriptor used by execution from a runtime rule ID.
    ///
    /// This is the adapter-friendly counterpart to [`Self::explain`]; it does
    /// not allocate, leak input strings, or maintain a second explanation map.
    pub fn explain_str(&self, rule_id: &str) -> Option<&'static RuleDescriptor> {
        self.rules
            .binary_search_by(|rule| rule.descriptor().id.as_str().cmp(rule_id))
            .ok()
            .map(|index| self.rules[index].descriptor())
    }

    /// Explain a runtime rule ID with descriptor metadata and field paths from
    /// the same executable rule object.
    pub fn explain_details_str(&self, rule_id: &str) -> Option<RuleExplanation> {
        self.rules
            .binary_search_by(|rule| rule.descriptor().id.as_str().cmp(rule_id))
            .ok()
            .map(|index| RuleExplanation {
                descriptor: self.rules[index].descriptor(),
                field_paths: self.rules[index].field_paths(),
            })
    }

    pub fn validate(
        &self,
        target: &ValidationTarget<'_>,
        context: &ValidationContext<'_>,
    ) -> ValidationReport {
        let mut report = ValidationReport::new();
        for rule in &self.rules {
            report.extend(rule.evaluate(target, context));
        }
        report.sort();
        report
    }

    pub(crate) fn validate_layers(
        &self,
        target: &ValidationTarget<'_>,
        context: &ValidationContext<'_>,
        available: impl Fn(ValidationLayer) -> bool,
    ) -> ValidationReport {
        let mut report = ValidationReport::new();
        for rule in &self.rules {
            if available(rule.descriptor().layer) {
                report.extend(rule.evaluate(target, context));
            }
        }
        report.sort();
        report
    }
}
