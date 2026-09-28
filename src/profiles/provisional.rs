//! Conservative, opt-in profile checks that can run before licensed rule packs.
//!
//! These rules deliberately cover only deterministic scalar constraints already
//! implemented by the SDK. They are not presented as CBPR+ or SEPA
//! certification. Once an authoritative release artifact is available, the
//! same rule objects can be replaced by release-bound descriptors without
//! changing the generated message model or validation target abstraction.

use crate::helpers::{Bic, Currency, Iban};
use crate::validation::{
    FieldPath, Rule, RuleDescriptor, RuleId, RuleSet, Severity, ValidationContext, ValidationIssue,
    ValidationLayer, ValidationTarget,
};

const IBAN_PATH: &str = "$financial.iban";
const BIC_PATH: &str = "$financial.bic";
const CURRENCY_PATH: &str = "$financial.currency";
const HEADER_MESSAGE_ID_PATH: &str = "$header.biz_msg_id";
const GROUP_MESSAGE_ID_PATH: &str = "$message.group_msg_id";
const ANY_BIC_PATH: &str = "$party.any_bic";
const PARTY_NAME_PATH: &str = "$party.name";
const POSTAL_ADDRESS_PATH: &str = "$party.postal_address";
const STRUCTURED_REMITTANCE_PATH: &str = "$remittance.structured";
const SETTLEMENT_CURRENCY_PATH: &str = "$settlement.currency";
const SETTLEMENT_AMOUNT_PATH: &str = "$settlement.amount";

static CBPR_IBAN_DESCRIPTOR: RuleDescriptor = RuleDescriptor {
    id: RuleId::new("CBPRPLUS-PROVISIONAL-IBAN"),
    layer: ValidationLayer::Profile,
    title: "CBPR+ provisional IBAN syntax",
    reason: "The supplied IBAN must pass the SDK's ISO 13616 syntax and checksum validator.",
    source: Some(
        "public ISO 13616 algorithm; CBPR+ field applicability pending authoritative release",
    ),
    profile: Some("cbpr-plus"),
    profile_version: Some("provisional"),
    affected_messages: &[],
};

static CBPR_BIC_DESCRIPTOR: RuleDescriptor = RuleDescriptor {
    id: RuleId::new("CBPRPLUS-PROVISIONAL-BIC"),
    layer: ValidationLayer::Profile,
    title: "CBPR+ provisional BIC syntax",
    reason: "The supplied BIC must pass the SDK's ISO 9362 syntax validator.",
    source: Some("public ISO 9362 syntax; CBPR+ field applicability pending authoritative release"),
    profile: Some("cbpr-plus"),
    profile_version: Some("provisional"),
    affected_messages: &[],
};

static CBPR_CURRENCY_DESCRIPTOR: RuleDescriptor = RuleDescriptor {
    id: RuleId::new("CBPRPLUS-PROVISIONAL-CURRENCY"),
    layer: ValidationLayer::Profile,
    title: "CBPR+ provisional currency code",
    reason: "The supplied currency must be present in the pinned ISO 4217 data set.",
    source: Some(
        "public ISO 4217 code list; CBPR+ field applicability pending authoritative release",
    ),
    profile: Some("cbpr-plus"),
    profile_version: Some("provisional"),
    affected_messages: &[],
};

static SEPA_IBAN_DESCRIPTOR: RuleDescriptor = RuleDescriptor {
    id: RuleId::new("SEPA-SCT-PROVISIONAL-IBAN"),
    layer: ValidationLayer::Profile,
    title: "SEPA SCT provisional IBAN syntax",
    reason: "The supplied account identifier must pass the SDK's ISO 13616 syntax and checksum validator.",
    source: Some(
        "public EPC scheme scope plus ISO 13616 algorithm; exact rulebook mapping pending",
    ),
    profile: Some("sepa-sct"),
    profile_version: Some("provisional"),
    affected_messages: &[],
};

