//! Observable-behavior fixtures for the additive compatibility baseline.

use rust_iso20022::{catalogue, detect, metadata};

const MESSAGE: &str = r#"<?xml version="1.0"?>
<Envelope>
  <AppHdr xmlns="urn:iso:std:iso:20022:tech:xsd:head.001.001.02"/>
  <Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
    <FIToFICstmrCdtTrf>
      <GrpHdr>
        <MsgId>BASELINE-MSG-001</MsgId>
        <TtlIntrBkSttlmAmt Ccy="EUR">125.00</TtlIntrBkSttlmAmt>
      </GrpHdr>
      <CdtTrfTxInf><IntrBkSttlmDt>2026-09-27</IntrBkSttlmDt></CdtTrfTxInf>
    </FIToFICstmrCdtTrf>
  </Document>
</Envelope>"#;

#[test]
fn core_detection_catalogue_and_metadata_match_golden() {
    let entry = catalogue::from_message_name("pacs.008.001.08").unwrap();
    let id = detect(MESSAGE).unwrap();
    let metadata = metadata::extract(MESSAGE);
    let actual = serde_json::json!({
        "catalogue_len": catalogue::all().len(),
        "pacs_008_namespace": entry.namespace,
        "pacs_008_business_area": entry.business_area,
        "pacs_008_has_model": entry.has_model,
        "detected_message": id.message_name(),
        "detected_area": id.business_area.code(),
        "message_id": metadata.message_id,
        "amount": metadata.amount,
        "currency": metadata.currency,
        "value_date": metadata.value_date,
    });
    let expected: serde_json::Value =
        serde_json::from_str(include_str!("compatibility/fixtures/core-behavior.json")).unwrap();
    assert_eq!(actual, expected);
}

#[cfg(feature = "serde")]
#[test]
fn serde_wire_shapes_match_golden() {
    let id = rust_iso20022::MxId::parse("pacs.008.001.08").unwrap();
    let entry = catalogue::from_message_name("pacs.008.001.08").unwrap();
    let actual = serde_json::json!({
        "mx_id": serde_json::to_string(&id).unwrap(),
        "business_area": serde_json::to_string(&id.business_area).unwrap(),
        "catalogue_entry": serde_json::to_string(entry).unwrap(),
    });
    let expected: serde_json::Value =
        serde_json::from_str(include_str!("compatibility/fixtures/serde-wire.json")).unwrap();
    assert_eq!(actual, expected);
}

#[cfg(feature = "model-pacs")]
#[test]
fn representative_generated_xml_matches_golden() {
    use rust_iso20022::generated::pacs::pacs_002_001_10::Document;

    let xml = rust_iso20022::to_xml(&Document::default()).unwrap();
    let fnv1a64 = xml
        .as_bytes()
        .iter()
        .fold(0xcbf29ce484222325_u64, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
        });
    let expected: serde_json::Value = serde_json::from_str(include_str!(
        "compatibility/fixtures/pacs.002.001.10.default-wire.json"
    ))
    .unwrap();
    let prefix = expected["prefix"].as_str().unwrap();
    let suffix = expected["suffix"].as_str().unwrap();
    let actual = serde_json::json!({
        "bytes": xml.len(),
        "fnv1a64": format!("{fnv1a64:016x}"),
        "prefix": prefix,
        "suffix": suffix,
    });
    assert!(xml.starts_with(prefix));
    assert!(xml.ends_with(suffix));
    assert_eq!(actual, expected);
}
