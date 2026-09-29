#![cfg(feature = "model-pacs")]

use rust_iso20022::prelude::{Pacs002Document, Pacs008Document, Pacs009Document};
use rust_iso20022::{lookup_message, required_feature};

#[test]
fn prelude_aliases_are_the_generated_canonical_types() {
    let _: Pacs002Document = Default::default();
    let _: Pacs008Document = Default::default();
    let _: Pacs009Document = Default::default();
}

#[test]
fn message_lookup_accepts_name_and_namespace_and_reports_feature() {
    let by_name = lookup_message("pacs.008.001.08").expect("message name");
    let by_namespace = lookup_message(by_name.namespace).expect("message namespace");
    assert_eq!(by_name, by_namespace);
    assert_eq!(required_feature(by_name.message_name), Some("model-pacs"));
}
