//! Builders and identifier helpers for first-phase cash-management reports.
//!
//! [`CamtReportId`] validates only an identifier used during construction. It
//! is consumed into a generated `Max35Text` and is not a cash-report model.

use super::{BuilderError, bounded_text, required};
use crate::generated::camt::{camt_052_001_09, camt_053_001_09, camt_054_001_09};

/// Validated Max35 identifier consumed by a camt report builder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CamtReportId(String);

impl CamtReportId {
    pub fn parse(value: &str) -> Result<Self, BuilderError> {
        bounded_text(value, "report_id")?;
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn into_inner(self) -> String {
        self.0
    }
}

/// Builder for canonical `camt.052.001.09::Document` values.
#[derive(Default)]
pub struct Camt052Builder {
    message_id: Option<String>,
    report_id: Option<CamtReportId>,
}

impl Camt052Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn report_id(mut self, value: CamtReportId) -> Self {
        self.report_id = Some(value);
        self
    }

    pub fn build(self) -> Result<camt_052_001_09::Document, BuilderError> {
        let message_id = required(self.message_id, "message_id")?;
        let report_id = required(self.report_id, "report_id")?;
        bounded_text(&message_id, "message_id")?;

        let mut document = camt_052_001_09::Document::default();
        document.bk_to_cstmr_acct_rpt.grp_hdr.msg_id = camt_052_001_09::Max35Text(message_id);
        document
            .bk_to_cstmr_acct_rpt
            .rpt
            .push(camt_052_001_09::AccountReport30 {
                id: camt_052_001_09::Max35Text(report_id.into_inner()),
                ..Default::default()
            });
        Ok(document)
    }
}

/// Builder for canonical `camt.053.001.09::Document` values.
#[derive(Default)]
pub struct Camt053Builder {
    message_id: Option<String>,
    statement_id: Option<CamtReportId>,
}

impl Camt053Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn statement_id(mut self, value: CamtReportId) -> Self {
        self.statement_id = Some(value);
        self
    }

    pub fn build(self) -> Result<camt_053_001_09::Document, BuilderError> {
        let message_id = required(self.message_id, "message_id")?;
        let statement_id = required(self.statement_id, "statement_id")?;
        bounded_text(&message_id, "message_id")?;

        let mut document = camt_053_001_09::Document::default();
        document.bk_to_cstmr_stmt.grp_hdr.msg_id = camt_053_001_09::Max35Text(message_id);
        document
            .bk_to_cstmr_stmt
            .stmt
            .push(camt_053_001_09::AccountStatement10 {
                id: camt_053_001_09::Max35Text(statement_id.into_inner()),
                ..Default::default()
            });
        Ok(document)
    }
}

/// Builder for canonical `camt.054.001.09::Document` values.
#[derive(Default)]
pub struct Camt054Builder {
    message_id: Option<String>,
    notification_id: Option<CamtReportId>,
}

impl Camt054Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn notification_id(mut self, value: CamtReportId) -> Self {
        self.notification_id = Some(value);
        self
    }

    pub fn build(self) -> Result<camt_054_001_09::Document, BuilderError> {
        let message_id = required(self.message_id, "message_id")?;
        let notification_id = required(self.notification_id, "notification_id")?;
        bounded_text(&message_id, "message_id")?;

        let mut document = camt_054_001_09::Document::default();
        document.bk_to_cstmr_dbt_cdt_ntfctn.grp_hdr.msg_id = camt_054_001_09::Max35Text(message_id);
        document
            .bk_to_cstmr_dbt_cdt_ntfctn
            .ntfctn
            .push(camt_054_001_09::AccountNotification19 {
                id: camt_054_001_09::Max35Text(notification_id.into_inner()),
                ..Default::default()
            });
        Ok(document)
    }
}
