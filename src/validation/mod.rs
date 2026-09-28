//! Structured, layered and explainable validation primitives.
//!
//! This API validates canonical generated values through rules and metadata; it
//! does not define another ISO 20022 message model. The legacy XSD facet
//! [`crate::validate::Validate`] trait remains available independently.

#[cfg(feature = "model-pacs")]
pub mod bindings;
pub mod cross_field;
mod engine;
mod financial;
mod registry;
mod report;
mod rule;

pub use engine::{LayerAvailability, ValidationContext, ValidationPipeline, ValidationTarget};
pub use financial::financial_rules;
pub use registry::{DuplicateRuleId, RuleRegistry};
pub use report::{
    FieldPath, LayerUnavailable, RuleId, Severity, ValidationIssue, ValidationLayer,
    ValidationReport,
};
pub use rule::{Rule, RuleDescriptor, RuleExplanation, RuleSet, RuleSetError};
