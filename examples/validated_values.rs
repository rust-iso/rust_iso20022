use rust_iso20022::helpers::{BicFi, Iban, Money};
use rust_iso20022::validation::{
    RuleRegistry, ValidationContext, ValidationTarget, financial_rules,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let iban = Iban::parse("DE89370400440532013000")?;
    let bic = BicFi::parse("DEUTDEFF")?;
    let money = Money::parse("EUR", "125.50")?;

    let generated_iban_value: String = iban.into();
    let generated_bic_value: String = bic.into();
    let (currency, generated_amount_value) = money.into_parts();
    assert_eq!(generated_iban_value, "DE89370400440532013000");
    assert_eq!(generated_bic_value, "DEUTDEFF");
    assert_eq!(currency.as_str(), "EUR");
    assert_eq!(generated_amount_value, "125.50");

    let fields = [("$financial.iban", "DE88370400440532013000")];
    let registry = RuleRegistry::new(financial_rules())?;
    let report = registry.validate(
        &ValidationTarget::from_pairs(&fields),
        &ValidationContext::default(),
    );
    assert_eq!(
        report.errors()[0].rule_id.as_str(),
        "ISO20022-L2-IBAN-CHECKSUM"
    );
    Ok(())
}
