use rust_iso20022::builders::{Camt052Builder, Camt053Builder, Camt054Builder, CamtReportId};
use rust_iso20022::generated::camt::{camt_052_001_09, camt_053_001_09, camt_054_001_09};
use rust_iso20022::validate::Validate;
use rust_iso20022::{from_xml, to_xml};

const VALID_FIXTURE: &str = include_str!("../../fixtures/iso/builders/camt/valid.json");
const INVALID_FIXTURE: &str = include_str!("../../fixtures/iso/builders/camt/invalid.json");

#[test]
fn builders_return_exact_generated_document_types() {
    let report_id = CamtReportId::parse("REPORT-052").unwrap();
    let report: camt_052_001_09::Document = Camt052Builder::new()
        .message_id("MSG-052")
        .report_id(report_id)
        .build()
        .unwrap();
    assert_eq!(report.bk_to_cstmr_acct_rpt.grp_hdr.msg_id.0, "MSG-052");
    assert_eq!(report.bk_to_cstmr_acct_rpt.rpt[0].id.0, "REPORT-052");

    let statement: camt_053_001_09::Document = Camt053Builder::new()
        .message_id("MSG-053")
        .statement_id(CamtReportId::parse("STATEMENT-053").unwrap())
        .build()
        .unwrap();
    assert_eq!(statement.bk_to_cstmr_stmt.grp_hdr.msg_id.0, "MSG-053");
    assert_eq!(statement.bk_to_cstmr_stmt.stmt[0].id.0, "STATEMENT-053");

    let notification: camt_054_001_09::Document = Camt054Builder::new()
        .message_id("MSG-054")
        .notification_id(CamtReportId::parse("NOTICE-054").unwrap())
        .build()
        .unwrap();
    assert_eq!(
        notification.bk_to_cstmr_dbt_cdt_ntfctn.grp_hdr.msg_id.0,
        "MSG-054"
    );
    assert_eq!(
        notification.bk_to_cstmr_dbt_cdt_ntfctn.ntfctn[0].id.0,
        "NOTICE-054"
    );
}

#[test]
fn missing_invalid_and_boundary_inputs_return_typed_errors() {
    let missing = Camt052Builder::new().build().unwrap_err();
    assert_eq!(missing.code(), "BUILDER-MISSING-FIELD");
    assert_eq!(missing.field(), "message_id");

    let too_long = CamtReportId::parse(&"X".repeat(36)).unwrap_err();
    assert_eq!(too_long.code(), "BUILDER-FIELD-LENGTH");
    assert_eq!(too_long.field(), "report_id");

    let invalid = Camt054Builder::new()
        .message_id("\u{7}")
        .notification_id(CamtReportId::parse("NOTICE").unwrap())
        .build()
        .unwrap_err();
    assert_eq!(invalid.code(), "BUILDER-FIELD-CHARSET");
    assert_eq!(invalid.field(), "message_id");
}

#[test]
fn built_documents_round_trip_and_use_generated_validation() {
    let report = Camt052Builder::new()
        .message_id("ROUNDTRIP-052")
        .report_id(CamtReportId::parse("REPORT-052").unwrap())
        .build()
        .unwrap();
    assert_eq!(report.validate(), Ok(()));
    let parsed: camt_052_001_09::Document = from_xml(&to_xml(&report).unwrap()).unwrap();
    assert_eq!(parsed.bk_to_cstmr_acct_rpt.rpt[0].id.0, "REPORT-052");

    let statement = Camt053Builder::new()
        .message_id("ROUNDTRIP-053")
        .statement_id(CamtReportId::parse("STATEMENT-053").unwrap())
        .build()
        .unwrap();
    assert_eq!(statement.validate(), Ok(()));
    let parsed: camt_053_001_09::Document = from_xml(&to_xml(&statement).unwrap()).unwrap();
    assert_eq!(parsed.bk_to_cstmr_stmt.stmt[0].id.0, "STATEMENT-053");

    let notification = Camt054Builder::new()
        .message_id("ROUNDTRIP-054")
        .notification_id(CamtReportId::parse("NOTICE-054").unwrap())
        .build()
        .unwrap();
    assert_eq!(notification.validate(), Ok(()));
    let parsed: camt_054_001_09::Document = from_xml(&to_xml(&notification).unwrap()).unwrap();
    assert_eq!(
        parsed.bk_to_cstmr_dbt_cdt_ntfctn.ntfctn[0].id.0,
        "NOTICE-054"
    );
}

#[test]
fn fixtures_pin_inputs_versions_and_typed_error_paths() {
    let valid: serde_json::Value = serde_json::from_str(VALID_FIXTURE).unwrap();
    let input = &valid["camt052"];
    let report = Camt052Builder::new()
        .message_id(input["message_id"].as_str().unwrap())
        .report_id(CamtReportId::parse(input["report_id"].as_str().unwrap()).unwrap())
        .build()
        .unwrap();
    assert_eq!(report.bk_to_cstmr_acct_rpt.grp_hdr.msg_id.0, "FIXTURE-052");
    assert_eq!(input["output_type"], "camt.052.001.09");

    let input = &valid["camt053"];
    let document = Camt053Builder::new()
        .message_id(input["message_id"].as_str().unwrap())
        .statement_id(CamtReportId::parse(input["statement_id"].as_str().unwrap()).unwrap())
        .build()
        .unwrap();
    assert_eq!(document.bk_to_cstmr_stmt.grp_hdr.msg_id.0, "FIXTURE-053");
    assert_eq!(input["output_type"], "camt.053.001.09");

    let input = &valid["camt054"];
    let notification = Camt054Builder::new()
        .message_id(input["message_id"].as_str().unwrap())
        .notification_id(CamtReportId::parse(input["notification_id"].as_str().unwrap()).unwrap())
        .build()
        .unwrap();
    assert_eq!(
        notification.bk_to_cstmr_dbt_cdt_ntfctn.grp_hdr.msg_id.0,
        "FIXTURE-054"
    );
    assert_eq!(input["output_type"], "camt.054.001.09");

    let invalid: serde_json::Value = serde_json::from_str(INVALID_FIXTURE).unwrap();
    let expected = &invalid["cases"][0];
    let error = Camt052Builder::new().build().unwrap_err();
    assert_eq!(error.code(), expected["expected_code"].as_str().unwrap());
    assert_eq!(error.field(), expected["expected_field"].as_str().unwrap());
}
