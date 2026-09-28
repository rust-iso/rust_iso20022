//! Generated-message projections into reusable validation rules.
//!
//! Bindings borrow fields from generated structs and attach schema logical
//! paths. They do not create or own a parallel message representation.

use super::cross_field::{
    AgentAccountRule, CardinalityDependencyRule, CurrencyAmountRule, FieldPredicate,
    MutualExclusionRule, RequiredIfRule,
};
use super::{
    DuplicateRuleId, Rule, RuleDescriptor, RuleId, RuleRegistry, ValidationContext,
    ValidationLayer, ValidationReport, ValidationTarget,
};
use crate::generated::pacs::{
    pacs_008_001_08::{CreditTransferTransaction39, Document as Pacs008Document},
    pacs_009_001_08::{CreditTransferTransaction36, Document as Pacs009Document},
};
use crate::metadata::MessageDescriptor;
use core::fmt;

const TX: &str = "Document/FIToFICstmrCdtTrf/CdtTrfTxInf";
const TX_COUNT: &str = "Document/FIToFICstmrCdtTrf/GrpHdr/NbOfTxs";
const INSTD_AMOUNT: &str = "Document/FIToFICstmrCdtTrf/CdtTrfTxInf/InstdAmt";
const EXCHANGE_RATE: &str = "Document/FIToFICstmrCdtTrf/CdtTrfTxInf/XchgRate";
const ACCOUNT_IBAN: &str = "Document/FIToFICstmrCdtTrf/CdtTrfTxInf/DbtrAcct/Id/IBAN";
const ACCOUNT_OTHER: &str = "Document/FIToFICstmrCdtTrf/CdtTrfTxInf/DbtrAcct/Id/Othr";
const SETTLEMENT_CURRENCY: &str = "Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt/@Ccy";
const SETTLEMENT_AMOUNT: &str = "Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt";
const DEBTOR_ACCOUNT: &str = "Document/FIToFICstmrCdtTrf/CdtTrfTxInf/DbtrAcct";
const DEBTOR_AGENT: &str = "Document/FIToFICstmrCdtTrf/CdtTrfTxInf/DbtrAgt";

macro_rules! descriptor {
    ($name:ident, $id:literal, $title:literal, $reason:literal) => {
        static $name: RuleDescriptor = RuleDescriptor {
            id: RuleId::new($id),
            layer: ValidationLayer::IsoSemantic,
            title: $title,
            reason: $reason,
            source: Some("pacs.008.001.08 schema logical-path binding"),
            profile: None,
            profile_version: None,
            affected_messages: &["pacs.008.001.08"],
        };
    };
}

descriptor!(
    INSTD_XCHG,
    "ISO20022-L2-PACS008-INSTDAMT-XCHGRATE",
    "Instructed amount exchange rate",
    "Exchange rate is required when an instructed amount is supplied."
);
descriptor!(
    ACCOUNT_XOR,
    "ISO20022-L2-PACS008-ACCOUNT-ID-XOR",
    "Debtor account identification choice",
    "IBAN and other account identification are mutually exclusive."
);
descriptor!(
    COUNT,
    "ISO20022-L2-PACS008-TX-COUNT",
    "Declared transaction count",
    "Declared transaction count must equal the generated transaction collection length."
);
descriptor!(
    AMOUNT,
    "ISO20022-L2-PACS008-CURRENCY-AMOUNT",
    "Settlement currency and amount",
    "Settlement amount must satisfy the active currency's decimal precision."
);
descriptor!(
    AGENT_ACCOUNT,
    "ISO20022-L2-PACS008-DEBTOR-AGENT-ACCOUNT",
    "Debtor agent and account",
    "A debtor agent is required when a debtor account is supplied."
);

/// Schema-derived descriptor used to audit every generated-field projection in
/// this binding.
pub fn pacs_008_001_08_binding_descriptor() -> Option<&'static MessageDescriptor> {
    crate::catalogue::lookup_descriptor("pacs.008.001.08")
}

