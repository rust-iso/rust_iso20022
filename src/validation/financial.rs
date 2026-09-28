//! Explainable ISO semantic rules for validated financial scalar fields.

use super::{
    FieldPath, Rule, RuleDescriptor, RuleId, Severity, ValidationContext, ValidationIssue,
    ValidationLayer, ValidationTarget,
};
use crate::helpers::{
    AccountIdentifier, Bic, ClearingIdentifier, CountryCode, Currency, Iban, IsoDate, IsoDateTime,
    Lei, Money, ValueError,
};

type Validator = fn(&ValidationTarget<'_>) -> Vec<(&'static str, ValueError)>;

struct FinancialRule {
    descriptor: &'static RuleDescriptor,
    validator: Validator,
    field_paths: &'static [&'static str],
}

impl Rule for FinancialRule {
    fn descriptor(&self) -> &'static RuleDescriptor {
        self.descriptor
    }

    fn field_paths(&self) -> Vec<&'static str> {
        self.field_paths.to_vec()
    }

    fn evaluate(
        &self,
        target: &ValidationTarget<'_>,
        _context: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue> {
        (self.validator)(target)
            .into_iter()
            .filter(|(_, error)| error.rule_id() == self.descriptor.id)
            .map(|(path, _)| {
                ValidationIssue::new(
                    self.descriptor,
                    Severity::Error,
                    FieldPath::new(path),
                    self.descriptor.reason,
                )
            })
            .collect()
    }
}

macro_rules! financial_rule {
    ($descriptor:ident, $rule:ident, $id:literal, $title:literal, $reason:literal, $source:path, $validator:ident, $paths:expr) => {
        static $descriptor: RuleDescriptor = RuleDescriptor {
            id: RuleId::new($id),
            layer: ValidationLayer::IsoSemantic,
            title: $title,
            reason: $reason,
            source: Some($source),
            profile: None,
            profile_version: None,
            affected_messages: &[],
        };
        static $rule: FinancialRule = FinancialRule {
            descriptor: &$descriptor,
            validator: $validator,
            field_paths: $paths,
        };
    };
}

const ISO_SOURCE: &str = "ISO 20022 schema-derived scalar constraint";
const IBAN_SOURCE: &str = "ISO 13616 and SWIFT IBAN Registry release 102 (June 2026)";
const BIC_SOURCE: &str = "ISO 9362 syntax; no SWIFT BIC Directory existence lookup";
const LEI_SOURCE: &str = "ISO 17442 checksum syntax; no GLEIF directory status lookup";
const CURRENCY_SOURCE: &str = "ISO 4217 SIX List One published 2026-09-17";
const COUNTRY_SOURCE: &str = "ISO 3166-1 alpha-2 snapshot from rust_iso3166 0.2.0";

financial_rule!(
    IBAN_CHARSET_D,
    IBAN_CHARSET_R,
    "ISO20022-L2-IBAN-CHARSET",
    "IBAN character set",
    "IBAN must contain only canonical uppercase ASCII letters and digits.",
    IBAN_SOURCE,
    validate_iban,
    &["$financial.iban"]
);
financial_rule!(
    IBAN_LENGTH_D,
    IBAN_LENGTH_R,
    "ISO20022-L2-IBAN-LENGTH",
    "IBAN length",
    "IBAN length must match the registered national format.",
    IBAN_SOURCE,
    validate_iban,
    &["$financial.iban"]
);
financial_rule!(
    IBAN_FORMAT_D,
    IBAN_FORMAT_R,
    "ISO20022-L2-IBAN-FORMAT",
    "IBAN header",
    "IBAN must start with an uppercase country code and two check digits.",
    IBAN_SOURCE,
    validate_iban,
    &["$financial.iban"]
);
financial_rule!(
    IBAN_COUNTRY_D,
    IBAN_COUNTRY_R,
    "ISO20022-L2-IBAN-COUNTRY",
    "IBAN registry country",
    "IBAN country must exist in the pinned SWIFT IBAN Registry snapshot.",
    IBAN_SOURCE,
    validate_iban,
    &["$financial.iban"]
);
financial_rule!(
    IBAN_BBAN_D,
    IBAN_BBAN_R,
    "ISO20022-L2-IBAN-BBAN-FORMAT",
    "IBAN national structure",
    "IBAN BBAN characters must match the registered national structure.",
    IBAN_SOURCE,
    validate_iban,
    &["$financial.iban"]
);
financial_rule!(
    IBAN_CHECKSUM_D,
    IBAN_CHECKSUM_R,
    "ISO20022-L2-IBAN-CHECKSUM",
    "IBAN checksum",
    "IBAN must satisfy the ISO 13616 MOD-97 checksum.",
    IBAN_SOURCE,
    validate_iban,
    &["$financial.iban"]
);

