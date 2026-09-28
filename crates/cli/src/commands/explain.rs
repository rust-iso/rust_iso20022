use rust_iso20022::validation::{Rule, RuleExplanation, RuleRegistry};

use super::CommandError;

/// Return the exact immutable descriptor owned by the executable core rule.
pub fn explain(rule_id: &str) -> Result<RuleExplanation, CommandError> {
    explain_from_rules(rust_iso20022::validation::financial_rules(), rule_id)
}

/// Map any core registry, including an exact profile release's rule set,
/// without copying explanation metadata into the adapter.
pub fn explain_from_rules(
    rules: &[&dyn Rule],
    rule_id: &str,
) -> Result<RuleExplanation, CommandError> {
    RuleRegistry::new(rules)
        .map_err(|_| CommandError::internal())?
        .explain_details_str(rule_id)
        .ok_or_else(CommandError::rule_not_found)
}
