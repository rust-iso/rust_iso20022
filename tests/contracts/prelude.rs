#![cfg(feature = "model-pacs")]

use rust_iso20022::prelude::{Pacs002Document, Pacs008Document, Pacs009Document};
use rust_iso20022::{AnyMessage, lookup_message, parse_auto, required_feature};

#[cfg(feature = "model-head")]
use rust_iso20022::prelude::BusinessApplicationHeaderV02;
#[cfg(feature = "model-camt")]
use rust_iso20022::prelude::Camt053Document;
#[cfg(feature = "model-pain")]
use rust_iso20022::prelude::{Pain001Document, Pain002Document};

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
    assert_eq!(required_feature(by_namespace.namespace), Some("model-pacs"));
}

#[test]
fn root_auto_dispatch_is_the_generated_any_message() {
    let xml = include_str!("../../fixtures/iso/valid/pacs.008.001.08-cross-field.xml");
    let parsed = parse_auto(xml).expect("generated auto-dispatch");
    assert!(matches!(parsed, AnyMessage::Pacs_008_001_08(_)));
}

#[cfg(feature = "model-head")]
#[test]
fn head_prelude_alias_is_the_generated_canonical_type() {
    let _: BusinessApplicationHeaderV02 = Default::default();
}

#[cfg(feature = "model-camt")]
#[test]
fn camt_prelude_alias_is_the_generated_canonical_type() {
    let _: Camt053Document = Default::default();
}

#[cfg(feature = "model-pain")]
#[test]
fn pain_prelude_aliases_are_the_generated_canonical_types() {
    let _: Pain001Document = Default::default();
    let _: Pain002Document = Default::default();
}
