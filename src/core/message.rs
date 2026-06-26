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
    yaserde::ser::to_string(value).map_err(Error::Serialize)
}

/// Serialize an MX message to XML without the `<?xml ...?>` declaration.
pub fn to_xml_fragment<T: yaserde::YaSerialize>(value: &T) -> Result<String, Error> {
    let cfg = yaserde::ser::Config {
        write_document_declaration: false,
        ..Default::default()
    };
    yaserde::ser::to_string_with_config(value, &cfg).map_err(Error::Serialize)
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
