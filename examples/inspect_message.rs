//! Inspect an incoming ISO 20022 business message — a realistic "what did I just
//! receive?" flow that needs **no `model` feature**: identify the message, read
//! its Business Application Header, extract business metadata, and pull specific
//! fields out of the generic tree.
//!
//! Run it:
//! ```bash
//! cargo run --example inspect_message
//! ```

use rust_iso20022::{detect, read_business_message, MxNode};

/// A pacs.008 (FI-to-FI customer credit transfer) wrapped in an AppHdr envelope,
/// the way it would arrive on the wire.
const MESSAGE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<BizMsgEnvlp>
  <AppHdr xmlns="urn:iso:std:iso:20022:tech:xsd:head.001.001.02">
    <Fr><FIId><FinInstnId><BICFI>BANKBEBB</BICFI></FinInstnId></FIId></Fr>
    <To><FIId><FinInstnId><BICFI>BANKDEFF</BICFI></FinInstnId></FIId></To>
    <BizMsgIdr>MSG-2026-0001</BizMsgIdr>
    <MsgDefIdr>pacs.008.001.08</MsgDefIdr>
    <CreDt>2026-06-27T10:30:00Z</CreDt>
  </AppHdr>
  <Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
    <FIToFICstmrCdtTrf>
      <GrpHdr>
        <MsgId>ABC-2026-0001</MsgId>
        <CreDtTm>2026-06-27T10:30:00Z</CreDtTm>
        <NbOfTxs>1</NbOfTxs>
        <SttlmInf><SttlmMtd>INDA</SttlmMtd></SttlmInf>
      </GrpHdr>
      <CdtTrfTxInf>
        <PmtId><EndToEndId>E2E-REF-001</EndToEndId></PmtId>
        <IntrBkSttlmAmt Ccy="EUR">1234.56</IntrBkSttlmAmt>
        <IntrBkSttlmDt>2026-06-30</IntrBkSttlmDt>
        <Dbtr><Nm>Alice Inc.</Nm></Dbtr>
        <Cdtr><Nm>Bob Ltd.</Nm></Cdtr>
      </CdtTrfTxInf>
    </FIToFICstmrCdtTrf>
  </Document>
</BizMsgEnvlp>"#;

fn main() {
    // 1. What message is this? (from the Document namespace)
    let id = detect(MESSAGE).expect("a recognisable ISO 20022 message");
    println!("Message type : {} ({})", id.message_name(), id.business_area.description());

    // 2. Header + identification + business metadata in one call.
    let bm = read_business_message(MESSAGE);
    if let Some(h) = &bm.header {
        println!("From / To    : {} -> {}", opt(&h.from), opt(&h.to));
        println!("Biz Msg Id   : {}", opt(&h.biz_msg_idr));
    }
    let m = &bm.metadata;
    println!("Group Msg Id : {}", opt(&m.message_id));
    println!(
        "Amount       : {} {}",
        opt(&m.amount),
        opt(&m.currency)
    );
    println!("Value date   : {}", opt(&m.value_date));

    // 3. Pull specific fields from the generic tree (no typed model needed).
    let doc = MxNode::parse(MESSAGE).expect("parseable tree");
    let e2e = doc.find("EndToEndId").and_then(|n| n.text()).unwrap_or("-");
    let dbtr = doc.find("Dbtr").and_then(|n| n.get("Nm")).and_then(|n| n.text()).unwrap_or("-");
    let cdtr = doc.find("Cdtr").and_then(|n| n.get("Nm")).and_then(|n| n.text()).unwrap_or("-");
    println!("End-to-end   : {e2e}");
    println!("Debtor       : {dbtr}");
    println!("Creditor     : {cdtr}");

    // The amount element with its `Ccy` attribute, straight from the tree:
    if let Some(amt) = doc.find("IntrBkSttlmAmt") {
        println!(
            "Settlement   : {} {}",
            amt.text().unwrap_or("-"),
            amt.attr("Ccy").unwrap_or("-")
        );
    }
}

fn opt(v: &Option<String>) -> &str {
    v.as_deref().unwrap_or("-")
}