static SEPA_CURRENCY_DESCRIPTOR: RuleDescriptor = RuleDescriptor {
    id: RuleId::new("SEPA-SCT-PROVISIONAL-CURRENCY"),
    layer: ValidationLayer::Profile,
    title: "SEPA SCT provisional currency code",
    reason: "The supplied currency must be a recognized ISO 4217 code; scheme-specific settlement restrictions are not inferred.",
    source: Some("public ISO 4217 code list; exact EPC rulebook mapping pending"),
    profile: Some("sepa-sct"),
    profile_version: Some("provisional"),
    affected_messages: &[],
};

static CBPR_HEADER_ID_DESCRIPTOR: RuleDescriptor = RuleDescriptor {
    id: RuleId::new("CBPRPLUS-PROVISIONAL-HEADER-MESSAGE-ID"),
    layer: ValidationLayer::Profile,
    title: "CBPR+ provisional header/message identity",
    reason: "Business message identifier must match the generated message group identifier.",
    source: Some(
        "CBPR+ candidate rule shape from public reference implementation; exact release mapping pending",
    ),
    profile: Some("cbpr-plus"),
    profile_version: Some("provisional"),
    affected_messages: &[],
};

static CBPR_ANY_BIC_DESCRIPTOR: RuleDescriptor = RuleDescriptor {
    id: RuleId::new("CBPRPLUS-PROVISIONAL-ANYBIC-EXCLUSIVE"),
    layer: ValidationLayer::Profile,
    title: "CBPR+ provisional AnyBIC exclusivity",
    reason: "AnyBIC must not be combined with party name or postal address in the same projection.",
    source: Some(
        "CBPR+ candidate rule shape from public reference implementation; exact release mapping pending",
    ),
    profile: Some("cbpr-plus"),
    profile_version: Some("provisional"),
    affected_messages: &[],
};

static CBPR_REMITTANCE_DESCRIPTOR: RuleDescriptor = RuleDescriptor {
    id: RuleId::new("CBPRPLUS-PROVISIONAL-REMITTANCE-LENGTH"),
    layer: ValidationLayer::Profile,
    title: "CBPR+ provisional structured remittance length",
    reason: "Structured remittance text must not exceed 9,000 characters.",
    source: Some(
        "CBPR+ candidate rule shape from public reference implementation; exact release mapping pending",
    ),
    profile: Some("cbpr-plus"),
    profile_version: Some("provisional"),
    affected_messages: &[],
};

static CBPR_COMMODITY_DESCRIPTOR: RuleDescriptor = RuleDescriptor {
    id: RuleId::new("CBPRPLUS-PROVISIONAL-COMMODITY-CURRENCY"),
    layer: ValidationLayer::Profile,
    title: "CBPR+ provisional settlement currency restriction",
    reason: "XAU, XAG, XPD, and XPT are not accepted by this provisional CBPR+ candidate rule.",
    source: Some(
        "CBPR+ candidate rule shape from public reference implementation; exact release mapping pending",
    ),
    profile: Some("cbpr-plus"),
    profile_version: Some("provisional"),
    affected_messages: &[],
};

static SEPA_EUR_DESCRIPTOR: RuleDescriptor = RuleDescriptor {
    id: RuleId::new("SEPA-SCT-PROVISIONAL-EUR"),
    layer: ValidationLayer::Profile,
    title: "SEPA SCT provisional EUR settlement",
    reason: "SEPA SCT payment currency must be EUR in this provisional candidate rule.",
    source: Some(
        "EPC SCT candidate rule shape from public implementation reference; exact edition mapping pending",
    ),
    profile: Some("sepa-sct"),
    profile_version: Some("provisional"),
    affected_messages: &[],
};

