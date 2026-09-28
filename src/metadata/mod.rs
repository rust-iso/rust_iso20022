//! Immutable schema-derived message and field metadata.
//!
//! This module describes generated structs and their XSD provenance. It does
//! not contain message-instance values and is not a second ISO 20022 model;
//! generated structs remain canonical. Existing best-effort business metadata
//! extraction remains available through `metadata::extract`.
//!
//! ```
//! let pacs008 = rust_iso20022::metadata::all()
//!     .iter()
//!     .find(|descriptor| descriptor.identity == "pacs.008.001.08")
//!     .expect("pacs.008 descriptor");
//! assert_eq!(pacs008.root_element, "Document");
//! assert_eq!(pacs008.required_feature, "model-pacs");
//! assert!(pacs008.fields.iter().any(|field| {
//!     field.logical_path == "Document/FIToFICstmrCdtTrf"
//! }));
//! ```

pub use crate::core::metadata::{MessageMetadata, extract};

mod generated;

/// The XSD declaration kind represented by a field descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum FieldKind {
    Element,
    Attribute,
    SimpleContent,
}

/// Immutable schema declaration metadata for one generated Rust field.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldDescriptor {
    pub logical_path: &'static str,
    pub namespace: &'static str,
    pub wire_name: &'static str,
    pub rust_name: &'static str,
    pub type_name: &'static str,
    pub min_occurs: u32,
    /// `None` represents `maxOccurs="unbounded"`.
    pub max_occurs: Option<u32>,
    pub enum_values: &'static [&'static str],
    pub documentation: Option<&'static str>,
    pub choice_group: Option<u32>,
    pub kind: FieldKind,
}

/// Immutable provenance and field graph for one generated message version.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MessageDescriptor {
    pub identity: &'static str,
    pub business_area: &'static str,
    pub namespace: &'static str,
    pub root_element: &'static str,
    pub root_type: &'static str,
    pub description: Option<&'static str>,
    pub schema_path: &'static str,
    pub schema_sha256: &'static str,
    pub generated_module: &'static str,
    pub required_feature: &'static str,
    pub fields: &'static [FieldDescriptor],
}

/// All schema-derived descriptors in canonical message-identity order.
pub fn all() -> &'static [MessageDescriptor] {
    generated::all()
}
