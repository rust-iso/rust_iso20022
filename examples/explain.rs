//! Resolve an explanation from the same registry that executes the rule.

use rust_iso20022::validation::{RuleRegistry, financial_rules};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = RuleRegistry::new(financial_rules())?;
    let explanation = registry
        .explain_details_str("ISO20022-L2-IBAN-CHECKSUM")
        .expect("registered IBAN checksum rule");
    println!(
        "{}: {}",
        explanation.descriptor.id, explanation.descriptor.reason
    );
    Ok(())
}
