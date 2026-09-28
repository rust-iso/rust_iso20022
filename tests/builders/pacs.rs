use rust_iso20022::builders::{Pacs002Builder, Pacs008Builder, Pacs009Builder};
use rust_iso20022::generated::pacs::{pacs_002_001_10, pacs_008_001_08, pacs_009_001_08};
use rust_iso20022::helpers::Money;
use rust_iso20022::validation::bindings::validate_pacs_008_001_08;
use rust_iso20022::{from_xml, to_xml};

const VALID_FIXTURE: &str = include_str!("../../fixtures/iso/builders/pacs/valid.json");
const INVALID_FIXTURE: &str = include_str!("../../fixtures/iso/builders/pacs/invalid.json");

#[test]
fn builders_return_exact_generated_document_types() {
    let pacs008: pacs_008_001_08::Document = Pacs008Builder::new()
        .message_id("MSG-008")
        .transaction_id("TX-008")
        .settlement(Money::parse("EUR", "125.50").unwrap())
        .build()
        .unwrap();
    assert_eq!(pacs008.fi_to_fi_cstmr_cdt_trf.grp_hdr.msg_id.0, "MSG-008");
    assert_eq!(
        pacs008.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf[0]
            .intr_bk_sttlm_amt
            .value,
        "125.50"
    );
    assert!(
        validate_pacs_008_001_08(&pacs008)
            .unwrap()
            .issues()
            .is_empty()
    );

    let pacs009: pacs_009_001_08::Document = Pacs009Builder::new()
        .message_id("MSG-009")
        .transaction_id("TX-009")
        .settlement(Money::parse("USD", "250.00").unwrap())
        .build()
        .unwrap();
    assert_eq!(pacs009.fi_cdt_trf.grp_hdr.msg_id.0, "MSG-009");
    assert_eq!(
        pacs009.fi_cdt_trf.cdt_trf_tx_inf[0].pmt_id.tx_id.0,
        "TX-009"
    );

    let pacs002: pacs_002_001_10::Document = Pacs002Builder::new()
        .message_id("MSG-002")
        .original_group("ORIGINAL-1", "pacs.008.001.08", "ACCP")
        .build()
        .unwrap();
    assert_eq!(pacs002.fi_to_fi_pmt_sts_rpt.grp_hdr.msg_id.0, "MSG-002");
    assert_eq!(
        pacs002.fi_to_fi_pmt_sts_rpt.orgnl_grp_inf_and_sts[0]
            .orgnl_msg_id
            .0,
        "ORIGINAL-1"
    );
}

#[test]
fn missing_and_boundary_inputs_return_typed_builder_errors() {
    let missing = Pacs008Builder::new().build().unwrap_err();
    assert_eq!(missing.field(), "message_id");
    assert_eq!(missing.code(), "BUILDER-MISSING-FIELD");

    let too_long = Pacs009Builder::new()
        .message_id("X".repeat(36))
        .transaction_id("TX")
        .settlement(Money::parse("EUR", "1.00").unwrap())
        .build()
        .unwrap_err();
    assert_eq!(too_long.field(), "message_id");
    assert_eq!(too_long.code(), "BUILDER-FIELD-LENGTH");

    let missing_original = Pacs002Builder::new().message_id("MSG").build().unwrap_err();
    assert_eq!(missing_original.field(), "original_message_id");
}

#[test]
fn built_documents_round_trip_xml_as_the_same_generated_types() {
    let pacs008 = Pacs008Builder::new()
        .message_id("ROUNDTRIP-008")
        .transaction_id("TX-008")
        .settlement(Money::parse("EUR", "1.00").unwrap())
        .build()
        .unwrap();
    let xml = to_xml(&pacs008).unwrap();
    let parsed: pacs_008_001_08::Document = from_xml(&xml).unwrap();
    assert_eq!(
        parsed.fi_to_fi_cstmr_cdt_trf.grp_hdr.msg_id.0,
        "ROUNDTRIP-008"
    );

    let pacs009 = Pacs009Builder::new()
        .message_id("ROUNDTRIP-009")
        .transaction_id("TX-009")
        .settlement(Money::parse("USD", "2.00").unwrap())
        .build()
        .unwrap();
    let xml = to_xml(&pacs009).unwrap();
    let parsed: pacs_009_001_08::Document = from_xml(&xml).unwrap();
    assert_eq!(parsed.fi_cdt_trf.grp_hdr.msg_id.0, "ROUNDTRIP-009");

    let pacs002 = Pacs002Builder::new()
        .message_id("ROUNDTRIP-002")
        .original_group("ORIGINAL", "pacs.008.001.08", "ACCP")
        .build()
        .unwrap();
    let xml = to_xml(&pacs002).unwrap();
    let parsed: pacs_002_001_10::Document = from_xml(&xml).unwrap();
    assert_eq!(
        parsed.fi_to_fi_pmt_sts_rpt.grp_hdr.msg_id.0,
        "ROUNDTRIP-002"
    );
}

#[test]
fn builder_fixtures_pin_inputs_output_versions_and_typed_error_paths() {
    let valid: serde_json::Value = serde_json::from_str(VALID_FIXTURE).unwrap();
    let input = &valid["pacs008"];
    let document = Pacs008Builder::new()
        .message_id(input["message_id"].as_str().unwrap())
        .transaction_id(input["transaction_id"].as_str().unwrap())
        .settlement(
            Money::parse(
                input["currency"].as_str().unwrap(),
                input["amount"].as_str().unwrap(),
            )
            .unwrap(),
        )
        .build()
        .unwrap();
    assert_eq!(
        document.fi_to_fi_cstmr_cdt_trf.grp_hdr.msg_id.0,
        "FIXTURE-008"
    );
    assert_eq!(input["output_type"], "pacs.008.001.08");

    let invalid: serde_json::Value = serde_json::from_str(INVALID_FIXTURE).unwrap();
    let expected = &invalid["cases"][0];
    let error = Pacs008Builder::new().build().unwrap_err();
    assert_eq!(error.code(), expected["expected_code"].as_str().unwrap());
    assert_eq!(error.field(), expected["expected_field"].as_str().unwrap());
}