static SEPA_INST_LIMIT_DESCRIPTOR: RuleDescriptor = RuleDescriptor {
    id: RuleId::new("SEPA-SCT-INST-PROVISIONAL-AMOUNT-LIMIT"),
    layer: ValidationLayer::Profile,
    title: "SEPA SCT Inst provisional amount limit",
    reason: "SEPA SCT Inst amount must not exceed EUR 100,000 in this provisional candidate rule.",
    source: Some(
        "SEPA Inst candidate rule shape from public implementation reference; exact edition mapping pending",
    ),
    profile: Some("sepa-sct-inst"),
    profile_version: Some("provisional"),
    affected_messages: &[],
};
static SEPA_INST_IBAN_DESCRIPTOR: RuleDescriptor = RuleDescriptor {
    id: RuleId::new("SEPA-SCT-INST-PROVISIONAL-IBAN"),
    layer: ValidationLayer::Profile,
    title: "SEPA SCT Inst provisional IBAN syntax",
    reason: "The supplied account identifier must pass the SDK's ISO 13616 syntax and checksum validator.",
    source: Some("public ISO 13616 algorithm; exact EPC SCT Inst edition mapping pending"),
    profile: Some("sepa-sct-inst"),
    profile_version: Some("provisional"),
    affected_messages: &[],
};
static SEPA_INST_CURRENCY_DESCRIPTOR: RuleDescriptor = RuleDescriptor {
    id: RuleId::new("SEPA-SCT-INST-PROVISIONAL-CURRENCY"),
    layer: ValidationLayer::Profile,
    title: "SEPA SCT Inst provisional currency code",
    reason: "The supplied currency must be EUR in this provisional candidate rule.",
    source: Some("public EPC scheme scope plus ISO 4217; exact edition mapping pending"),
    profile: Some("sepa-sct-inst"),
    profile_version: Some("provisional"),
    affected_messages: &[],
};

struct ScalarRule {
    descriptor: &'static RuleDescriptor,
    path: &'static str,
    validate: fn(&str) -> bool,
}

struct PairEqualityRule {
    descriptor: &'static RuleDescriptor,
    left: &'static str,
    right: &'static str,
}

impl Rule for PairEqualityRule {
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
            (Some(left), Some(right)) if left != right => vec![ValidationIssue::new(
                self.descriptor,
                Severity::Error,
                FieldPath::new(self.left),
                self.descriptor.reason,
            )],
            _ => Vec::new(),
        }
    }
}

struct ExclusivePartyRule;
impl Rule for ExclusivePartyRule {
    fn descriptor(&self) -> &'static RuleDescriptor {
        &CBPR_ANY_BIC_DESCRIPTOR
    }
    fn field_paths(&self) -> Vec<&'static str> {
        vec![ANY_BIC_PATH, PARTY_NAME_PATH, POSTAL_ADDRESS_PATH]
    }
    fn evaluate(
        &self,
        target: &ValidationTarget<'_>,
        _: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue> {
        if target.present(ANY_BIC_PATH)
            && (target.present(PARTY_NAME_PATH) || target.present(POSTAL_ADDRESS_PATH))
        {
            vec![ValidationIssue::new(
                &CBPR_ANY_BIC_DESCRIPTOR,
                Severity::Error,
                FieldPath::new(ANY_BIC_PATH),
                CBPR_ANY_BIC_DESCRIPTOR.reason,
            )]
        } else {
            Vec::new()
        }
    }
}

struct LengthRule;
impl Rule for LengthRule {
    fn descriptor(&self) -> &'static RuleDescriptor {
        &CBPR_REMITTANCE_DESCRIPTOR
    }
    fn field_paths(&self) -> Vec<&'static str> {
        vec![STRUCTURED_REMITTANCE_PATH]
    }
    fn evaluate(
        &self,
        target: &ValidationTarget<'_>,
        _: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue> {
        target
            .values(STRUCTURED_REMITTANCE_PATH)
            .filter(|value| value.chars().count() > 9_000)
            .map(|_| {
                ValidationIssue::new(
                    &CBPR_REMITTANCE_DESCRIPTOR,
                    Severity::Error,
                    FieldPath::new(STRUCTURED_REMITTANCE_PATH),
                    CBPR_REMITTANCE_DESCRIPTOR.reason,
                )
            })
            .collect()
    }
}

