//! Business metadata extraction (no `model` feature needed).

use rust_iso20022::metadata::extract;

const PACS008: &str = r#"<?xml version="1.0"?>
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
  <FIToFICstmrCdtTrf>
    <GrpHdr>
      <MsgId>MSG-2026-0001</MsgId>
      <CreDtTm>2026-06-25T10:00:00Z</CreDtTm>
      <NbOfTxs>2</NbOfTxs>
      <TtlIntrBkSttlmAmt Ccy="EUR">5000.00</TtlIntrBkSttlmAmt>
    </GrpHdr>
    <CdtTrfTxInf>
      <IntrBkSttlmAmt Ccy="EUR">1234.56</IntrBkSttlmAmt>
      <IntrBkSttlmDt>2026-06-26</IntrBkSttlmDt>
    </CdtTrfTxInf>
  </FIToFICstmrCdtTrf>
</Document>"#;

#[test]
fn extracts_payment_metadata() {
    let m = extract(PACS008);
    assert_eq!(m.message_id.as_deref(), Some("MSG-2026-0001"));
    assert_eq!(m.creation_date_time.as_deref(), Some("2026-06-25T10:00:00Z"));
    assert_eq!(m.number_of_transactions.as_deref(), Some("2"));
    // The total settlement amount is preferred over the per-tx amount.
    assert_eq!(m.amount.as_deref(), Some("5000.00"));
    assert_eq!(m.currency.as_deref(), Some("EUR"));
    assert_eq!(m.value_date.as_deref(), Some("2026-06-26"));
}

#[test]
fn empty_for_unknown_content() {
    let m = extract("<Document><Foo>x</Foo></Document>");
    assert_eq!(m, Default::default());
}
