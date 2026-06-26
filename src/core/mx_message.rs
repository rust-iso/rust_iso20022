//! The `MxMessage` trait — the rust parallel to prowide's `AbstractMX`: every
//! generated message `Document` carries its own ISO 20022 identity and gains
//! parse/serialize convenience methods.
//!
//! The per-message `impl MxMessage for Document { … }` blocks are emitted by
//! `src/bin/codegen.rs`, so they are only present with the `model` feature.

use crate::core::{from_xml, to_xml, BusinessArea, Error, MxId};

/// Identity and (de)serialization contract implemented by every generated
/// message `Document`. Mirrors prowide's `AbstractMX`.
///
/// ```ignore
/// // Requires the `model` feature.
/// use rust_iso20022::MxMessage;
/// use rust_iso20022::generated::pacs::pacs_008_001_08::Document;
///
/// assert_eq!(Document::MESSAGE_NAME, "pacs.008.001.08");
/// assert_eq!(Document::mx_id().business_area, rust_iso20022::BusinessArea::pacs);
///
/// let doc = Document::parse(&xml)?;        // == from_xml
/// let xml = doc.to_xml_string()?;          // == to_xml
/// ```
pub trait MxMessage: yaserde::YaSerialize + yaserde::YaDeserialize + Sized {
    /// Business area, e.g. `BusinessArea::pacs`.
    const BUSINESS_AREA: BusinessArea;
    /// Message functionality (type), e.g. `"008"`.
    const FUNCTIONALITY: &'static str;
    /// Variant, e.g. `"001"`.
    const VARIANT: &'static str;
    /// Version, e.g. `"08"`.
    const VERSION: &'static str;
    /// Canonical message name, e.g. `"pacs.008.001.08"`.
    const MESSAGE_NAME: &'static str;
    /// XSD target namespace of the message.
    const NAMESPACE: &'static str;

    /// The message identification.
    fn mx_id() -> MxId {
        MxId::new(
            Self::BUSINESS_AREA,
            Self::FUNCTIONALITY,
            Self::VARIANT,
            Self::VERSION,
        )
    }

    /// Parse the message from XML.
    fn parse(xml: &str) -> Result<Self, Error> {
        from_xml(xml)
    }

    /// Parse only if the XML's detected message type matches this one,
    /// otherwise return [`Error::InvalidMxId`].
    fn parse_checked(xml: &str) -> Result<Self, Error> {
        match detect(xml) {
            Some(id) if id.message_name() == Self::MESSAGE_NAME => from_xml(xml),
            Some(id) => Err(Error::InvalidMxId(format!(
                "expected {}, found {}",
                Self::MESSAGE_NAME,
                id.message_name()
            ))),
            None => Err(Error::InvalidMxId(
                "no ISO 20022 namespace found in XML".to_string(),
            )),
        }
    }

    /// Serialize the message to XML.
    fn to_xml_string(&self) -> Result<String, Error> {
        to_xml(self)
    }
}

/// Detect the ISO 20022 message type of a raw XML document from the namespace
/// declared on its `Document` (or other root) element. Does not require the
/// message model.
///
/// When a Business Application Header is present, the message `Document`
/// namespace is preferred over the `head.*` header namespace.
///
/// ```
/// let xml = r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08"></Document>"#;
/// let id = rust_iso20022::detect(xml).unwrap();
/// assert_eq!(id.message_name(), "pacs.008.001.08");
/// ```
pub fn detect(xml: &str) -> Option<MxId> {
    let mut first: Option<MxId> = None;
    for token in xml.split(|c: char| matches!(c, '"' | '\'' | '<' | '>' | ' ' | '\t' | '\n' | '\r'))
    {
        if !token.starts_with("urn:") {
            continue;
        }
        if let Ok(id) = MxId::parse(token) {
            // Prefer the actual message over the BAH header namespace.
            if id.business_area != BusinessArea::head {
                return Some(id);
            }
            first.get_or_insert(id);
        }
    }
    first
}
