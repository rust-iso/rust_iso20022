//! Business Application Header extraction (no `model` feature needed).

use rust_iso20022::app_hdr::parse_business_header;

const MSG: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<Envelope>
  <AppHdr xmlns="urn:iso:std:iso:20022:tech:xsd:head.001.001.02">
    <Fr><FIId><FinInstnId><BICFI>BANKBEBB</BICFI></FinInstnId></FIId></Fr>
    <To><FIId><FinInstnId><BICFI>BANKDEFF</BICFI></FinInstnId></FIId></To>
    <BizMsgIdr>MSG-12345</BizMsgIdr>
    <MsgDefIdr>pacs.008.001.08</MsgDefIdr>
    <CreDt>2026-06-25T10:30:00Z</CreDt>
  </AppHdr>
  <Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
    <FIToFICstmrCdtTrf><GrpHdr><CreDt>2099-01-01T00:00:00Z</CreDt></GrpHdr></FIToFICstmrCdtTrf>
  </Document>
</Envelope>"#;

#[test]
fn reads_header_fields() {
    let h = parse_business_header(MSG).expect("header");
    assert_eq!(h.from.as_deref(), Some("BANKBEBB"));
    assert_eq!(h.to.as_deref(), Some("BANKDEFF"));
    assert_eq!(h.biz_msg_idr.as_deref(), Some("MSG-12345"));
    assert_eq!(h.msg_def_idr.as_deref(), Some("pacs.008.001.08"));
    // CreDt must come from the AppHdr, not the Document's GrpHdr.
    assert_eq!(h.cre_dt.as_deref(), Some("2026-06-25T10:30:00Z"));
}

#[test]
fn header_and_document_detect_independently() {
    // The message type detected from the whole envelope is the Document's.
    let id = rust_iso20022::detect(MSG).unwrap();
    assert_eq!(id.message_name(), "pacs.008.001.08");
}

#[test]
fn none_when_absent() {
    assert!(parse_business_header("<Document><Foo>x</Foo></Document>").is_none());
}