struct CurrencyRule {
    descriptor: &'static RuleDescriptor,
    allowed: &'static [&'static str],
    path: &'static str,
}
impl Rule for CurrencyRule {
    fn descriptor(&self) -> &'static RuleDescriptor {
        self.descriptor
    }
    fn field_paths(&self) -> Vec<&'static str> {
        vec![self.path]
    }
    fn evaluate(
        &self,
        target: &ValidationTarget<'_>,
        _: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue> {
        target
            .values(self.path)
            .filter(|value| !self.allowed.iter().any(|allowed| allowed == value))
            .map(|_| {
                ValidationIssue::new(
                    self.descriptor,
                    Severity::Error,
                    FieldPath::new(self.path),
                    self.descriptor.reason,
                )
            })
            .collect()
    }
}

struct CommodityCurrencyRule;
impl Rule for CommodityCurrencyRule {
    fn descriptor(&self) -> &'static RuleDescriptor {
        &CBPR_COMMODITY_DESCRIPTOR
    }
    fn field_paths(&self) -> Vec<&'static str> {
        vec![SETTLEMENT_CURRENCY_PATH]
    }
    fn evaluate(
        &self,
        target: &ValidationTarget<'_>,
        _: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue> {
        target
            .values(SETTLEMENT_CURRENCY_PATH)
            .filter(|value| matches!(*value, "XAU" | "XAG" | "XPD" | "XPT"))
            .map(|_| {
                ValidationIssue::new(
                    &CBPR_COMMODITY_DESCRIPTOR,
                    Severity::Error,
                    FieldPath::new(SETTLEMENT_CURRENCY_PATH),
                    CBPR_COMMODITY_DESCRIPTOR.reason,
                )
            })
            .collect()
    }
}

struct AmountLimitRule;
impl Rule for AmountLimitRule {
    fn descriptor(&self) -> &'static RuleDescriptor {
        &SEPA_INST_LIMIT_DESCRIPTOR
    }
    fn field_paths(&self) -> Vec<&'static str> {
        vec![SETTLEMENT_CURRENCY_PATH, SETTLEMENT_AMOUNT_PATH]
    }
    fn evaluate(
        &self,
        target: &ValidationTarget<'_>,
        _: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue> {
        let currency = target.text(SETTLEMENT_CURRENCY_PATH);
        let amount = target.text(SETTLEMENT_AMOUNT_PATH);
        if currency == Some("EUR") && amount.is_some_and(exceeds_eur_limit) {
            vec![ValidationIssue::new(
                &SEPA_INST_LIMIT_DESCRIPTOR,
                Severity::Error,
                FieldPath::new(SETTLEMENT_AMOUNT_PATH),
                SEPA_INST_LIMIT_DESCRIPTOR.reason,
            )]
        } else {
            Vec::new()
        }
    }
}

fn exceeds_eur_limit(value: &str) -> bool {
    let mut parts = value.split('.');
    let whole = parts.next().unwrap_or("").trim_start_matches('0');
    let fraction = parts.next().unwrap_or("");
    if parts.next().is_some()
        || !whole.bytes().all(|byte| byte.is_ascii_digit())
        || !fraction.bytes().all(|byte| byte.is_ascii_digit())
    {
        return false;
    }
    whole.len() > 5
        || (whole.len() == 5 && whole > "100000")
        || (whole == "100000" && fraction.bytes().any(|byte| byte != b'0'))
}

impl Rule for ScalarRule {
    fn descriptor(&self) -> &'static RuleDescriptor {
        self.descriptor
    }

    fn field_paths(&self) -> Vec<&'static str> {
        vec![self.path]
    }

    fn evaluate(
        &self,
        target: &ValidationTarget<'_>,
        _context: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue> {
        target
            .values(self.path)
            .filter(|value| !(self.validate)(value))
            .map(|_| {
                ValidationIssue::new(
                    self.descriptor,
                    Severity::Error,
                    FieldPath::new(self.path),
                    self.descriptor.reason,
                )
            })
            .collect()
    }
}