/// Validate selected cross-field relationships directly on the canonical
/// generated `pacs.008.001.08::Document` value.
pub fn validate_pacs_008_001_08(
    document: &Pacs008Document,
) -> Result<ValidationReport, DuplicateRuleId> {
    let message = &document.fi_to_fi_cstmr_cdt_trf;
    let count_rule = CardinalityDependencyRule::matches_declared(&COUNT, TX, TX_COUNT);
    let count_rules: [&dyn Rule; 1] = [&count_rule];
    let count_registry = RuleRegistry::new(&count_rules)?;
    let mut count_fields = vec![(TX_COUNT, message.grp_hdr.nb_of_txs.0.as_str())];
    count_fields.extend(message.cdt_trf_tx_inf.iter().map(|_| (TX, "present")));
    let mut report = count_registry.validate(
        &ValidationTarget::from_pairs(&count_fields),
        &ValidationContext {
            message_id: Some("pacs.008.001.08"),
            ..ValidationContext::default()
        },
    );

    let required = RequiredIfRule::new(
        &INSTD_XCHG,
        FieldPredicate::present(INSTD_AMOUNT),
        EXCHANGE_RATE,
    );
    let xor = MutualExclusionRule::new(&ACCOUNT_XOR, &[ACCOUNT_IBAN, ACCOUNT_OTHER]);
    let money = CurrencyAmountRule::new(&AMOUNT, SETTLEMENT_CURRENCY, SETTLEMENT_AMOUNT);
    let agent = AgentAccountRule::new(&AGENT_ACCOUNT, DEBTOR_ACCOUNT, DEBTOR_AGENT);
    let transaction_rules: [&dyn Rule; 4] = [&required, &xor, &money, &agent];
    let transaction_registry = RuleRegistry::new(&transaction_rules)?;
    for transaction in &message.cdt_trf_tx_inf {
        let target = transaction_target(transaction);
        report.extend(
            transaction_registry
                .validate(
                    &target,
                    &ValidationContext {
                        message_id: Some("pacs.008.001.08"),
                        ..ValidationContext::default()
                    },
                )
                .issues()
                .iter()
                .cloned(),
        );
    }
    report.sort();
    Ok(report)
}

const PACS009_TX: &str = "Document/FICdtTrf/CdtTrfTxInf";
const PACS009_TX_COUNT: &str = "Document/FICdtTrf/GrpHdr/NbOfTxs";
const PACS009_CURRENCY: &str = "Document/FICdtTrf/CdtTrfTxInf/IntrBkSttlmAmt/@Ccy";
const PACS009_AMOUNT: &str = "Document/FICdtTrf/CdtTrfTxInf/IntrBkSttlmAmt";

static PACS009_COUNT: RuleDescriptor = RuleDescriptor {
    id: RuleId::new("ISO20022-L2-PACS009-TX-COUNT"),
    layer: ValidationLayer::IsoSemantic,
    title: "Declared transaction count",
    reason: "Declared transaction count must equal the generated transaction collection length.",
    source: Some("pacs.009.001.08 schema logical-path binding"),
    profile: None,
    profile_version: None,
    affected_messages: &["pacs.009.001.08"],
};

static PACS009_MONEY: RuleDescriptor = RuleDescriptor {
    id: RuleId::new("ISO20022-L2-PACS009-CURRENCY-AMOUNT"),
    layer: ValidationLayer::IsoSemantic,
    title: "Settlement currency and amount",
    reason: "Settlement amount must satisfy the active currency's decimal precision.",
    source: Some("pacs.009.001.08 schema logical-path binding"),
    profile: None,
    profile_version: None,
    affected_messages: &["pacs.009.001.08"],
};

