//! Seven MCP tools mapped directly to core SDK contracts.

use rmcp::{
    Json, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::Tool,
    tool, tool_handler, tool_router,
};

use crate::dto::*;
use rust_iso20022::metadata::{FieldDescriptor, MessageDescriptor};
use rust_iso20022::validation::{Severity, ValidationLayer};

const UNKNOWN_MESSAGE: &str = "unknown ISO 20022 message";
const INVALID_MESSAGE: &str = "message is malformed, forbidden, or unsupported";
const UNAVAILABLE_VALIDATION: &str = "requested validation layer or profile release is unavailable";

#[derive(Debug, Clone)]
pub struct Iso20022Mcp {
    tool_router: ToolRouter<Self>,
}

impl Default for Iso20022Mcp {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router(router = tool_router)]
impl Iso20022Mcp {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    pub fn listed_tools(&self) -> Vec<Tool> {
        self.tool_router.list_all()
    }

    #[tool(description = "Detect an in-memory ISO 20022 XML message using bounded core parsing")]
    pub fn detect_message(
        &self,
        Parameters(input): Parameters<MessageInput>,
    ) -> Result<Json<DetectOutput>, String> {
        let detected =
            rust_iso20022::detect_message(&input.message, rust_iso20022::ParseLimits::DEFAULT)
                .map_err(|_| INVALID_MESSAGE.to_owned())?;
        let id = detected.message_id();
        Ok(Json(DetectOutput {
            message_id: id.as_str().to_owned(),
            business_area: id.business_area().to_owned(),
            family: id.family().to_owned(),
            version: id.version().as_str().to_owned(),
            namespace: detected.namespace().to_owned(),
            root_element: detected.root_element().to_owned(),
        }))
    }

    #[tool(description = "Inspect schema-derived metadata for an in-memory ISO 20022 XML message")]
    pub fn inspect_message(
        &self,
        Parameters(input): Parameters<MessageInput>,
    ) -> Result<Json<MessageOutput>, String> {
        let detected =
            rust_iso20022::detect_message(&input.message, rust_iso20022::ParseLimits::DEFAULT)
                .map_err(|_| INVALID_MESSAGE.to_owned())?;
        Ok(Json(message_output(detected.descriptor())))
    }

