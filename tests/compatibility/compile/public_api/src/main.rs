#![allow(non_local_definitions)]

use rust_iso20022::{
    catalogue, detect, from_namespace, from_xml, parse_as, parse_envelope, read_business_message,
    to_xml, to_xml_fragment, BusinessArea, Error, MxId, MxMessage, Result,
};
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

fn exhaustive_error(error: Error) -> &'static str {
    match error {
        Error::InvalidMxId(_) => "invalid-id",
        Error::UnknownBusinessArea(_) => "unknown-area",
        Error::Deserialize(_) => "deserialize",
        Error::Serialize(_) => "serialize",
    }
}

fn exhaustive_area(area: BusinessArea) -> &'static str {
    use BusinessArea::*;
    match area {
        acmt => "acmt",
        admi => "admi",
        auth => "auth",
        caaa => "caaa",
        caad => "caad",
        caam => "caam",
        cafc => "cafc",
        cafm => "cafm",
        cafr => "cafr",
        cain => "cain",
        camt => "camt",
        canm => "canm",
        casp => "casp",
        casr => "casr",
        catm => "catm",
        catp => "catp",
        cbrf => "cbrf",
        colr => "colr",
        fxtr => "fxtr",
        head => "head",
        pacs => "pacs",
        pain => "pain",
        reda => "reda",
        remt => "remt",
        secl => "secl",
        seev => "seev",
        semt => "semt",
        sese => "sese",
        seti => "seti",
        setr => "setr",
        supl => "supl",
        trck => "trck",
        trea => "trea",
        tsin => "tsin",
        tsmt => "tsmt",
        tsrv => "tsrv",
        xsys => "xsys",
    }
}

fn main() {
    let id = MxId::new(BusinessArea::pacs, "008", "001", "08");
    let _public_fields = (
        id.business_area,
        id.functionality.clone(),
        id.variant.clone(),
        id.version.clone(),
    );
    let _result: Result<MxId> = MxId::parse(&id.namespace());
    let _ = from_namespace(&id.message_name());
    let _ = detect(r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08"/>"#);
    let _: Result<rust_iso20022::Envelope<ExternalMessage>> = parse_envelope("<Envelope/>");
    let _ = read_business_message("<Envelope/>");
    let _ = catalogue::all();
    let entry = rust_iso20022::catalogue::CatalogueEntry {
        message_name: "pacs.008.001.08",
        namespace: "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08",
        business_area: "pacs",
        has_model: true,
    };
    let _entry_fields = (
        entry.message_name,
        entry.namespace,
        entry.business_area,
        entry.has_model,
    );
    let external = ExternalMessage;
    let _ = ExternalMessage::mx_id();
    let _ = ExternalMessage::parse("<ExternalMessage/>");
    let _ = ExternalMessage::parse_checked("<ExternalMessage/>");
    let _ = external.to_xml_string();
    let _ = from_xml::<ExternalMessage>("<ExternalMessage/>");
    let _ = parse_as::<ExternalMessage>("<ExternalMessage/>");
    let _ = to_xml(&external);
    let _ = to_xml_fragment(&external);
    let _ = exhaustive_area(BusinessArea::pacs);
    let _ = exhaustive_error(Error::InvalidMxId(String::new()));

    #[cfg(feature = "serde")]
    {
        let json = rust_iso20022::to_json(&id).unwrap();
        let _: MxId = rust_iso20022::from_json(&json).unwrap();
    }

    #[cfg(feature = "model-pacs")]
    {
        use rust_iso20022::generated::pacs::pacs_002_001_10::Document;
        let _: Document = Default::default();
    }
}
