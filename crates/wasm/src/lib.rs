//! Bounded, serde-backed WebAssembly adapter over canonical generated messages.
//!
//! Validation results cover only implemented rules; they do not guarantee bank
//! or network acceptance, certification, onboarding, or legal compliance.

/// Native-testable core-to-WASM operation mappings with typed adapter errors.
pub mod api;
/// Compatibility views for the legacy root-crate WASM surface.
pub mod compat;
/// Serde DTOs returned to JavaScript without a second message model.
pub mod dto;

#[cfg(target_arch = "wasm32")]
mod bindings {
    use wasm_bindgen::prelude::*;

    fn output<T: serde::Serialize>(
        result: Result<T, crate::dto::WasmError>,
    ) -> Result<JsValue, JsValue> {
        match result {
            Ok(value) => serde_wasm_bindgen::to_value(&value)
                .map_err(|_| JsValue::from_str("serialization_failure")),
            Err(error) => Err(serde_wasm_bindgen::to_value(&error)
                .unwrap_or_else(|_| JsValue::from_str("serialization_failure"))),
        }
    }

    #[wasm_bindgen(start)]
    pub fn start() {
        rust_iso20022::privacy::install_redacted_panic_hook();
    }

    #[wasm_bindgen]
    pub fn detect_message(xml: &str) -> Result<JsValue, JsValue> {
        output(crate::api::detect_message(xml))
    }

    #[wasm_bindgen]
    pub fn catalogue_message(message_id: &str) -> Result<JsValue, JsValue> {
        output(crate::api::catalogue_message(message_id))
    }

    #[wasm_bindgen]
    #[cfg(feature = "model-pacs")]
    pub fn parse_message(xml: &str) -> Result<JsValue, JsValue> {
        output(crate::api::parse_message(xml))
    }

    #[wasm_bindgen]
    #[cfg(feature = "model-pacs")]
    pub fn serialize_message(message_id: &str, document: JsValue) -> Result<JsValue, JsValue> {
        let document = serde_wasm_bindgen::from_value(document).map_err(|_| {
            serde_wasm_bindgen::to_value(&crate::dto::WasmError {
                code: crate::dto::ErrorCode::InvalidInput,
                message: "message JSON is malformed or unsupported".to_owned(),
                required_feature: None,
            })
            .unwrap_or_else(|_| JsValue::from_str("invalid_input"))
        })?;
        output(crate::api::serialize_message(message_id, &document))
    }

    #[wasm_bindgen]
    #[cfg(feature = "model-pacs")]
    pub fn validate_message(xml: &str, layer: &str) -> Result<JsValue, JsValue> {
        output(crate::api::validate_message(xml, layer))
    }
}
