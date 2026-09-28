use rust_iso20022::builders::{Pain001Builder, Pain002Builder};
use rust_iso20022::generated::pain::{pain_001_001_09, pain_002_001_10};
use rust_iso20022::helpers::Money;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let initiation: pain_001_001_09::Document = Pain001Builder::new()
        .message_id("EXAMPLE-001")
        .payment_info_id("EXAMPLE-PMT-001")
        .end_to_end_id("EXAMPLE-E2E-001")
        .instructed_amount(Money::parse("EUR", "125.50")?)
        .build()?;
    assert_eq!(
        initiation.cstmr_cdt_trf_initn.pmt_inf[0].cdt_trf_tx_inf[0]
            .pmt_id
            .end_to_end_id
            .0,
        "EXAMPLE-E2E-001"
    );

    let status: pain_002_001_10::Document = Pain002Builder::new()
        .message_id("EXAMPLE-002")
        .original_group("EXAMPLE-001", "pain.001.001.09", "ACCP")
        .build()?;
    assert_eq!(
        status
            .cstmr_pmt_sts_rpt
            .orgnl_grp_inf_and_sts
            .orgnl_msg_id
            .0,
        "EXAMPLE-001"
    );
    Ok(())
}
