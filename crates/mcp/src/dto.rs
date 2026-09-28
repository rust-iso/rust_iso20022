//! MCP input/output DTOs and their generated JSON Schemas.

use rmcp::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MessageInput {
    /// Complete in-memory ISO 20022 XML document. File paths and URLs are not accepted.
    pub message: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ValidateInput {
    /// Complete in-memory ISO 20022 XML document.
    pub message: String,
    /// Explicit layer: `l2`; unavailable layers fail rather than degrade.
    #[serde(default)]
    pub layer: Option<String>,
    /// Complete immutable profile release key. No implicit latest release.
    #[serde(default)]
    pub profile_release: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MessageIdInput {
    pub message_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FieldInput {
    pub message_id: String,
    pub logical_path: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CompareInput {
    pub from_message_id: String,
    pub to_message_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExplainInput {
    pub rule_id: String,
}

#[derive(Debug, Serialize, JsonSchema, PartialEq, Eq)]
pub struct DetectOutput {
    pub message_id: String,
    pub business_area: String,
    pub family: String,
    pub version: String,
    pub namespace: String,
    pub root_element: String,
}

#[derive(Debug, Serialize, JsonSchema, PartialEq, Eq)]
pub struct MessageOutput {
    pub message_id: String,
    pub business_area: String,
    pub namespace: String,
    pub root_element: String,
    pub root_type: String,
    pub description: Option<String>,
    pub schema_path: String,
    pub schema_sha256: String,
    pub generated_module: String,
    pub required_feature: String,
    pub field_count: usize,
}

#[derive(Debug, Serialize, JsonSchema, PartialEq, Eq)]
pub struct FieldOutput {
    pub message_id: String,
    pub logical_path: String,
    pub namespace: String,
    pub wire_name: String,
    pub rust_name: String,
    pub type_name: String,
    pub min_occurs: u32,
    pub max_occurs: Option<u32>,
    pub enum_values: Vec<String>,
    pub documentation: Option<String>,
    pub choice_group: Option<u32>,
    pub kind: String,
}

#[derive(Debug, Serialize, JsonSchema, PartialEq, Eq)]
pub struct ValidationOutput {
    pub message_id: String,
    pub layer: String,
    pub valid: bool,
    pub complete: bool,
    pub errors: Vec<ValidationIssueOutput>,
    pub warnings: Vec<ValidationIssueOutput>,
    pub unavailable_layers: Vec<UnavailableLayerOutput>,
}

#[derive(Debug, Clone, Serialize, JsonSchema, PartialEq, Eq)]
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

#[derive(Debug, Serialize, JsonSchema, PartialEq, Eq)]
pub struct UnavailableLayerOutput {
    pub layer: String,
    pub reason: String,
}

#[derive(Debug, Serialize, JsonSchema, PartialEq, Eq)]
pub struct CompareOutput {
    pub from: VersionEndpointOutput,
    pub to: VersionEndpointOutput,
    pub changes: Vec<SchemaChangeOutput>,
}

#[derive(Debug, Serialize, JsonSchema, PartialEq, Eq)]
pub struct VersionEndpointOutput {
    pub identity: String,
    pub schema_sha256: String,
}

#[derive(Debug, Serialize, JsonSchema, PartialEq, Eq)]
pub struct SchemaChangeOutput {
    pub path: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

#[derive(Debug, Serialize, JsonSchema, PartialEq, Eq)]
pub struct ExplainOutput {
    pub id: String,
    pub layer: String,
    pub title: String,
    pub reason: String,
    pub source: Option<String>,
    pub profile: Option<String>,
    pub profile_version: Option<String>,
    pub affected_messages: Vec<String>,
    pub field_paths: Vec<String>,
}
