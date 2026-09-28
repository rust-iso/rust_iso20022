use rust_iso20022::builders::{Pain001Builder, Pain002Builder};
use rust_iso20022::generated::pain::{pain_001_001_09, pain_002_001_10};
use rust_iso20022::helpers::Money;
use rust_iso20022::{from_xml, to_xml};

const VALID_FIXTURE: &str = include_str!("../../fixtures/iso/builders/pain/valid.json");
const INVALID_FIXTURE: &str = include_str!("../../fixtures/iso/builders/pain/invalid.json");

#[test]
fn builders_return_exact_generated_document_types() {
    let initiation: pain_001_001_09::Document = Pain001Builder::new()
        .message_id("MSG-001")
        .payment_info_id("PMT-INFO-001")
        .end_to_end_id("E2E-001")
        .instructed_amount(Money::parse("EUR", "125.50").unwrap())
        .build()
        .unwrap();
    let payment = &initiation.cstmr_cdt_trf_initn.pmt_inf[0];
    assert_eq!(initiation.cstmr_cdt_trf_initn.grp_hdr.msg_id.0, "MSG-001");
    assert_eq!(payment.pmt_inf_id.0, "PMT-INFO-001");
    assert_eq!(payment.cdt_trf_tx_inf[0].pmt_id.end_to_end_id.0, "E2E-001");
    assert_eq!(
        payment.cdt_trf_tx_inf[0]
            .amt
            .instd_amt
            .as_ref()
            .unwrap()
            .value,
        "125.50"
    );

    let status: pain_002_001_10::Document = Pain002Builder::new()
        .message_id("MSG-002")
        .original_group("ORIGINAL-001", "pain.001.001.09", "ACCP")
        .build()
        .unwrap();
    assert_eq!(status.cstmr_pmt_sts_rpt.grp_hdr.msg_id.0, "MSG-002");
    assert_eq!(
        status
            .cstmr_pmt_sts_rpt
            .orgnl_grp_inf_and_sts
            .orgnl_msg_id
            .0,
        "ORIGINAL-001"
    );
}

#[test]
fn missing_invalid_and_boundary_inputs_return_typed_errors() {
    let missing = Pain001Builder::new().build().unwrap_err();
    assert_eq!(missing.code(), "BUILDER-MISSING-FIELD");
    assert_eq!(missing.field(), "message_id");

    let boundary = Pain001Builder::new()
        .message_id("X".repeat(36))
        .payment_info_id("PMT")
        .end_to_end_id("E2E")
        .instructed_amount(Money::parse("EUR", "1.00").unwrap())
        .build()
        .unwrap_err();
    assert_eq!(boundary.code(), "BUILDER-FIELD-LENGTH");
    assert_eq!(boundary.field(), "message_id");

    let invalid_status = Pain002Builder::new()
        .message_id("MSG")
        .original_group("ORIGINAL", "pain.001.001.09", "TOO-LONG")
        .build()
        .unwrap_err();
    assert_eq!(invalid_status.code(), "BUILDER-FIELD-LENGTH");
    assert_eq!(invalid_status.field(), "group_status");
}

#[test]
fn built_documents_round_trip_xml_as_the_same_generated_types() {
    let initiation = Pain001Builder::new()
        .message_id("ROUNDTRIP-001")
        .payment_info_id("PMT-001")
        .end_to_end_id("E2E-001")
        .instructed_amount(Money::parse("EUR", "1.00").unwrap())
        .build()
        .unwrap();
    let xml = to_xml(&initiation).unwrap();
    let parsed: pain_001_001_09::Document = from_xml(&xml).unwrap();
    assert_eq!(parsed.cstmr_cdt_trf_initn.grp_hdr.msg_id.0, "ROUNDTRIP-001");

    let status = Pain002Builder::new()
        .message_id("ROUNDTRIP-002")
        .original_group("ROUNDTRIP-001", "pain.001.001.09", "ACCP")
        .build()
        .unwrap();
    let xml = to_xml(&status).unwrap();
    let parsed: pain_002_001_10::Document = from_xml(&xml).unwrap();
    assert_eq!(parsed.cstmr_pmt_sts_rpt.grp_hdr.msg_id.0, "ROUNDTRIP-002");
}

#[test]
fn fixtures_pin_inputs_versions_and_typed_error_paths() {
    let valid: serde_json::Value = serde_json::from_str(VALID_FIXTURE).unwrap();
    let input = &valid["pain001"];
    let document = Pain001Builder::new()
        .message_id(input["message_id"].as_str().unwrap())
        .payment_info_id(input["payment_info_id"].as_str().unwrap())
        .end_to_end_id(input["end_to_end_id"].as_str().unwrap())
        .instructed_amount(
            Money::parse(
                input["currency"].as_str().unwrap(),
                input["amount"].as_str().unwrap(),
            )
            .unwrap(),
        )
        .build()
        .unwrap();
    assert_eq!(document.cstmr_cdt_trf_initn.grp_hdr.msg_id.0, "FIXTURE-001");
    assert_eq!(input["output_type"], "pain.001.001.09");

    let input = &valid["pain002"];
    let status = Pain002Builder::new()
        .message_id(input["message_id"].as_str().unwrap())
        .original_group(
            input["original_message_id"].as_str().unwrap(),
            input["original_message_name"].as_str().unwrap(),
            input["group_status"].as_str().unwrap(),
        )
        .build()
        .unwrap();
    assert_eq!(status.cstmr_pmt_sts_rpt.grp_hdr.msg_id.0, "FIXTURE-002");
    assert_eq!(input["output_type"], "pain.002.001.10");

    let invalid: serde_json::Value = serde_json::from_str(INVALID_FIXTURE).unwrap();
    let expected = &invalid["cases"][0];
    let error = Pain001Builder::new().build().unwrap_err();
    assert_eq!(error.code(), expected["expected_code"].as_str().unwrap());
    assert_eq!(error.field(), expected["expected_field"].as_str().unwrap());
}
