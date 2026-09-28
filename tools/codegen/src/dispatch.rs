//! Generation entry point for feature-gated `AnyMessage` dispatch.
//!
//! The emitted enum stores each schema's concrete generated root type and adds
//! typed borrow/ownership accessors. It never defines another message model.

use std::path::Path;

pub type DispatchMessage = (String, String, String, String);

pub fn write(output: &Path, messages: &[DispatchMessage]) -> Result<(), String> {
    super::write_any(output, messages)
}
