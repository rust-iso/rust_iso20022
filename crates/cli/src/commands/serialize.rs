use serde::Serialize;

use super::CommandError;

#[derive(Debug, Serialize)]
pub struct JsonData {
    pub message_id: String,
    pub document: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct XmlData {
    pub message_id: String,
    pub document: String,
}

#[cfg(all(feature = "json", feature = "model-pacs"))]
pub fn to_json(xml: &str) -> Result<JsonData, CommandError> {
    let parsed = rust_iso20022::parse(xml).map_err(classify_parse_error)?;
    let message_id = parsed.message_id().as_str().to_owned();
    let serialized = parsed
        .to_json()
        .map_err(|_| CommandError::parse("serialize_failed", "JSON serialization failed"))?;
    let document = serde_json::from_str(&serialized)
        .map_err(|_| CommandError::parse("serialize_failed", "JSON serialization failed"))?;
    Ok(JsonData {
        message_id,
        document,
    })
}

#[cfg(not(all(feature = "json", feature = "model-pacs")))]
pub fn to_json(_xml: &str) -> Result<JsonData, CommandError> {
    Err(CommandError::unavailable())
}

#[cfg(all(feature = "json", feature = "model-pacs"))]
pub fn to_xml(message_id: &str, json: &str) -> Result<XmlData, CommandError> {
    let parsed = rust_iso20022::parse_json(message_id, json).map_err(classify_parse_error)?;
    let message_id = parsed.message_id().as_str().to_owned();
    let document = parsed
        .to_xml()
        .map_err(|_| CommandError::parse("serialize_failed", "XML serialization failed"))?;
    Ok(XmlData {
        message_id,
        document,
    })
}

#[cfg(not(all(feature = "json", feature = "model-pacs")))]
pub fn to_xml(_message_id: &str, _json: &str) -> Result<XmlData, CommandError> {
    Err(CommandError::unavailable())
}

#[cfg(all(feature = "json", feature = "model-pacs"))]
fn classify_parse_error(error: rust_iso20022::ParseError) -> CommandError {
    match error {
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
            "input could not be deserialized as the selected generated message",
        ),
    }
}