static CBPR_IBAN_RULE: ScalarRule = ScalarRule {
    descriptor: &CBPR_IBAN_DESCRIPTOR,
    path: IBAN_PATH,
    validate: |value| Iban::parse(value).is_ok(),
};
static CBPR_BIC_RULE: ScalarRule = ScalarRule {
    descriptor: &CBPR_BIC_DESCRIPTOR,
    path: BIC_PATH,
    validate: |value| Bic::parse(value).is_ok(),
};
static CBPR_CURRENCY_RULE: ScalarRule = ScalarRule {
    descriptor: &CBPR_CURRENCY_DESCRIPTOR,
    path: CURRENCY_PATH,
    validate: |value| Currency::parse(value).is_ok(),
};
static SEPA_IBAN_RULE: ScalarRule = ScalarRule {
    descriptor: &SEPA_IBAN_DESCRIPTOR,
    path: IBAN_PATH,
    validate: |value| Iban::parse(value).is_ok(),
};
static SEPA_CURRENCY_RULE: ScalarRule = ScalarRule {
    descriptor: &SEPA_CURRENCY_DESCRIPTOR,
    path: CURRENCY_PATH,
    validate: |value| Currency::parse(value).is_ok(),
};
static CBPR_HEADER_ID_RULE: PairEqualityRule = PairEqualityRule {
    descriptor: &CBPR_HEADER_ID_DESCRIPTOR,
    left: HEADER_MESSAGE_ID_PATH,
    right: GROUP_MESSAGE_ID_PATH,
};
static CBPR_ANY_BIC_RULE: ExclusivePartyRule = ExclusivePartyRule;
static CBPR_REMITTANCE_RULE: LengthRule = LengthRule;
static CBPR_COMMODITY_RULE: CommodityCurrencyRule = CommodityCurrencyRule;
static SEPA_EUR_RULE: CurrencyRule = CurrencyRule {
    descriptor: &SEPA_EUR_DESCRIPTOR,
    allowed: &["EUR"],
    path: SETTLEMENT_CURRENCY_PATH,
};
static SEPA_INST_LIMIT_RULE: AmountLimitRule = AmountLimitRule;
static SEPA_INST_IBAN_RULE: ScalarRule = ScalarRule {
    descriptor: &SEPA_INST_IBAN_DESCRIPTOR,
    path: IBAN_PATH,
    validate: |value| Iban::parse(value).is_ok(),
};
static SEPA_INST_CURRENCY_RULE: CurrencyRule = CurrencyRule {
    descriptor: &SEPA_INST_CURRENCY_DESCRIPTOR,
    allowed: &["EUR"],
    path: SETTLEMENT_CURRENCY_PATH,
};

/// Provisional CBPR+ checks. This is intentionally not registered as an
/// authoritative release and must be labelled `provisional` by adapters.
pub fn cbpr_plus_provisional_rules() -> RuleSet<'static> {
    RuleSet::new(
        "cbpr-plus",
        Some("provisional"),
        ValidationLayer::Profile,
        &[
            &CBPR_IBAN_RULE,
            &CBPR_BIC_RULE,
            &CBPR_CURRENCY_RULE,
            &CBPR_HEADER_ID_RULE,
            &CBPR_ANY_BIC_RULE,
            &CBPR_REMITTANCE_RULE,
            &CBPR_COMMODITY_RULE,
        ],
    )
    .expect("static provisional CBPR+ rule identities are valid")
}

/// Provisional SEPA SCT checks. These are useful for developer feedback but do
/// not assert EPC scheme acceptance or completeness.
pub fn sepa_sct_provisional_rules() -> RuleSet<'static> {
    RuleSet::new(
        "sepa-sct",
        Some("provisional"),
        ValidationLayer::Profile,
        &[&SEPA_IBAN_RULE, &SEPA_CURRENCY_RULE, &SEPA_EUR_RULE],
    )
    .expect("static provisional SEPA rule identities are valid")
}

/// Provisional SCT Inst checks, kept separate from SCT because the scheme
/// amount constraint must never be silently reused across profiles.
pub fn sepa_sct_inst_provisional_rules() -> RuleSet<'static> {
    RuleSet::new(
        "sepa-sct-inst",
        Some("provisional"),
        ValidationLayer::Profile,
        &[
            &SEPA_INST_IBAN_RULE,
            &SEPA_INST_CURRENCY_RULE,
            &SEPA_INST_LIMIT_RULE,
        ],
    )
    .expect("static provisional SCT Inst identities are valid")
}