/// Validate selected semantic relationships directly on the canonical
/// generated `pacs.009.001.08::Document` value.
pub fn validate_pacs_009_001_08(
    document: &Pacs009Document,
) -> Result<ValidationReport, DuplicateRuleId> {
    let message = &document.fi_cdt_trf;
    let count_rule =
        CardinalityDependencyRule::matches_declared(&PACS009_COUNT, PACS009_TX, PACS009_TX_COUNT);
    let count_rules: [&dyn Rule; 1] = [&count_rule];
    let count_registry = RuleRegistry::new(&count_rules)?;
    let mut count_fields = vec![(PACS009_TX_COUNT, message.grp_hdr.nb_of_txs.0.as_str())];
    count_fields.extend(
        message
            .cdt_trf_tx_inf
            .iter()
            .map(|_| (PACS009_TX, "present")),
    );
    let context = ValidationContext {
        message_id: Some("pacs.009.001.08"),
        ..ValidationContext::default()
    };
    let mut report =
        count_registry.validate(&ValidationTarget::from_pairs(&count_fields), &context);

    let money = CurrencyAmountRule::new(&PACS009_MONEY, PACS009_CURRENCY, PACS009_AMOUNT);
    let transaction_rules: [&dyn Rule; 1] = [&money];
    let transaction_registry = RuleRegistry::new(&transaction_rules)?;
    for transaction in &message.cdt_trf_tx_inf {
        report.extend(
            transaction_registry
                .validate(&pacs009_transaction_target(transaction), &context)
                .issues()
                .iter()
                .cloned(),
        );
    }
    report.sort();
    Ok(report)
}

/// Validate an enabled parsed message through its exact generated value.
///
/// This first binding slice supports `pacs.008.001.08`; other identifiers are
/// returned as explicitly unavailable rather than treated as valid.
pub fn validate_parsed_message(
    message: &crate::ParsedMessage,
) -> Result<ValidationReport, MessageValidationError> {
    if let Some(document) = message.message().as_pacs_008_001_08() {
        return validate_pacs_008_001_08(document)
            .map_err(|_| MessageValidationError::RegistryConflict);
    }
    Err(MessageValidationError::UnsupportedMessage {
        message_id: message.message_id().as_str().to_owned(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageValidationError {
    UnsupportedMessage { message_id: String },
    RegistryConflict,
}

impl fmt::Display for MessageValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedMessage { message_id } => {
                write!(formatter, "validation unavailable for {message_id}")
            }
            Self::RegistryConflict => formatter.write_str("validation rule registry conflict"),
        }
    }
}

impl std::error::Error for MessageValidationError {}

fn transaction_target(transaction: &CreditTransferTransaction39) -> ValidationTarget<'_> {
    let mut fields = Vec::new();
    if !transaction.instd_amt.value.is_empty() {
        fields.push((INSTD_AMOUNT, transaction.instd_amt.value.as_str()));
    }
    if !transaction.xchg_rate.0.is_empty() {
        fields.push((EXCHANGE_RATE, transaction.xchg_rate.0.as_str()));
    }
    if let Some(iban) = &transaction.dbtr_acct.id.iban {
        fields.push((ACCOUNT_IBAN, iban.0.as_str()));
        fields.push((DEBTOR_ACCOUNT, iban.0.as_str()));
    }
    if let Some(other) = &transaction.dbtr_acct.id.othr {
        fields.push((ACCOUNT_OTHER, other.id.0.as_str()));
        fields.push((DEBTOR_ACCOUNT, other.id.0.as_str()));
    }
    fields.push((
        SETTLEMENT_CURRENCY,
        transaction.intr_bk_sttlm_amt.ccy.0.as_str(),
    ));
    fields.push((
        SETTLEMENT_AMOUNT,
        transaction.intr_bk_sttlm_amt.value.as_str(),
    ));
    let agent = &transaction.dbtr_agt.fin_instn_id;
    if !agent.bicfi.0.is_empty() {
        fields.push((DEBTOR_AGENT, agent.bicfi.0.as_str()));
    } else if !agent.nm.0.is_empty() {
        fields.push((DEBTOR_AGENT, agent.nm.0.as_str()));
    }
    ValidationTarget::from_pairs(&fields)
}

fn pacs009_transaction_target(transaction: &CreditTransferTransaction36) -> ValidationTarget<'_> {
    ValidationTarget::from_pairs(&[
        (
            PACS009_CURRENCY,
            transaction.intr_bk_sttlm_amt.ccy.0.as_str(),
        ),
        (PACS009_AMOUNT, transaction.intr_bk_sttlm_amt.value.as_str()),
    ])
}
