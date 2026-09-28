use rust_iso20022::builders::{Camt052Builder, Camt053Builder, Camt054Builder, CamtReportId};
use rust_iso20022::generated::camt::{camt_052_001_09, camt_053_001_09, camt_054_001_09};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let report: camt_052_001_09::Document = Camt052Builder::new()
        .message_id("EXAMPLE-052")
        .report_id(CamtReportId::parse("REPORT-052")?)
        .build()?;
    assert_eq!(report.bk_to_cstmr_acct_rpt.rpt[0].id.0, "REPORT-052");

    let statement: camt_053_001_09::Document = Camt053Builder::new()
        .message_id("EXAMPLE-053")
        .statement_id(CamtReportId::parse("STATEMENT-053")?)
        .build()?;
    assert_eq!(statement.bk_to_cstmr_stmt.stmt[0].id.0, "STATEMENT-053");

    let notification: camt_054_001_09::Document = Camt054Builder::new()
        .message_id("EXAMPLE-054")
        .notification_id(CamtReportId::parse("NOTICE-054")?)
        .build()?;
    assert_eq!(
        notification.bk_to_cstmr_dbt_cdt_ntfctn.ntfctn[0].id.0,
        "NOTICE-054"
    );
    Ok(())
}
