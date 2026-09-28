use rust_iso20022::builders::{Pacs002Builder, Pacs008Builder, Pacs009Builder};
use rust_iso20022::generated::pacs::{pacs_002_001_10, pacs_008_001_08, pacs_009_001_08};
use rust_iso20022::helpers::Money;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let payment: pacs_008_001_08::Document = Pacs008Builder::new()
        .message_id("EXAMPLE-008")
        .transaction_id("EXAMPLE-TX-008")
        .settlement(Money::parse("EUR", "125.50")?)
        .build()?;
    assert_eq!(
        payment.fi_to_fi_cstmr_cdt_trf.grp_hdr.msg_id.0,
        "EXAMPLE-008"
    );

    let cover: pacs_009_001_08::Document = Pacs009Builder::new()
        .message_id("EXAMPLE-009")
        .transaction_id("EXAMPLE-TX-009")
        .settlement(Money::parse("USD", "250.00")?)
        .build()?;
    assert_eq!(cover.fi_cdt_trf.grp_hdr.msg_id.0, "EXAMPLE-009");

    let status: pacs_002_001_10::Document = Pacs002Builder::new()
        .message_id("EXAMPLE-002")
        .original_group("EXAMPLE-008", "pacs.008.001.08", "ACCP")
        .build()?;
    assert_eq!(
        status.fi_to_fi_pmt_sts_rpt.orgnl_grp_inf_and_sts[0]
            .orgnl_msg_id
            .0,
        "EXAMPLE-008"
    );
    Ok(())
}
