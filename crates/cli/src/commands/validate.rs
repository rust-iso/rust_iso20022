use serde::Serialize;

use super::CommandError;

#[derive(Debug, Serialize)]
pub struct ValidateData {
    pub message_id: String,
    pub layer: &'static str,
    pub report: rust_iso20022::validation::ValidationReport,
}

#[cfg(feature = "model-pacs")]
pub fn validate(xml: &str) -> Result<ValidateData, CommandError> {
    let parsed = rust_iso20022::parse(xml).map_err(|error| match error {
        rust_iso20022::ParseError::UnsupportedFeature { .. } => CommandError::unavailable(),
        rust_iso20022::ParseError::UnknownMessage => {
            CommandError::parse("unknown_message", "unknown ISO 20022 message")
        }
        rust_iso20022::ParseError::RootMismatch { .. } => CommandError::parse(
            "root_mismatch",
            "message namespace is attached to an unexpected root element",
        ),
        rust_iso20022::ParseError::Deserialize { .. } => CommandError::parse(
            "deserialize_failed",
            "XML could not be deserialized as the selected generated message",
        ),
    })?;
    let message_id = parsed.message_id().as_str().to_owned();
    let report = rust_iso20022::validation::bindings::validate_parsed_message(&parsed)
        .map_err(|_| CommandError::unavailable())?;
    Ok(ValidateData {
        message_id,
        layer: "iso_semantic",
        report,
    })
}

#[cfg(not(feature = "model-pacs"))]
pub fn validate(_xml: &str) -> Result<ValidateData, CommandError> {
    Err(CommandError::unavailable())
}
