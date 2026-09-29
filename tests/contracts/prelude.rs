#![cfg(feature = "model-pacs")]

use rust_iso20022::prelude::{Pacs002Document, Pacs008Document, Pacs009Document};

#[test]
fn prelude_aliases_are_the_generated_canonical_types() {
    let _: Pacs002Document = Default::default();
    let _: Pacs008Document = Default::default();
    let _: Pacs009Document = Default::default();
}
