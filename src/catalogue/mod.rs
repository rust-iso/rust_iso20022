//! The schema-derived ISO 20022 message catalogue.
//!
//! Legacy [`crate::catalogue::CatalogueEntry`] data and descriptor indices are
//! generated from the same normalized schema descriptors as
//! [`crate::metadata`].
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
    /// Whether a generated Rust model exists for this message in
    /// `crate::generated`.
    pub has_model: bool,
}

/// Generated compatibility tables; applications should normally use the
/// typed lookup functions in this module.
pub mod data;
mod generated;

use crate::metadata::MessageDescriptor;
use crate::{MessageFamily, MessageIdentifier};

/// Whether a catalogue identity is backed by a generated model, known without
/// one, or absent from the schema set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverageState {
    Available { required_feature: &'static str },
    KnownUnavailable,
    Unknown,
}

/// Typed catalogue contract over the immutable generated indices.
pub trait Catalogue {
    fn lookup(&self, id: &MessageIdentifier) -> Option<&'static MessageDescriptor>;
    fn list_family(&self, family: &MessageFamily) -> &'static [MessageDescriptor];
    fn versions(&self, family: &MessageFamily) -> &'static [MessageDescriptor];
    fn latest(&self, family: &MessageFamily) -> Option<&'static MessageDescriptor>;
    fn by_namespace(&self, namespace: &str) -> Option<&'static MessageDescriptor>;
    fn by_root(&self, namespace: &str, root: &str) -> Option<&'static MessageDescriptor>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SchemaCatalogue;

pub static SCHEMA_CATALOGUE: SchemaCatalogue = SchemaCatalogue;

impl Catalogue for SchemaCatalogue {
    fn lookup(&self, id: &MessageIdentifier) -> Option<&'static MessageDescriptor> {
        lookup_descriptor(id.as_str())
    }

    fn list_family(&self, family: &MessageFamily) -> &'static [MessageDescriptor] {
        list_family(family.as_str())
    }

    fn versions(&self, family: &MessageFamily) -> &'static [MessageDescriptor] {
        versions(family.as_str())
    }

    fn latest(&self, family: &MessageFamily) -> Option<&'static MessageDescriptor> {
        latest(family.as_str())
    }

    fn by_namespace(&self, namespace: &str) -> Option<&'static MessageDescriptor> {
        by_namespace(namespace)
    }

    fn by_root(&self, namespace: &str, root: &str) -> Option<&'static MessageDescriptor> {
        by_root(namespace, root)
    }
}

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
    by_namespace(namespace).and_then(|descriptor| data::BY_NAME.get(descriptor.identity))
}

/// Resolve either a canonical message name or its full XSD namespace.
///
/// This is a convenience wrapper for routers that receive mixed identifiers;
/// it returns the same schema-derived entry as the exact lookup functions.
pub fn lookup_message(identifier: &str) -> Option<&'static CatalogueEntry> {
    from_message_name(identifier).or_else(|| from_namespace(identifier))
}

/// Return the smallest generated-model feature required for a message.
///
/// The value is derived from schema metadata and is suitable for diagnostics
/// and feature preflight. It does not dynamically load a model at runtime.
pub fn required_feature(message_name: &str) -> Option<&'static str> {
    lookup_descriptor(message_name).map(|descriptor| descriptor.required_feature)
}

/// Whether the catalogue contains the given message name.
pub fn contains(message_name: &str) -> bool {
    data::BY_NAME.contains_key(message_name)
}

/// Look up immutable schema metadata by exact message identifier.
pub fn lookup_descriptor(message_id: &str) -> Option<&'static MessageDescriptor> {
    let descriptors = crate::metadata::all();
    debug_assert_eq!(descriptors.len(), generated::DESCRIPTOR_COUNT);
    generated::BY_ID
        .get(message_id)
        .and_then(|ordinal| descriptors.get(*ordinal))
}

/// Look up one exact normalized field path within an exact message version.
pub fn lookup_field(
    message_id: &str,
    logical_path: &str,
) -> Option<&'static crate::metadata::FieldDescriptor> {
    lookup_descriptor(message_id)?
        .fields
        .iter()
        .find(|field| field.logical_path == logical_path)
}

/// List all versions of one three-component message family in numeric order.
pub fn list_family(family: &str) -> &'static [MessageDescriptor] {
    let descriptors = crate::metadata::all();
    generated::FAMILY_RANGES
        .get(family)
        .and_then(|(start, length)| descriptors.get(*start..(*start + *length)))
        .unwrap_or(&[])
}

/// Alias emphasizing that the returned family slice is version ordered.
pub fn versions(family: &str) -> &'static [MessageDescriptor] {
    list_family(family)
}

/// Numerically latest available version in a family.
pub fn latest(family: &str) -> Option<&'static MessageDescriptor> {
    list_family(family).last()
}

/// Look up by exact schema namespace.
pub fn by_namespace(namespace: &str) -> Option<&'static MessageDescriptor> {
    let descriptors = crate::metadata::all();
    generated::BY_NAMESPACE
        .get(namespace)
        .and_then(|ordinal| descriptors.get(*ordinal))
}

/// Look up by the exact `(namespace, root element)` pair.
pub fn by_root(namespace: &str, root: &str) -> Option<&'static MessageDescriptor> {
    let key = format!("{namespace}\u{1f}{root}");
    let descriptors = crate::metadata::all();
    generated::BY_ROOT
        .get(&key)
        .and_then(|ordinal| descriptors.get(*ordinal))
}

/// Classify schema/model coverage without conflating unknown identifiers with
/// known schemas that lack a generated model.
pub fn coverage(message_id: &str) -> CoverageState {
    if let Some(descriptor) = lookup_descriptor(message_id) {
        CoverageState::Available {
            required_feature: descriptor.required_feature,
        }
    } else if from_message_name(message_id).is_some() {
        CoverageState::KnownUnavailable
    } else {
        CoverageState::Unknown
    }
}
