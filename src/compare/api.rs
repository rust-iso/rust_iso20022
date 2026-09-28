use super::{CompareError, VersionDiff, compare_descriptors};

/// Compare two exact message versions from the generated schema catalogue.
pub fn compare_versions(from: &str, to: &str) -> Result<VersionDiff, CompareError> {
    let from_descriptor =
        crate::catalogue::lookup_descriptor(from).ok_or_else(|| CompareError::UnknownMessage {
            message_id: from.to_owned(),
        })?;
    let to_descriptor =
        crate::catalogue::lookup_descriptor(to).ok_or_else(|| CompareError::UnknownMessage {
            message_id: to.to_owned(),
        })?;
    compare_descriptors(from_descriptor, to_descriptor)
}
