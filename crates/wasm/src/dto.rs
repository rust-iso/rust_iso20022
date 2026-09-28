//! Structured WebAssembly result and error DTOs.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    InvalidInput,
    UnsupportedFeature,
    ValidationUnavailable,
    SerializationFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WasmError {
    pub code: ErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_feature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DetectionOutput {
    pub message_id: String,
    pub business_area: String,
    pub family: String,
    pub version: String,
    pub namespace: String,
    pub root_element: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CatalogueOutput {
    pub message_id: String,
    pub namespace: String,
    pub business_area: String,
    pub root_element: String,
    pub required_feature: String,
    pub schema_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedOutput {
    pub message_id: String,
    pub document: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct XmlOutput {
    pub message_id: String,
    pub xml: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationIssueOutput {
    pub code: String,
    pub severity: String,
    pub path: String,
    pub message: String,
    pub rule_id: String,
    pub layer: String,
    pub profile: Option<String>,
    pub profile_version: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationOutput {
    pub message_id: String,
    pub layer: String,
    pub valid: bool,
    pub complete: bool,
    pub errors: Vec<ValidationIssueOutput>,
    pub warnings: Vec<ValidationIssueOutput>,
}
