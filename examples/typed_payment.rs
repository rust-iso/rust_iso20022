//! Typed parse / access / serialize of a pacs.008 credit transfer, plus JSON and
//! a typed decimal amount. Requires the model for the `pacs` area:
//!
//! ```bash
//! cargo run --example typed_payment --features model-pacs,serde,convert
//! ```

use rust_iso20022::generated::pacs::pacs_008_001_08::Document;
use rust_iso20022::{from_xml, to_json, to_xml, MxMessage};

const XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
  <FIToFICstmrCdtTrf>
    <GrpHdr><MsgId>ABC-2026-0001</MsgId><NbOfTxs>1</NbOfTxs></GrpHdr>
    <CdtTrfTxInf>
      <PmtId><EndToEndId>E2E-REF-001</EndToEndId></PmtId>
      <IntrBkSttlmAmt Ccy="EUR">1234.56</IntrBkSttlmAmt>
    </CdtTrfTxInf>
  </FIToFICstmrCdtTrf>
</Document>"#;

fn main() {
    // Every Document carries its own identity.
    println!(
        "type: {} (area {})",
        Document::MESSAGE_NAME,
        Document::mx_id().business_area.code()
    );

    // Parse to a typed value and read fields directly.
    let doc: Document = from_xml(XML).expect("parse");
    let cdt = &doc.fi_to_fi_cstmr_cdt_trf;
    println!("MsgId        : {}", cdt.grp_hdr.msg_id.0);
    let tx = &cdt.cdt_trf_tx_inf[0];
    let amt = &tx.intr_bk_sttlm_amt;
    println!("Amount       : {} {}", amt.value, amt.ccy.0);

    // Scalars are exact strings; convert to a typed Decimal for arithmetic.
    let decimal = rust_iso20022::convert::to_decimal(&amt.value).expect("decimal");
    println!("As Decimal   : {decimal} (doubled = {})", decimal + decimal);

    // Serialize back to XML and confirm it round-trips.
    let xml = to_xml(&doc).expect("serialize");
    let back: Document = from_xml(&xml).expect("re-parse");
    println!("Round-trips  : {}", doc == back);

    // ...or to JSON (ISO 20022 element names).
    let json = to_json(&doc).expect("json");
    println!("JSON         : {}…", &json[..json.len().min(72)]);
}
