#![allow(non_local_definitions)]

use rust_iso20022::{BusinessArea, Error, MxMessage};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Default, YaSerialize, YaDeserialize)]
struct ExternalMessage;

impl MxMessage for ExternalMessage {
    const BUSINESS_AREA: BusinessArea = BusinessArea::pacs;
    const FUNCTIONALITY: &'static str = "999";
    const VARIANT: &'static str = "001";
    const VERSION: &'static str = "01";
    const MESSAGE_NAME: &'static str = "pacs.999.001.01";
    const NAMESPACE: &'static str = "urn:iso:std:iso:20022:tech:xsd:pacs.999.001.01";
}

fn existing_error_is_still_exhaustive(error: Error) -> &'static str {
    match error {
        Error::InvalidMxId(_) => "invalid-id",
        Error::UnknownBusinessArea(_) => "unknown-area",
        Error::Deserialize(_) => "deserialize",
        Error::Serialize(_) => "serialize",
    }
}

fn main() {
    let _ = ExternalMessage::mx_id();
    let _ = existing_error_is_still_exhaustive(Error::Deserialize(String::new()));

    #[cfg(feature = "model-pacs")]
    {
        let xml = include_str!("../../../data/pacs.002.001.10.xml");
        let parsed = rust_iso20022::parse(xml).expect("new API is additive");
        let _: &rust_iso20022::generated::pacs::pacs_002_001_10::Document = parsed
            .as_message_ref()
            .message()
            .as_pacs_002_001_10()
            .expect("typed accessor");
    }
}
