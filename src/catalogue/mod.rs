//! The ISO 20022 message catalogue: every message id known to this crate, with
//! its XSD namespace and business area, as static [`phf`] tables.
//!
//! The data ([`data`]) is generated from the XSD schemas by `src/bin/codegen.rs`
//! and covers the full set of staged schemas, including a few messages whose
//! Rust model is not yet generated (see the crate README for the known gap).
//!
//! ```
//! use rust_iso20022::catalogue;
//!
//! assert!(catalogue::contains("pacs.008.001.08"));
//! let e = catalogue::from_message_name("pacs.008.001.08").unwrap();
//! assert_eq!(e.business_area, "pacs");
//! assert!(catalogue::all().len() > 400);
//! ```

/// A catalogue entry describing one ISO 20022 message version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CatalogueEntry {
    /// Canonical message name, e.g. `"pacs.008.001.08"`.
    pub message_name: &'static str,
    /// XSD target namespace, e.g. `"urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08"`.
    pub namespace: &'static str,
    /// 4-letter business area code, e.g. `"pacs"`.
    pub business_area: &'static str,
    /// Whether a generated Rust model exists for this message in [`crate::generated`].
    pub has_model: bool,
}

pub mod data;

/// All catalogue entries, sorted by message name.
pub fn all() -> &'static [CatalogueEntry] {
    data::ENTRIES
}

/// Look up an entry by exact message name, e.g. `"pacs.008.001.08"`.
pub fn from_message_name(message_name: &str) -> Option<&'static CatalogueEntry> {
    data::BY_NAME.get(message_name)
}

/// Look up an entry by exact XSD namespace.
pub fn from_namespace(namespace: &str) -> Option<&'static CatalogueEntry> {
    data::ENTRIES.iter().find(|e| e.namespace == namespace)
}

/// Whether the catalogue contains the given message name.
pub fn contains(message_name: &str) -> bool {
    data::BY_NAME.contains_key(message_name)
}
