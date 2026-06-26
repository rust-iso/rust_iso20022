//! # rust_iso20022
//!
//! ISO 20022 financial message definitions and identification for Rust,
//! generated from the official ISO 20022 XSD schemas.
//!
//! The crate has three layers:
//!
//! * a small hand-written **core** ([`MxId`], [`BusinessArea`], the
//!   [`from_xml`] / [`to_xml`] helpers and the [`Error`] type);
//! * a generated **message model** ([`generated`]) — one Rust module per
//!   message version, e.g. [`generated::pacs::pacs_008_001_08`], whose types
//!   derive `yaserde` for XML (de)serialization; and
//! * a generated **catalogue** ([`catalogue`]) listing every message id and its
//!   namespace, as static [`phf`] tables.
//!
//! ## Sample code
//! ```
//! use rust_iso20022::{MxId, BusinessArea};
//!
//! // Identify a message from its namespace or bare name.
//! let id = rust_iso20022::from_namespace(
//!     "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08",
//! )
//! .unwrap();
//! assert_eq!(id.business_area, BusinessArea::pacs);
//! assert_eq!(id.message_name(), "pacs.008.001.08");
//!
//! // The catalogue knows every generated message.
//! assert!(rust_iso20022::catalogue::contains("pacs.008.001.08"));
//! let entry = rust_iso20022::catalogue::from_message_name("pacs.008.001.08").unwrap();
//! assert_eq!(entry.namespace, "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08");
//! ```

#[macro_use]
mod simple_type;
mod core;
pub mod validate;

#[cfg(all(direct_wasm, target_arch = "wasm32"))]
mod wasm;

pub use crate::core::{
    detect, from_xml, read_business_message, to_xml, to_xml_fragment, BusinessArea,
    BusinessMessage, Error, MxId, MxMessage, Result,
};
#[cfg(feature = "serde")]
pub use crate::core::{from_json, to_json};

/// Parse XML into the typed message `T` only if the XML's detected message type
/// matches `T` (prowide-style guarded parse). Requires the `model` feature on
/// the caller side to name a concrete `Document` type.
///
/// ```ignore
/// use rust_iso20022::generated::pacs::pacs_008_001_08::Document;
/// let doc = rust_iso20022::parse_as::<Document>(&xml)?;
/// ```
pub fn parse_as<T: MxMessage>(xml: &str) -> Result<T> {
    T::parse_checked(xml)
}

pub mod catalogue;

/// Business Application Header (BAH / `head.001`) reading.
pub use crate::core::app_hdr;

/// Business metadata extraction from a message (sender/amount/currency/dates).
pub use crate::core::metadata;

/// Runtime ISO 20022 schema fetcher (`catalogue` feature).
#[cfg(feature = "catalogue")]
pub mod fetch;

/// Generated ISO 20022 message types, one module per message version.
/// Requires the `model` feature.
#[cfg(feature = "model")]
pub mod generated;

/// Parse an [`MxId`] from a full namespace, partial namespace or bare message
/// name. Returns `None` if it cannot be parsed.
///
/// ```
/// let id = rust_iso20022::from_namespace("pacs.008.001.08").unwrap();
/// assert_eq!(id.message_name(), "pacs.008.001.08");
/// assert!(rust_iso20022::from_namespace("not-a-message").is_none());
/// ```
pub fn from_namespace(namespace: &str) -> Option<MxId> {
    MxId::parse(namespace).ok()
}
