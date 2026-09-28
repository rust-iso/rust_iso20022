//! Builders for the first-phase customer payment initiation and status models.

use super::{BuilderError, BuilderErrorKind, bounded_text, required};
use crate::generated::pain::{pain_001_001_09, pain_002_001_10};
use crate::helpers::Money;

/// Builder for canonical `pain.001.001.09::Document` values.
#[derive(Default)]
pub struct Pain001Builder {
    message_id: Option<String>,
    payment_info_id: Option<String>,
    end_to_end_id: Option<String>,
    instructed_amount: Option<Money>,
}

impl Pain001Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn payment_info_id(mut self, value: impl Into<String>) -> Self {
        self.payment_info_id = Some(value.into());
        self
    }

    pub fn end_to_end_id(mut self, value: impl Into<String>) -> Self {
        self.end_to_end_id = Some(value.into());
        self
    }

    pub fn instructed_amount(mut self, value: Money) -> Self {
        self.instructed_amount = Some(value);
        self
    }

    pub fn build(self) -> Result<pain_001_001_09::Document, BuilderError> {
        let message_id = required(self.message_id, "message_id")?;
        let payment_info_id = required(self.payment_info_id, "payment_info_id")?;
        let end_to_end_id = required(self.end_to_end_id, "end_to_end_id")?;
        let instructed_amount = required(self.instructed_amount, "instructed_amount")?;
        bounded_text(&message_id, "message_id")?;
        bounded_text(&payment_info_id, "payment_info_id")?;
        bounded_text(&end_to_end_id, "end_to_end_id")?;
        let (currency, amount) = instructed_amount.into_parts();

        let mut document = pain_001_001_09::Document::default();
        document.cstmr_cdt_trf_initn.grp_hdr.msg_id = pain_001_001_09::Max35Text(message_id);
        document.cstmr_cdt_trf_initn.grp_hdr.nb_of_txs =
            pain_001_001_09::Max15NumericText("1".to_owned());

        let mut transaction = pain_001_001_09::CreditTransferTransaction34::default();
        transaction.pmt_id.end_to_end_id = pain_001_001_09::Max35Text(end_to_end_id);
        transaction.amt.instd_amt = Some(pain_001_001_09::ActiveOrHistoricCurrencyAndAmount {
            value: amount,
            ccy: pain_001_001_09::ActiveOrHistoricCurrencyCode(currency.as_str().to_owned()),
        });

        let mut payment = pain_001_001_09::PaymentInstruction30 {
            pmt_inf_id: pain_001_001_09::Max35Text(payment_info_id),
            pmt_mtd: pain_001_001_09::PaymentMethod3Code::Trf,
            nb_of_txs: pain_001_001_09::Max15NumericText("1".to_owned()),
            ..Default::default()
        };
        payment.cdt_trf_tx_inf.push(transaction);
        document.cstmr_cdt_trf_initn.pmt_inf.push(payment);
        Ok(document)
    }
}

/// Builder for canonical `pain.002.001.10::Document` values.
#[derive(Default)]
pub struct Pain002Builder {
    message_id: Option<String>,
    original_message_id: Option<String>,
    original_message_name: Option<String>,
    group_status: Option<String>,
}

impl Pain002Builder {
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

    pub fn build(self) -> Result<pain_002_001_10::Document, BuilderError> {
        let message_id = required(self.message_id, "message_id")?;
        let original_message_id = required(self.original_message_id, "original_message_id")?;
        let original_message_name = required(self.original_message_name, "original_message_name")?;
        let group_status = required(self.group_status, "group_status")?;
        bounded_text(&message_id, "message_id")?;
        bounded_text(&original_message_id, "original_message_id")?;
        bounded_text(&original_message_name, "original_message_name")?;
        if group_status.is_empty()
            || group_status.len() > 4
            || group_status.chars().any(char::is_control)
        {
            return Err(BuilderError::new(
                "BUILDER-FIELD-LENGTH",
                "group_status",
                BuilderErrorKind::Constraint,
            ));
        }

        let mut document = pain_002_001_10::Document::default();
        document.cstmr_pmt_sts_rpt.grp_hdr.msg_id = pain_002_001_10::Max35Text(message_id);
        document.cstmr_pmt_sts_rpt.orgnl_grp_inf_and_sts = pain_002_001_10::OriginalGroupHeader17 {
            orgnl_msg_id: pain_002_001_10::Max35Text(original_message_id),
            orgnl_msg_nm_id: pain_002_001_10::Max35Text(original_message_name),
            grp_sts: pain_002_001_10::ExternalPaymentGroupStatus1Code(group_status),
            ..Default::default()
        };
        Ok(document)
    }
}
