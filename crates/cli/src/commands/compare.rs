use rust_iso20022::compare::{CompareError, VersionDiff};

use super::CommandError;

pub fn compare(from: &str, to: &str) -> Result<VersionDiff, CommandError> {
    rust_iso20022::compare::compare_versions(from, to).map_err(|error| match error {
        CompareError::UnknownMessage { .. } => CommandError::unknown_message(),
        CompareError::UnrelatedFamilies { .. } => CommandError::unrelated_families(),
    })
}
