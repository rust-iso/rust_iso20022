---
title: "Processing ISO 20022 payment XML in Rust: detection, typed parsing and JSON"
published: false
tags: rust, xml, opensource
description: "A runnable ISO 20022 example using a model-free Rust core and opt-in typed payment models."
---

ISO 20022 payment messages are versioned XML documents. In an integration service, the first task may be to identify a message and read a few business fields. A later stage may need typed field access, arithmetic on an amount, and serialization for another system.

This walkthrough uses **rust_iso20022 0.1.6**, an Apache-2.0 toolkit with a Rust 1.85 core MSRV. Its generated model collection covers 1,130 message versions across 32 business areas, including earlier versions. We'll use just one message version, `pacs.008.001.08`.

The XML below is a **synthetic, intentionally minimal teaching sample**. It demonstrates the SDK's parsing APIs; it is not a complete schema-valid or scheme-approved payment instruction. None of the examples performs complete XSD or banking-scheme validation.

Start with the model-free core. Create a Rust application and add the published version:

```bash
cargo new iso20022-demo
cd iso20022-demo
cargo add rust_iso20022@0.1.6
```

Replace `src/main.rs` with this complete program:

```rust
use rust_iso20022::{detect, MxNode};

const XML: &str = r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
  <FIToFICstmrCdtTrf>
    <GrpHdr><MsgId>DEMO-001</MsgId><NbOfTxs>1</NbOfTxs></GrpHdr>
    <CdtTrfTxInf>
      <PmtId><EndToEndId>E2E-DEMO-001</EndToEndId></PmtId>
      <IntrBkSttlmAmt Ccy="EUR">1234.56</IntrBkSttlmAmt>
    </CdtTrfTxInf>
  </FIToFICstmrCdtTrf>
</Document>"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id = detect(XML).ok_or("unrecognized ISO 20022 message")?;
    println!("message: {}", id.message_name());

    let tree = MxNode::parse(XML).ok_or("invalid XML")?;
    let message_id = tree.find("MsgId").and_then(|node| node.text());
    println!("message_id: {}", message_id.unwrap_or(""));

    let amount = tree.find("IntrBkSttlmAmt").ok_or("missing amount")?;
    println!(
        "amount: {} {}",
        amount.text().unwrap_or(""),
        amount.attr("Ccy").unwrap_or("")
    );
    Ok(())
}
```

Run it:

```bash
cargo run
```

For this sample, the output is:

```text
message: pacs.008.001.08
message_id: DEMO-001
amount: 1234.56 EUR
```

`detect` reads the message identity from its XML namespace. `MxNode` provides generic tree access, so no generated payment types are required. The example has one amount and one group header. In multi-transaction or enveloped messages, navigate the relevant subtree or use the business-message helpers; a generic first-match field lookup is not a complete payment routing policy.

Now add typed parsing, JSON and decimal conversion:

```bash
cargo add rust_iso20022@0.1.6 --features model-pacs,serde,convert
```

`model-pacs` compiles the pacs family, `serde` enables JSON serialization and `convert` provides decimal/date helpers. Replace `src/main.rs` with the following complete program:

```rust
use rust_iso20022::generated::pacs::pacs_008_001_08::Document;
use rust_iso20022::{detect, from_xml, to_json, to_xml};

const XML: &str = r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
  <FIToFICstmrCdtTrf>
    <GrpHdr><MsgId>DEMO-001</MsgId><NbOfTxs>1</NbOfTxs></GrpHdr>
    <CdtTrfTxInf>
      <PmtId><EndToEndId>E2E-DEMO-001</EndToEndId></PmtId>
      <IntrBkSttlmAmt Ccy="EUR">1234.56</IntrBkSttlmAmt>
    </CdtTrfTxInf>
  </FIToFICstmrCdtTrf>
</Document>"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id = detect(XML).ok_or("unrecognized ISO 20022 message")?;
    println!("message: {}", id.message_name());

    let document: Document = from_xml(XML)?;
    let payment = &document.fi_to_fi_cstmr_cdt_trf;
    println!("message_id: {}", payment.grp_hdr.msg_id.0);

    let amount = &payment.cdt_trf_tx_inf[0].intr_bk_sttlm_amt;
    println!("amount: {} {}", amount.value, amount.ccy.0);

    let decimal = rust_iso20022::convert::to_decimal(&amount.value)
        .ok_or("invalid decimal amount")?;
    println!("doubled: {}", decimal + decimal);

    let serialized_xml = to_xml(&document)?;
    let reparsed: Document = from_xml(&serialized_xml)?;
    println!("xml_round_trip: {}", document == reparsed);

    println!("json: {}", to_json(&document)?);
    Ok(())
}
```

Run `cargo run` again. The lines before the JSON document are:

```text
message: pacs.008.001.08
message_id: DEMO-001
amount: 1234.56 EUR
doubled: 2469.12
xml_round_trip: true
```

The field path refers directly to the generated `Document`. Financial scalars retain their string representation; the conversion helper returns a Decimal for arithmetic without introducing a binary floating-point amount. The equality check demonstrates an XML round-trip for this sample and these SDK types. It does not imply preservation of the original whitespace, prefix spelling or byte sequence.

The final line serializes the generated model to JSON for internal application processing. JSON output is not a claim that a banking network accepts JSON in place of its required ISO 20022 XML format.

The default feature set is empty. For typed processing, choose the needed business family rather than enabling the aggregate `model` feature. Compiling all 1,130 models together has substantial compilation and memory cost; even the pacs family adds more build work than the inspection core.

Validation is a separate decision. Complete offline XSD validation is not implemented. Generated-message L2 binding currently covers `pacs.008.001.08`, and CBPR+/SEPA public-reference checks are provisional developer checks rather than complete production rule packs or bank certification. Parsing or round-tripping this sample does not establish scheme compliance.

For a larger integration flow and existing examples:

- [Payment pipeline example](https://github.com/rust-iso/rust_iso20022/blob/v0.1.6/examples/payment_pipeline.rs)
- [Feature selection](https://github.com/rust-iso/rust_iso20022/blob/main/docs/model-features.md)
- [Coverage and limitations](https://github.com/rust-iso/rust_iso20022/blob/main/docs/status.md)
- [API documentation](https://rust-iso.github.io/rust_iso20022/)
- [Repository](https://github.com/rust-iso/rust_iso20022) and [crates.io package](https://crates.io/crates/rust_iso20022)

If you process ISO 20022 in Rust, which message versions and field-access workflows would make the most useful next examples?

Disclosure: This tutorial was prepared with AI assistance. The code examples were compiled and run locally against SDK source corresponding to the 0.1.6 release.
