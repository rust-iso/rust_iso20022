use super::{BuilderError, BuilderErrorKind, bounded_text, required};
use crate::generated::pacs::{pacs_002_001_10, pacs_008_001_08, pacs_009_001_08};
use crate::helpers::Money;

/// Builder for canonical `pacs.008.001.08::Document` values.
#[derive(Default)]
pub struct Pacs008Builder {
    message_id: Option<String>,
    transaction_id: Option<String>,
    settlement: Option<Money>,
}

impl Pacs008Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn transaction_id(mut self, value: impl Into<String>) -> Self {
        self.transaction_id = Some(value.into());
        self
    }

    pub fn settlement(mut self, value: Money) -> Self {
        self.settlement = Some(value);
        self
    }

    pub fn build(self) -> Result<pacs_008_001_08::Document, BuilderError> {
        let message_id = required(self.message_id, "message_id")?;
        let transaction_id = required(self.transaction_id, "transaction_id")?;
        let settlement = required(self.settlement, "settlement")?;
        bounded_text(&message_id, "message_id")?;
        bounded_text(&transaction_id, "transaction_id")?;
        let (currency, amount) = settlement.into_parts();

        let mut document = pacs_008_001_08::Document::default();
        document.fi_to_fi_cstmr_cdt_trf.grp_hdr.msg_id = pacs_008_001_08::Max35Text(message_id);
        document.fi_to_fi_cstmr_cdt_trf.grp_hdr.nb_of_txs =
            pacs_008_001_08::Max15NumericText("1".to_owned());
        let mut transaction = pacs_008_001_08::CreditTransferTransaction39::default();
        transaction.pmt_id.tx_id = pacs_008_001_08::Max35Text(transaction_id.clone());
        transaction.pmt_id.end_to_end_id = pacs_008_001_08::Max35Text(transaction_id);
        transaction.intr_bk_sttlm_amt = pacs_008_001_08::ActiveCurrencyAndAmount {
            value: amount,
            ccy: pacs_008_001_08::ActiveCurrencyCode(currency.as_str().to_owned()),
        };
        document
            .fi_to_fi_cstmr_cdt_trf
            .cdt_trf_tx_inf
            .push(transaction);

        let report =
            crate::validation::bindings::validate_pacs_008_001_08(&document).map_err(|_| {
                BuilderError::new(
                    "BUILDER-VALIDATION-REGISTRY",
                    "document",
                    BuilderErrorKind::Internal,
                )
            })?;
        if !report.issues().is_empty() {
            return Err(BuilderError::new(
                "BUILDER-VALIDATION",
                "document",
                BuilderErrorKind::Validation,
            ));
        }
        Ok(document)
    }
}

/// Builder for canonical `pacs.009.001.08::Document` values.
#[derive(Default)]
pub struct Pacs009Builder {
    message_id: Option<String>,
    transaction_id: Option<String>,
    settlement: Option<Money>,
}

impl Pacs009Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn transaction_id(mut self, value: impl Into<String>) -> Self {
        self.transaction_id = Some(value.into());
        self
    }

    pub fn settlement(mut self, value: Money) -> Self {
        self.settlement = Some(value);
        self
    }

    pub fn build(self) -> Result<pacs_009_001_08::Document, BuilderError> {
        let message_id = required(self.message_id, "message_id")?;
        let transaction_id = required(self.transaction_id, "transaction_id")?;
        let settlement = required(self.settlement, "settlement")?;
        bounded_text(&message_id, "message_id")?;
        bounded_text(&transaction_id, "transaction_id")?;
        let (currency, amount) = settlement.into_parts();

        let mut document = pacs_009_001_08::Document::default();
        document.fi_cdt_trf.grp_hdr.msg_id = pacs_009_001_08::Max35Text(message_id);
        document.fi_cdt_trf.grp_hdr.nb_of_txs = pacs_009_001_08::Max15NumericText("1".to_owned());
        let mut transaction = pacs_009_001_08::CreditTransferTransaction36::default();
        transaction.pmt_id.tx_id = pacs_009_001_08::Max35Text(transaction_id.clone());
        transaction.pmt_id.end_to_end_id = pacs_009_001_08::Max35Text(transaction_id);
        transaction.intr_bk_sttlm_amt = pacs_009_001_08::ActiveCurrencyAndAmount {
            value: amount,
            ccy: pacs_009_001_08::ActiveCurrencyCode(currency.as_str().to_owned()),
        };
        document.fi_cdt_trf.cdt_trf_tx_inf.push(transaction);
        let report =
            crate::validation::bindings::validate_pacs_009_001_08(&document).map_err(|_| {
                BuilderError::new(
                    "BUILDER-VALIDATION-REGISTRY",
                    "document",
                    BuilderErrorKind::Internal,
                )
            })?;
        if !report.issues().is_empty() {
            return Err(BuilderError::new(
                "BUILDER-VALIDATION",
                "document",
                BuilderErrorKind::Validation,
            ));
        }
        Ok(document)
    }
}

/// Builder for canonical `pacs.002.001.10::Document` values.
#[derive(Default)]
pub struct Pacs002Builder {
    message_id: Option<String>,
    original_message_id: Option<String>,
    original_message_name: Option<String>,
    group_status: Option<String>,
}

impl Pacs002Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn original_group(
        mut self,
        message_id: impl Into<String>,
        message_name: impl Into<String>,
        status: impl Into<String>,
    ) -> Self {
        self.original_message_id = Some(message_id.into());
        self.original_message_name = Some(message_name.into());
        self.group_status = Some(status.into());
        self
    }

    pub fn build(self) -> Result<pacs_002_001_10::Document, BuilderError> {
        let message_id = required(self.message_id, "message_id")?;
        let original_message_id = required(self.original_message_id, "original_message_id")?;
        let original_message_name = required(self.original_message_name, "original_message_name")?;
        let group_status = required(self.group_status, "group_status")?;
        bounded_text(&message_id, "message_id")?;
        bounded_text(&original_message_id, "original_message_id")?;
        bounded_text(&original_message_name, "original_message_name")?;
        if group_status.is_empty() || group_status.len() > 4 {
            return Err(BuilderError::new(
                "BUILDER-FIELD-LENGTH",
                "group_status",
                BuilderErrorKind::Constraint,
            ));
        }

        let mut document = pacs_002_001_10::Document::default();
        document.fi_to_fi_pmt_sts_rpt.grp_hdr.msg_id = pacs_002_001_10::Max35Text(message_id);
        let original = pacs_002_001_10::OriginalGroupHeader17 {
            orgnl_msg_id: pacs_002_001_10::Max35Text(original_message_id),
            orgnl_msg_nm_id: pacs_002_001_10::Max35Text(original_message_name),
            grp_sts: pacs_002_001_10::ExternalPaymentGroupStatus1Code(group_status),
            ..Default::default()
        };
        document
            .fi_to_fi_pmt_sts_rpt
            .orgnl_grp_inf_and_sts
            .push(original);
        Ok(document)
    }
}
