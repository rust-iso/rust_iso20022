//! Schema-metadata version comparison.
//!
//! Comparisons operate only on immutable descriptors emitted by codegen. They
//! do not diff XML text, inspect message instances, or infer field renames.

mod api;

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::metadata::{FieldDescriptor, MessageDescriptor};

pub use api::compare_versions;

/// The six semantic change classes supported by the comparison contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum SchemaChangeKind {
    FieldAdded,
    FieldRemoved,
    TypeChanged,
    CardinalityChanged,
    EnumValueAdded,
    EnumValueRemoved,
}

/// One deterministic field-level schema change.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SchemaChange {
    pub path: String,
    pub kind: SchemaChangeKind,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub before: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub after: Option<String>,
}

/// Stable identity and provenance of one comparison endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct VersionEndpoint {
    pub identity: &'static str,
    pub schema_sha256: &'static str,
}

/// Complete ordered difference between two versions in the same family.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct VersionDiff {
    pub from: VersionEndpoint,
    pub to: VersionEndpoint,
    pub changes: Vec<SchemaChange>,
}

/// Typed comparison failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompareError {
    UnknownMessage { message_id: String },
    UnrelatedFamilies { from: String, to: String },
}

impl fmt::Display for CompareError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownMessage { message_id } => {
                write!(formatter, "unknown ISO 20022 message: {message_id}")
            }
            Self::UnrelatedFamilies { from, to } => {
                write!(
                    formatter,
                    "message families are not comparable: {from}, {to}"
                )
            }
        }
    }
}

impl std::error::Error for CompareError {}

/// Compare two schema descriptors without using their source XML text.
pub fn compare_descriptors(
    from: &MessageDescriptor,
    to: &MessageDescriptor,
) -> Result<VersionDiff, CompareError> {
    if family(from.identity) != family(to.identity) {
        return Err(CompareError::UnrelatedFamilies {
            from: from.identity.to_owned(),
            to: to.identity.to_owned(),
        });
    }

    let from_fields: BTreeMap<_, _> = from
        .fields
        .iter()
        .map(|field| (field.logical_path, field))
        .collect();
    let to_fields: BTreeMap<_, _> = to
        .fields
        .iter()
        .map(|field| (field.logical_path, field))
        .collect();
    let paths: BTreeSet<_> = from_fields
        .keys()
        .chain(to_fields.keys())
        .copied()
        .collect();
    let mut changes = Vec::new();

    for path in paths {
        match (from_fields.get(path), to_fields.get(path)) {
            (None, Some(field)) => changes.push(SchemaChange {
                path: path.to_owned(),
                kind: SchemaChangeKind::FieldAdded,
                before: None,
                after: Some(field_shape(field)),
            }),
            (Some(field), None) => changes.push(SchemaChange {
                path: path.to_owned(),
                kind: SchemaChangeKind::FieldRemoved,
                before: Some(field_shape(field)),
                after: None,
            }),
            (Some(before), Some(after)) => compare_field(path, before, after, &mut changes),
            (None, None) => unreachable!(),
        }
    }
    changes.sort();

    Ok(VersionDiff {
        from: endpoint(from),
        to: endpoint(to),
        changes,
    })
}

fn compare_field(
    path: &str,
    before: &FieldDescriptor,
    after: &FieldDescriptor,
    changes: &mut Vec<SchemaChange>,
) {
    if before.type_name != after.type_name {
        changes.push(SchemaChange {
            path: path.to_owned(),
            kind: SchemaChangeKind::TypeChanged,
            before: Some(before.type_name.to_owned()),
            after: Some(after.type_name.to_owned()),
        });
    }
    if (before.min_occurs, before.max_occurs) != (after.min_occurs, after.max_occurs) {
        changes.push(SchemaChange {
            path: path.to_owned(),
            kind: SchemaChangeKind::CardinalityChanged,
            before: Some(cardinality(before)),
            after: Some(cardinality(after)),
        });
    }

    let old: BTreeSet<_> = before.enum_values.iter().copied().collect();
    let new: BTreeSet<_> = after.enum_values.iter().copied().collect();
    for value in new.difference(&old) {
        changes.push(SchemaChange {
            path: path.to_owned(),
            kind: SchemaChangeKind::EnumValueAdded,
            before: None,
            after: Some((*value).to_owned()),
        });
    }
    for value in old.difference(&new) {
        changes.push(SchemaChange {
            path: path.to_owned(),
            kind: SchemaChangeKind::EnumValueRemoved,
            before: Some((*value).to_owned()),
            after: None,
        });
    }
}

fn endpoint(descriptor: &MessageDescriptor) -> VersionEndpoint {
    VersionEndpoint {
        identity: descriptor.identity,
        schema_sha256: descriptor.schema_sha256,
    }
}

fn family(identity: &str) -> &str {
    identity
        .rsplit_once('.')
        .map_or(identity, |(family, _)| family)
}

fn cardinality(field: &FieldDescriptor) -> String {
    let max = field
        .max_occurs
        .map_or_else(|| "unbounded".to_owned(), |value| value.to_string());
    format!("{}..{max}", field.min_occurs)
}

fn field_shape(field: &FieldDescriptor) -> String {
    format!("{} [{}]", field.type_name, cardinality(field))
}