    #[tool(
        description = "Validate an in-memory generated ISO 20022 message with an explicit layer. Implemented rules only; no bank/network acceptance, certification, onboarding, or legal-compliance guarantee."
    )]
    pub fn validate_message(
        &self,
        Parameters(input): Parameters<ValidateInput>,
    ) -> Result<Json<ValidationOutput>, String> {
        if let Some(profile_release) = input.profile_release.as_deref() {
            rust_iso20022::profiles::ProfileReleaseKey::parse(profile_release)
                .map_err(|_| "profile release key is incomplete or invalid".to_owned())?;
            return Err(UNAVAILABLE_VALIDATION.to_owned());
        }
        if !matches!(
            input.layer.as_deref().unwrap_or("l2"),
            "l2" | "iso" | "iso-semantic"
        ) {
            return Err(UNAVAILABLE_VALIDATION.to_owned());
        }
        rust_iso20022::detect_message(&input.message, rust_iso20022::ParseLimits::DEFAULT)
            .map_err(|_| INVALID_MESSAGE.to_owned())?;
        let parsed =
            rust_iso20022::parse(&input.message).map_err(|_| INVALID_MESSAGE.to_owned())?;
        let message_id = parsed.message_id().as_str().to_owned();
        let report = rust_iso20022::validation::bindings::validate_parsed_message(&parsed)
            .map_err(|_| UNAVAILABLE_VALIDATION.to_owned())?;
        let issues: Vec<_> = report.issues().iter().map(issue_output).collect();
        Ok(Json(ValidationOutput {
            message_id,
            layer: "iso_semantic".to_owned(),
            valid: report.valid(),
            complete: report.complete(),
            errors: issues
                .iter()
                .filter(|issue| issue.severity == "error")
                .cloned()
                .collect(),
            warnings: issues
                .iter()
                .filter(|issue| issue.severity == "warning")
                .cloned()
                .collect(),
            unavailable_layers: report
                .unavailable_layers()
                .iter()
                .map(|entry| UnavailableLayerOutput {
                    layer: layer_name(entry.layer).to_owned(),
                    reason: entry.reason.clone(),
                })
                .collect(),
        }))
    }

    #[tool(description = "Look up one exact message version in the generated catalogue")]
    pub fn lookup_message(
        &self,
        Parameters(input): Parameters<MessageIdInput>,
    ) -> Result<Json<MessageOutput>, String> {
        rust_iso20022::catalogue::lookup_descriptor(&input.message_id)
            .map(message_output)
            .map(Json)
            .ok_or_else(|| UNKNOWN_MESSAGE.to_owned())
    }

    #[tool(description = "Look up one exact normalized schema field path")]
    pub fn lookup_field(
        &self,
        Parameters(input): Parameters<FieldInput>,
    ) -> Result<Json<FieldOutput>, String> {
        rust_iso20022::catalogue::lookup_field(&input.message_id, &input.logical_path)
            .map(|field| Json(field_output(&input.message_id, field)))
            .ok_or_else(|| "message field is not registered".to_owned())
    }

    #[tool(description = "Compare two exact same-family message versions using schema metadata")]
    pub fn compare_versions(
        &self,
        Parameters(input): Parameters<CompareInput>,
    ) -> Result<Json<CompareOutput>, String> {
        let diff =
            rust_iso20022::compare::compare_versions(&input.from_message_id, &input.to_message_id)
                .map_err(|_| "message versions are unknown or not comparable".to_owned())?;
        Ok(Json(CompareOutput {
            from: VersionEndpointOutput {
                identity: diff.from.identity.to_owned(),
                schema_sha256: diff.from.schema_sha256.to_owned(),
            },
            to: VersionEndpointOutput {
                identity: diff.to.identity.to_owned(),
                schema_sha256: diff.to.schema_sha256.to_owned(),
            },
            changes: diff
                .changes
                .into_iter()
                .map(|change| SchemaChangeOutput {
                    path: change.path,
                    kind: change_kind(change.kind).to_owned(),
                    before: change.before,
                    after: change.after,
                })
                .collect(),
        }))
    }

    #[tool(description = "Explain a registered validation rule from its executable core registry")]
    pub fn explain_validation_error(
        &self,
        Parameters(input): Parameters<ExplainInput>,
    ) -> Result<Json<ExplainOutput>, String> {
        let registry = rust_iso20022::validation::RuleRegistry::new(
            rust_iso20022::validation::financial_rules(),
        )
        .map_err(|_| "validation registry is unavailable".to_owned())?;
        let explanation = registry
            .explain_details_str(&input.rule_id)
            .ok_or_else(|| "validation rule is not registered".to_owned())?;
        let descriptor = explanation.descriptor;
        Ok(Json(ExplainOutput {
            id: descriptor.id.as_str().to_owned(),
            layer: layer_name(descriptor.layer).to_owned(),
            title: descriptor.title.to_owned(),
            reason: descriptor.reason.to_owned(),
            source: descriptor.source.map(str::to_owned),
            profile: descriptor.profile.map(str::to_owned),
            profile_version: descriptor.profile_version.map(str::to_owned),
            affected_messages: descriptor
                .affected_messages
                .iter()
                .map(|value| (*value).to_owned())
                .collect(),
            field_paths: explanation
                .field_paths
                .iter()
                .map(|value| (*value).to_owned())
                .collect(),
        }))
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for Iso20022Mcp {}

fn message_output(descriptor: &MessageDescriptor) -> MessageOutput {
    MessageOutput {
        message_id: descriptor.identity.to_owned(),
        business_area: descriptor.business_area.to_owned(),
        namespace: descriptor.namespace.to_owned(),
        root_element: descriptor.root_element.to_owned(),
        root_type: descriptor.root_type.to_owned(),
        description: descriptor.description.map(str::to_owned),
        schema_path: descriptor.schema_path.to_owned(),
        schema_sha256: descriptor.schema_sha256.to_owned(),
        generated_module: descriptor.generated_module.to_owned(),
        required_feature: descriptor.required_feature.to_owned(),
        field_count: descriptor.fields.len(),
    }
}

fn field_output(message_id: &str, field: &FieldDescriptor) -> FieldOutput {
    FieldOutput {
        message_id: message_id.to_owned(),
        logical_path: field.logical_path.to_owned(),
        namespace: field.namespace.to_owned(),
        wire_name: field.wire_name.to_owned(),
        rust_name: field.rust_name.to_owned(),
        type_name: field.type_name.to_owned(),
        min_occurs: field.min_occurs,
        max_occurs: field.max_occurs,
        enum_values: field
            .enum_values
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
        documentation: field.documentation.map(str::to_owned),
        choice_group: field.choice_group,
        kind: field_kind(field.kind).to_owned(),
    }
}

fn issue_output(issue: &rust_iso20022::validation::ValidationIssue) -> ValidationIssueOutput {
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
        layer: layer_name(issue.layer).to_owned(),
        profile: issue.profile.map(str::to_owned),
        profile_version: issue.profile_version.map(str::to_owned),
        source: issue.source.map(str::to_owned),
    }
}

fn layer_name(layer: ValidationLayer) -> &'static str {
    match layer {
        ValidationLayer::SyntaxSchema => "syntax_schema",
        ValidationLayer::IsoSemantic => "iso_semantic",
        ValidationLayer::Profile => "profile",
    }
}

fn field_kind(kind: rust_iso20022::metadata::FieldKind) -> &'static str {
    use rust_iso20022::metadata::FieldKind;
    match kind {
        FieldKind::Element => "element",
        FieldKind::Attribute => "attribute",
        FieldKind::SimpleContent => "simple_content",
    }
}

fn change_kind(kind: rust_iso20022::compare::SchemaChangeKind) -> &'static str {
    use rust_iso20022::compare::SchemaChangeKind;
    match kind {
        SchemaChangeKind::FieldAdded => "field_added",
        SchemaChangeKind::FieldRemoved => "field_removed",
        SchemaChangeKind::TypeChanged => "type_changed",
        SchemaChangeKind::CardinalityChanged => "cardinality_changed",
        SchemaChangeKind::EnumValueAdded => "enum_value_added",
        SchemaChangeKind::EnumValueRemoved => "enum_value_removed",
    }
}
