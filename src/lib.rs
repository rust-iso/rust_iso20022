//! # rust_iso20022
//!
//! ISO 20022 financial message definitions and identification for Rust,
//! generated from the official ISO 20022 XSD schemas.
//!
//! The crate has three layers:
//!
//! * a small hand-written **core** ([`MxId`], [`BusinessArea`], the
//!   [`from_xml`] / [`to_xml`] helpers and the [`Error`] type);
//! * a generated **message model** (`generated`) — one Rust module per
//!   message version, e.g. `generated::pacs::pacs_008_001_08`, whose types
//!   derive `yaserde` for XML (de)serialization; and
//! * a generated **catalogue** ([`catalogue`]) listing every message id and its
//!   namespace, as static [`phf`] tables.
//!
//! ## Which API should I use?
//!
//! | Task | API | Feature |
//! |---|---|---|
//! | Identify a message from its XML namespace | [`detect`], [`MxId`] | none |
//! | Inspect arbitrary fields without generated types | [`MxNode::parse`] | none |
//! | Read AppHdr and common business metadata | [`read_business_message`] | none |
//! | Parse or build generated message types | [`from_xml`], [`to_xml`] | `model-<area>` |
//! | Serialize as JSON | `from_json`, `to_json` | `serde` |
//! | Convert amount and date strings | `convert` | `convert` |
//!
//! The first four letters of a message name select its model feature and Rust
//! module. For example, `pacs.008.001.08` uses feature `model-pacs` and type
//! `generated::pacs::pacs_008_001_08::Document`. Prefer per-area features over
//! the umbrella `model` feature, which compiles all 1,130 message modules.
//!
//! See the [model feature guide] for all 32 generated business areas, or start
//! with the repository's runnable [`inspect_message`] and [`typed_payment`]
//! examples.
//!
//! [model feature guide]: https://github.com/rust-iso/rust_iso20022/blob/main/docs/model-features.md
//! [`inspect_message`]: https://github.com/rust-iso/rust_iso20022/blob/main/examples/inspect_message.rs
//! [`typed_payment`]: https://github.com/rust-iso/rust_iso20022/blob/main/examples/typed_payment.rs
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
mod node;
/// Compatibility XSD-facet validation retained for generated values.
pub mod validate;

/// Additive typed builders whose outputs are canonical generated documents.
pub mod builders;
/// Deterministic semantic comparison of schema-derived message versions.
pub mod compare;
/// Exact, source-addressed market-profile releases and dispatch.
#[cfg(feature = "profiles")]
pub mod profiles;
/// Layered reports, executable rules, and explanation registry.
pub mod validation;

/// Serde-backed compatibility views used by legacy WebAssembly exports.
#[doc(hidden)]
#[cfg(feature = "serde")]
pub mod wasm_compat;

pub use node::MxNode;

/// Typed conversions for scalar values (`convert` feature).
#[cfg(feature = "convert")]
pub mod convert;

#[cfg(all(direct_wasm, target_arch = "wasm32", not(feature = "serde")))]
compile_error!("direct_wasm requires --features serde for structured compatibility bindings");

#[cfg(all(direct_wasm, target_arch = "wasm32", feature = "serde"))]
mod wasm;

#[cfg(all(feature = "__message_model", feature = "serde"))]
pub use crate::core::parse_json;
pub use crate::core::{
    BoundedXmlReader, BusinessArea, BusinessMessage, DetectedMessage, DetectionError, Envelope,
    Error, MessageFamily, MessageIdentifier, MessageVersion, MxId, MxMessage, ParseError,
    ParseLimits, Result, SerializeError, XmlLimit, XmlReadError, XmlStats, detect, detect_message,
    from_xml, parse_envelope, read_business_message, to_xml, to_xml_fragment, validate_xml,
};
#[cfg(feature = "__message_model")]
pub use crate::core::{MessageRef, ParsedMessage, parse};
#[cfg(feature = "serde")]
pub use crate::core::{from_json, to_json};

/// Parse XML into the typed message `T` only if the XML's detected message type
/// matches `T` (prowide-style guarded parse). Requires the `model` feature on
/// the caller side to name a concrete `Document` type.
///
/// ```
/// # #[cfg(feature = "model-pacs")] {
/// use rust_iso20022::generated::pacs::pacs_008_001_08::Document;
/// let xml = r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08"></Document>"#;
/// // Parses because the detected type matches `Document`.
/// let _ = rust_iso20022::parse_as::<Document>(xml);
/// # }
/// ```
pub fn parse_as<T: MxMessage>(xml: &str) -> Result<T> {
    T::parse_checked(xml)
}

/// Static schema-derived message and field discovery.
pub mod catalogue;
pub use catalogue::{Catalogue, CatalogueEntry, SchemaCatalogue, lookup_message, required_feature};

/// Small, stable convenience surface for common generated payment messages.
///
/// These are aliases to the generated structs, not a second message model.
/// The module is available only when the corresponding model feature is
/// enabled; all generated paths remain available under [`crate::generated`].
#[cfg(feature = "model-pacs")]
pub mod prelude {
    pub use crate::generated::pacs::pacs_002_001_10::Document as Pacs002Document;
    pub use crate::generated::pacs::pacs_008_001_08::Document as Pacs008Document;
    pub use crate::generated::pacs::pacs_009_001_08::Document as Pacs009Document;
}

/// Business Application Header (BAH / `head.001`) reading.
pub use crate::core::app_hdr;

/// Schema descriptors plus business metadata extraction compatibility APIs;
/// descriptors never replace the generated message model.
pub mod metadata;

/// Validated scalar helpers with typed errors and generated-field conversions.
pub mod helpers;

/// Financial-data redaction policy for diagnostics and adapters.
pub mod privacy;

/// Runtime ISO 20022 schema fetcher (`catalogue` feature).
#[cfg(feature = "catalogue")]
pub mod fetch;

/// Generated ISO 20022 message types, one module per message version. Enabled
/// by any `model-<area>` feature (or `model` for all areas).
#[cfg(feature = "__model")]
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
