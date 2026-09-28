//! Thin WASM operation mappings over bounded core SDK calls.

#[cfg(feature = "model-pacs")]
use rust_iso20022::validation::{Severity, ValidationIssue, ValidationLayer};

use crate::dto::*;

const INVALID_INPUT: &str = "message is malformed, forbidden, oversized, or unsupported";
#[cfg(feature = "model-pacs")]
const JSON_LIMIT: usize = rust_iso20022::ParseLimits::DEFAULT.max_input_bytes;

pub fn detect_message(xml: &str) -> Result<DetectionOutput, WasmError> {
    let detected = rust_iso20022::detect_message(xml, rust_iso20022::ParseLimits::DEFAULT)
        .map_err(|_| invalid_input())?;
    let id = detected.message_id();
    Ok(DetectionOutput {
        message_id: id.as_str().to_owned(),
        business_area: id.business_area().to_owned(),
        family: id.family().to_owned(),
        version: id.version().as_str().to_owned(),
        namespace: detected.namespace().to_owned(),
        root_element: detected.root_element().to_owned(),
    })
}

pub fn catalogue_message(message_id: &str) -> Result<CatalogueOutput, WasmError> {
    let descriptor =
        rust_iso20022::catalogue::lookup_descriptor(message_id).ok_or_else(invalid_input)?;
    Ok(CatalogueOutput {
        message_id: descriptor.identity.to_owned(),
        namespace: descriptor.namespace.to_owned(),
        business_area: descriptor.business_area.to_owned(),
        root_element: descriptor.root_element.to_owned(),
        required_feature: descriptor.required_feature.to_owned(),
        schema_sha256: descriptor.schema_sha256.to_owned(),
    })
}

#[cfg(feature = "model-pacs")]
pub fn parse_message(xml: &str) -> Result<ParsedOutput, WasmError> {
    detect_message(xml)?;
    let parsed = rust_iso20022::parse(xml).map_err(parse_error)?;
    let message_id = parsed.message_id().as_str().to_owned();
    let json = parsed.to_json().map_err(|_| serialization_failure())?;
    let document = serde_json::from_str(&json).map_err(|_| serialization_failure())?;
    Ok(ParsedOutput {
        message_id,
        document,
    })
}

#[cfg(feature = "model-pacs")]
pub fn serialize_message(
    message_id: &str,
    document: &serde_json::Value,
) -> Result<XmlOutput, WasmError> {
    let json = serde_json::to_string(document).map_err(|_| serialization_failure())?;
    if json.len() > JSON_LIMIT {
        return Err(invalid_input());
    }
    let parsed = rust_iso20022::parse_json(message_id, &json).map_err(parse_error)?;
    let xml = parsed.to_xml().map_err(|_| serialization_failure())?;
    Ok(XmlOutput {
        message_id: parsed.message_id().as_str().to_owned(),
        xml,
    })
}

#[cfg(feature = "model-pacs")]
pub fn validate_message(xml: &str, layer: &str) -> Result<ValidationOutput, WasmError> {
    if !matches!(layer, "l2" | "iso" | "iso-semantic") {
        return Err(WasmError {
            code: ErrorCode::ValidationUnavailable,
            message: "requested validation layer is unavailable".to_owned(),
            required_feature: None,
        });
    }
    detect_message(xml)?;
    let parsed = rust_iso20022::parse(xml).map_err(parse_error)?;
    let message_id = parsed.message_id().as_str().to_owned();
    let report =
        rust_iso20022::validation::bindings::validate_parsed_message(&parsed).map_err(|_| {
            WasmError {
                code: ErrorCode::ValidationUnavailable,
                message: "validation binding is unavailable for this message".to_owned(),
                required_feature: None,
            }
        })?;
    Ok(ValidationOutput {
        message_id,
        layer: "iso_semantic".to_owned(),
        valid: report.valid(),
        complete: report.complete(),
        errors: report.errors().into_iter().map(issue_output).collect(),
        warnings: report.warnings().into_iter().map(issue_output).collect(),
    })
}

#[cfg(feature = "model-pacs")]
fn issue_output(issue: &ValidationIssue) -> ValidationIssueOutput {
    ValidationIssueOutput {
        code: issue.code.as_str().to_owned(),
        severity: match issue.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        }
        .to_owned(),
        path: issue.path.as_str().to_owned(),
        message: issue.message.clone(),
        rule_id: issue.rule_id.as_str().to_owned(),
        layer: match issue.layer {
            ValidationLayer::SyntaxSchema => "syntax_schema",
            ValidationLayer::IsoSemantic => "iso_semantic",
            ValidationLayer::Profile => "profile",
        }
        .to_owned(),
        profile: issue.profile.map(str::to_owned),
        profile_version: issue.profile_version.map(str::to_owned),
        source: issue.source.map(str::to_owned),
    }
}

#[cfg(feature = "model-pacs")]
fn parse_error(error: rust_iso20022::ParseError) -> WasmError {
    match error {
        rust_iso20022::ParseError::UnsupportedFeature {
            required_feature, ..
        } => WasmError {
            code: ErrorCode::UnsupportedFeature,
            message: "generated message feature is unavailable".to_owned(),
            required_feature: Some(required_feature.to_owned()),
        },
        _ => invalid_input(),
    }
}

fn invalid_input() -> WasmError {
    WasmError {
        code: ErrorCode::InvalidInput,
        message: INVALID_INPUT.to_owned(),
        required_feature: None,
    }
}

#[cfg(feature = "model-pacs")]
fn serialization_failure() -> WasmError {
    WasmError {
        code: ErrorCode::SerializationFailure,
        message: "generated message serialization failed".to_owned(),
        required_feature: None,
    }
}