financial_rule!(
    BIC_LENGTH_D,
    BIC_LENGTH_R,
    "ISO20022-L2-BIC-LENGTH",
    "BIC length",
    "BIC must contain 8 or 11 characters.",
    BIC_SOURCE,
    validate_bic,
    &["$financial.bic"]
);
financial_rule!(
    BIC_FORMAT_D,
    BIC_FORMAT_R,
    "ISO20022-L2-BIC-FORMAT",
    "BIC syntax",
    "BIC components must use the ISO 9362 uppercase ASCII syntax.",
    BIC_SOURCE,
    validate_bic,
    &["$financial.bic"]
);
financial_rule!(
    BIC_COUNTRY_D,
    BIC_COUNTRY_R,
    "ISO20022-L2-BIC-COUNTRY",
    "BIC country",
    "BIC country component must be an assigned ISO 3166-1 alpha-2 code.",
    BIC_SOURCE,
    validate_bic,
    &["$financial.bic"]
);

financial_rule!(
    LEI_LENGTH_D,
    LEI_LENGTH_R,
    "ISO20022-L2-LEI-LENGTH",
    "LEI length",
    "LEI must contain exactly 20 characters.",
    LEI_SOURCE,
    validate_lei,
    &["$financial.lei"]
);
financial_rule!(
    LEI_CHARSET_D,
    LEI_CHARSET_R,
    "ISO20022-L2-LEI-CHARSET",
    "LEI character set",
    "LEI must use uppercase ASCII letters and digits.",
    LEI_SOURCE,
    validate_lei,
    &["$financial.lei"]
);
financial_rule!(
    LEI_CHECKSUM_D,
    LEI_CHECKSUM_R,
    "ISO20022-L2-LEI-CHECKSUM",
    "LEI checksum",
    "LEI must satisfy the ISO 17442 MOD-97 checksum.",
    LEI_SOURCE,
    validate_lei,
    &["$financial.lei"]
);

financial_rule!(
    CURRENCY_D,
    CURRENCY_R,
    "ISO20022-L2-CURRENCY-CODE",
    "Active currency code",
    "Currency must be present in the pinned active ISO 4217 snapshot.",
    CURRENCY_SOURCE,
    validate_currency,
    &["$financial.currency"]
);
financial_rule!(
    COUNTRY_D,
    COUNTRY_R,
    "ISO20022-L2-COUNTRY-CODE",
    "Country code",
    "Country must be present in the pinned ISO 3166-1 alpha-2 snapshot.",
    COUNTRY_SOURCE,
    validate_country,
    &["$financial.country"]
);
financial_rule!(
    AMOUNT_FORMAT_D,
    AMOUNT_FORMAT_R,
    "ISO20022-L2-AMOUNT-FORMAT",
    "Amount syntax",
    "Amount must use non-negative plain decimal notation.",
    ISO_SOURCE,
    validate_money,
    &["$financial.amount_currency", "$financial.amount"]
);
financial_rule!(
    AMOUNT_PRECISION_D,
    AMOUNT_PRECISION_R,
    "ISO20022-L2-AMOUNT-PRECISION",
    "Currency amount precision",
    "Amount fractional precision must not exceed the currency minor units.",
    CURRENCY_SOURCE,
    validate_money,
    &["$financial.amount_currency", "$financial.amount"]
);
financial_rule!(
    AMOUNT_RANGE_D,
    AMOUNT_RANGE_R,
    "ISO20022-L2-AMOUNT-RANGE",
    "Amount digit bound",
    "Amount must not exceed the supported 18 total decimal digits.",
    ISO_SOURCE,
    validate_money,
    &["$financial.amount_currency", "$financial.amount"]
);
financial_rule!(
    DATE_FORMAT_D,
    DATE_FORMAT_R,
    "ISO20022-L2-DATE-TIME-FORMAT",
    "Date and time syntax",
    "Date and date-time values must use the supported ISO lexical form.",
    ISO_SOURCE,
    validate_temporal,
    &["$financial.date", "$financial.datetime"]
);
financial_rule!(
    DATE_RANGE_D,
    DATE_RANGE_R,
    "ISO20022-L2-DATE-TIME-RANGE",
    "Calendar range",
    "Date and date-time components must form a valid calendar value.",
    ISO_SOURCE,
    validate_temporal,
    &["$financial.date", "$financial.datetime"]
);
financial_rule!(
    ACCOUNT_D,
    ACCOUNT_R,
    "ISO20022-L2-ACCOUNT-ID",
    "Account identifier",
    "Account identifier must satisfy the bounded ISO 20022 text syntax.",
    ISO_SOURCE,
    validate_account,
    &["$financial.account"]
);
financial_rule!(
    CLEARING_SCHEME_D,
    CLEARING_SCHEME_R,
    "ISO20022-L2-CLEARING-SCHEME",
    "Clearing scheme",
    "Clearing scheme identifier must satisfy the bounded ISO 20022 text syntax.",
    ISO_SOURCE,
    validate_clearing,
    &["$financial.clearing.scheme"]
);
financial_rule!(
    CLEARING_MEMBER_D,
    CLEARING_MEMBER_R,
    "ISO20022-L2-CLEARING-MEMBER",
    "Clearing member",
    "Clearing member identifier must satisfy the bounded ISO 20022 text syntax.",
    ISO_SOURCE,
    validate_clearing,
    &["$financial.clearing.member"]
);

