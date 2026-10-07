//! The bounded CAMT CI snapshot still uses the production dispatch and builders.
#![cfg(all(feature = "model-camt", feature = "serde"))]

use rust_iso20022::builders::{Camt053Builder, CamtReportId};
use rust_iso20022::generated::camt::camt_053_001_09;
use rust_iso20022::prelude::Camt053Document;
use rust_iso20022::{AnyMessage, parse, parse_auto, parse_json, to_xml};

#[test]
fn camt_builder_alias_and_xml_json_dispatch_keep_canonical_types() {
    std::thread::Builder::new()
        .stack_size(64 << 20)
        .spawn(|| {
            let document: Camt053Document = Camt053Builder::new()
                .message_id("CI-SHARD-053")
                .statement_id(CamtReportId::parse("STATEMENT-053").unwrap())
                .build()
                .unwrap();
            let xml = to_xml(&document).unwrap();
            let typed: Box<camt_053_001_09::Document> =
                parse_auto(&xml).unwrap().into_camt_053_001_09().unwrap();
            assert_eq!(typed.bk_to_cstmr_stmt.grp_hdr.msg_id.0, "CI-SHARD-053");

            let parsed = parse(&xml).unwrap();
            assert_eq!(parsed.message_id().to_string(), "camt.053.001.09");
            assert!(matches!(parsed.message(), AnyMessage::Camt_053_001_09(_)));
            let json = parsed.to_json().unwrap();
            let parsed_json = parse_json("camt.053.001.09", &json).unwrap();
            assert_eq!(parsed_json.to_xml().unwrap(), parsed.to_xml().unwrap());
            assert_eq!(
                parsed_json
                    .message()
                    .as_camt_053_001_09()
                    .unwrap()
                    .bk_to_cstmr_stmt
                    .stmt[0]
                    .id
                    .0,
                "STATEMENT-053"
            );
        })
        .unwrap()
        .join()
        .unwrap();
}
