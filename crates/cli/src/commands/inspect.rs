use serde::Serialize;

use super::CommandError;

#[derive(Debug, Serialize)]
pub struct InspectData {
    pub message_id: String,
    pub business_area: String,
    pub family: String,
    pub version: String,
    pub namespace: &'static str,
    pub root_element: &'static str,
    pub description: &'static str,
    pub generated_module: &'static str,
    pub required_feature: &'static str,
    pub field_count: usize,
}

pub fn inspect(xml: &str) -> Result<InspectData, CommandError> {
    let detected = rust_iso20022::detect_message(xml, rust_iso20022::ParseLimits::DEFAULT)
        .map_err(|error| match error {
            rust_iso20022::DetectionError::UnknownMessage => {
                CommandError::parse("unknown_message", "unknown ISO 20022 message")
            }
            rust_iso20022::DetectionError::RootMismatch { .. } => CommandError::parse(
                "root_mismatch",
                "message namespace is attached to an unexpected root element",
            ),
            rust_iso20022::DetectionError::Xml(_) => {
                CommandError::parse("malformed_xml", "XML input is malformed or forbidden")
            }
        })?;
    let identifier = detected.message_id();
    let descriptor = detected.descriptor();
    Ok(InspectData {
        message_id: identifier.as_str().to_owned(),
        business_area: identifier.business_area().to_owned(),
        family: identifier.family().to_owned(),
        version: identifier.version().as_str().to_owned(),
        namespace: descriptor.namespace,
        root_element: descriptor.root_element,
        description: descriptor.description.unwrap_or(""),
        generated_module: descriptor.generated_module,
        required_feature: descriptor.required_feature,
        field_count: descriptor.fields.len(),
    })
}