static FINANCIAL_RULES: &[&dyn Rule] = &[
    &IBAN_CHARSET_R,
    &IBAN_LENGTH_R,
    &IBAN_FORMAT_R,
    &IBAN_COUNTRY_R,
    &IBAN_BBAN_R,
    &IBAN_CHECKSUM_R,
    &BIC_LENGTH_R,
    &BIC_FORMAT_R,
    &BIC_COUNTRY_R,
    &LEI_LENGTH_R,
    &LEI_CHARSET_R,
    &LEI_CHECKSUM_R,
    &CURRENCY_R,
    &COUNTRY_R,
    &AMOUNT_FORMAT_R,
    &AMOUNT_PRECISION_R,
    &AMOUNT_RANGE_R,
    &DATE_FORMAT_R,
    &DATE_RANGE_R,
    &ACCOUNT_R,
    &CLEARING_SCHEME_R,
    &CLEARING_MEMBER_R,
];

/// The executable L2 scalar rules used for both validation and explanation.
pub fn financial_rules() -> &'static [&'static dyn Rule] {
    FINANCIAL_RULES
}

fn one(
    target: &ValidationTarget<'_>,
    path: &'static str,
    parse: impl FnOnce(&str) -> Result<(), ValueError>,
) -> Vec<(&'static str, ValueError)> {
    target
        .text(path)
        .and_then(|value| parse(value).err())
        .map(|error| vec![(path, error)])
        .unwrap_or_default()
}

fn validate_iban(target: &ValidationTarget<'_>) -> Vec<(&'static str, ValueError)> {
    one(target, "$financial.iban", |value| {
        Iban::parse(value).map(drop)
    })
}

fn validate_bic(target: &ValidationTarget<'_>) -> Vec<(&'static str, ValueError)> {
    one(target, "$financial.bic", |value| {
        Bic::parse(value).map(drop)
    })
}

fn validate_lei(target: &ValidationTarget<'_>) -> Vec<(&'static str, ValueError)> {
    one(target, "$financial.lei", |value| {
        Lei::parse(value).map(drop)
    })
}

fn validate_currency(target: &ValidationTarget<'_>) -> Vec<(&'static str, ValueError)> {
    one(target, "$financial.currency", |value| {
        Currency::parse(value).map(drop)
    })
}

fn validate_country(target: &ValidationTarget<'_>) -> Vec<(&'static str, ValueError)> {
    one(target, "$financial.country", |value| {
        CountryCode::parse(value).map(drop)
    })
}

fn validate_money(target: &ValidationTarget<'_>) -> Vec<(&'static str, ValueError)> {
    let Some(currency) = target
        .text("$financial.amount_currency")
        .or_else(|| target.text("$financial.currency"))
    else {
        return Vec::new();
    };
    let Some(amount) = target.text("$financial.amount") else {
        return Vec::new();
    };
    if Currency::parse(currency).is_err() {
        return Vec::new();
    }
    Money::parse(currency, amount)
        .err()
        .map(|error| vec![("$financial.amount", error)])
        .unwrap_or_default()
}

fn validate_temporal(target: &ValidationTarget<'_>) -> Vec<(&'static str, ValueError)> {
    let mut issues = one(target, "$financial.date", |value| {
        IsoDate::parse(value).map(drop)
    });
    issues.extend(one(target, "$financial.datetime", |value| {
        IsoDateTime::parse(value).map(drop)
    }));
    issues
}

fn validate_account(target: &ValidationTarget<'_>) -> Vec<(&'static str, ValueError)> {
    one(target, "$financial.account", |value| {
        AccountIdentifier::parse(value).map(drop)
    })
}

fn validate_clearing(target: &ValidationTarget<'_>) -> Vec<(&'static str, ValueError)> {
    let Some(scheme) = target.text("$financial.clearing.scheme") else {
        return Vec::new();
    };
    let Some(member) = target.text("$financial.clearing.member_id") else {
        return Vec::new();
    };
    ClearingIdentifier::parse(scheme, member)
        .err()
        .map(|error| {
            let path = if error.rule_id().as_str() == "ISO20022-L2-CLEARING-SCHEME" {
                "$financial.clearing.scheme"
            } else {
                "$financial.clearing.member_id"
            };
            vec![(path, error)]
        })
        .unwrap_or_default()
}
