//! WebAssembly / JavaScript bindings for the identification, catalogue,
//! metadata and Business Application Header layers.
//!
//! These are compiled only for `wasm32` builds done with `--cfg direct_wasm`
//! (see `scripts/build-wasm.sh`), so they never affect normal native builds.
//! The generated message model is intentionally not exposed to JS (502 large
//! types); the data/identification layer is what is useful in the browser.

#![cfg(all(direct_wasm, target_arch = "wasm32"))]

use js_sys::Array;
use wasm_bindgen::prelude::*;

/// Detect the message name from an XML document, e.g. `"pacs.008.001.08"`.
#[wasm_bindgen]
pub fn detect(xml: &str) -> Option<String> {
    crate::detect(xml).map(|id| id.message_name())
}

/// Parse a namespace or bare name into the canonical message name.
#[wasm_bindgen]
pub fn from_namespace(namespace: &str) -> Option<String> {
    crate::from_namespace(namespace).map(|id| id.message_name())
}

/// The XSD namespace for a message name, or `undefined`.
#[wasm_bindgen]
pub fn namespace_of(message_name: &str) -> Option<String> {
    crate::catalogue::from_message_name(message_name).map(|e| e.namespace.to_string())
}

/// The business-area code of a message name, e.g. `"pacs"`.
#[wasm_bindgen]
pub fn business_area_of(message_name: &str) -> Option<String> {
    crate::catalogue::from_message_name(message_name).map(|e| e.business_area.to_string())
}

/// Whether the catalogue contains the given message name.
#[wasm_bindgen]
pub fn catalogue_contains(message_name: &str) -> bool {
    crate::catalogue::contains(message_name)
}

/// Every message name in the catalogue, as a JS array of strings.
#[wasm_bindgen]
pub fn catalogue_all() -> Array {
    crate::catalogue::all()
        .iter()
        .map(|e| JsValue::from_str(e.message_name))
        .collect()
}

/// Human-readable description of a business-area code, e.g. `"pacs"` →
/// `"Payments Clearing and Settlement"`.
#[wasm_bindgen]
pub fn business_area_description(code: &str) -> Option<String> {
    crate::BusinessArea::from_code(code).map(|a| a.description().to_string())
}

/// Extract business metadata from a message, as a JSON string.
#[wasm_bindgen]
pub fn extract_metadata_json(xml: &str) -> String {
    let m = crate::metadata::extract(xml);
    // Hand-rolled JSON to avoid pulling serde_json into the wasm build.
    fn f(k: &str, v: &Option<String>) -> String {
        match v {
            Some(x) => format!("\"{k}\":\"{}\"", x.replace('"', "\\\"")),
            None => format!("\"{k}\":null"),
        }
    }
    format!(
        "{{{},{},{},{},{},{}}}",
        f("message_id", &m.message_id),
        f("creation_date_time", &m.creation_date_time),
        f("number_of_transactions", &m.number_of_transactions),
        f("amount", &m.amount),
        f("currency", &m.currency),
        f("value_date", &m.value_date),
    )
}
