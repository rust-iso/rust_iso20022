//! Generic XML (de)serialization helpers for ISO 20022 MX message types.
//!
//! Every generated message type derives `yaserde`'s `YaSerialize` /
//! `YaDeserialize`, so these wrappers work for any of them and map the
//! underlying string errors onto [`Error`].
//!
//! # Examples
//! ```ignore
//! // Requires the `model` feature.
//! use rust_iso20022::generated::pacs::pacs_008_001_08::Document;
//!
//! let xml = std::fs::read_to_string("message.xml").unwrap();
//! let doc: Document = rust_iso20022::from_xml(&xml).unwrap();
//! let back = rust_iso20022::to_xml(&doc).unwrap();
//! ```

use crate::core::Error;

/// Parse an MX message of type `T` from its XML representation.
pub fn from_xml<T: yaserde::YaDeserialize>(xml: &str) -> Result<T, Error> {
    yaserde::de::from_str(xml).map_err(Error::Deserialize)
}

/// Serialize an MX message to XML.
pub fn to_xml<T: yaserde::YaSerialize>(value: &T) -> Result<String, Error> {
    yaserde::ser::to_string(value)
        .map(|s| strip_unknown(&s))
        .map_err(Error::Serialize)
}

/// Serialize an MX message to XML without the `<?xml ...?>` declaration.
pub fn to_xml_fragment<T: yaserde::YaSerialize>(value: &T) -> Result<String, Error> {
    let cfg = yaserde::ser::Config {
        write_document_declaration: false,
        ..Default::default()
    };
    yaserde::ser::to_string_with_config(value, &cfg)
        .map(|s| strip_unknown(&s))
        .map_err(Error::Serialize)
}

/// Generated choice enums carry a synthetic `__Unknown__` fallback variant for
/// the "no valid choice selected" state. yaserde renders it as an invalid
/// `<__Unknown__>…</__Unknown__>` element; remove those so an unset/absent choice
/// does not leak a placeholder into the output. Re-parsing the cleaned XML
/// yields the same value (an absent choice deserializes back to `__Unknown__`),
/// so this does not affect round-tripping.
fn strip_unknown(xml: &str) -> String {
    if !xml.contains("__Unknown__") {
        return xml.to_string();
    }
    let mut out = String::with_capacity(xml.len());
    let mut rest = xml;
    loop {
        let Some(open) = rest.find("<__Unknown__") else {
            out.push_str(rest);
            break;
        };
        out.push_str(&rest[..open]);
        let after = &rest[open..];
        // Self-closing `<__Unknown__/>` or `<__Unknown__ ... />`.
        if let Some(end) = after.find("/>") {
            if !after[..end].contains('>') {
                rest = &after[end + 2..];
                continue;
            }
        }
        // Paired `<__Unknown__ ...>...</__Unknown__>`.
        if let Some(close) = after.find("</__Unknown__>") {
            rest = &after[close + "</__Unknown__>".len()..];
            continue;
        }
        // Malformed; keep the rest verbatim.
        out.push_str(after);
        break;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::strip_unknown;

    #[test]
    fn strips_unknown_elements() {
        assert_eq!(
            strip_unknown("<AdrTp><__Unknown__>No valid variants</__Unknown__></AdrTp>"),
            "<AdrTp></AdrTp>"
        );
        assert_eq!(strip_unknown("<a><__Unknown__/></a>"), "<a></a>");
        assert_eq!(strip_unknown("<a><IBAN>x</IBAN></a>"), "<a><IBAN>x</IBAN></a>");
    }
}

/// Serialize an MX message to JSON (requires the `serde` feature, and the
/// `model` feature for the generated message types). Mirrors prowide's
/// `AbstractMX.toJson`.
#[cfg(feature = "serde")]
pub fn to_json<T: serde::Serialize>(value: &T) -> Result<String, Error> {
    serde_json::to_string(value).map_err(|e| Error::Serialize(e.to_string()))
}

/// Deserialize an MX message from JSON. Mirrors prowide's `AbstractMX.fromJson`.
#[cfg(feature = "serde")]
pub fn from_json<T: serde::de::DeserializeOwned>(json: &str) -> Result<T, Error> {
    serde_json::from_str(json).map_err(|e| Error::Deserialize(e.to_string()))
}
