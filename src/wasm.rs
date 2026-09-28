//! WebAssembly / JavaScript bindings for the identification, catalogue,
//! Business Application Header and metadata layers.
//!
//! Compiled only for `wasm32` builds done with `--cfg direct_wasm` (see
//! `scripts/build-wasm.sh`), so they never affect native builds. The generated
//! message model is intentionally not exposed to JS (722 large types); the
//! data/identification layer is what is useful in the browser. Structured values
//! are returned as JSON strings (`JSON.parse` them on the JS side).

#![cfg(all(direct_wasm, target_arch = "wasm32", feature = "serde"))]

use js_sys::Array;
use wasm_bindgen::prelude::*;

/// Module entry point: install a panic hook that suppresses potentially
/// sensitive panic payloads. Runs automatically on module load.
#[wasm_bindgen(start)]
pub fn start() {
    crate::privacy::install_redacted_panic_hook();
}

// ----------------------------------------------------------------- identity ---

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

/// Structured identification as JSON:
/// `{messageName, namespace, businessArea, functionality, variant, version}`.
#[wasm_bindgen]
pub fn mx_id(namespace_or_name: &str) -> Option<String> {
    crate::wasm_compat::mx_id(namespace_or_name)
}

// ----------------------------------------------------------- business areas ---

/// Human-readable description of a business-area code, e.g. `"pacs"` →
/// `"Payments Clearing and Settlement"`.
#[wasm_bindgen]
pub fn business_area_description(code: &str) -> Option<String> {
    crate::BusinessArea::from_code(code).map(|a| a.description().to_string())
}

/// Every business-area code, as a JS array of strings.
#[wasm_bindgen]
pub fn business_areas() -> Array {
    crate::BusinessArea::ALL
        .iter()
        .map(|a| JsValue::from_str(a.code()))
        .collect()
}

// ---------------------------------------------------------------- catalogue ---

/// Whether the catalogue contains the given message name.
#[wasm_bindgen]
pub fn catalogue_contains(message_name: &str) -> bool {
    crate::catalogue::contains(message_name)
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

/// Whether a typed model exists for the message.
#[wasm_bindgen]
pub fn has_model(message_name: &str) -> bool {
    crate::catalogue::from_message_name(message_name)
        .map(|e| e.has_model)
        .unwrap_or(false)
}

/// A catalogue entry as JSON: `{messageName, namespace, businessArea, hasModel}`.
#[wasm_bindgen]
pub fn catalogue_entry(message_name: &str) -> Option<String> {
    crate::wasm_compat::catalogue_entry(message_name)
}

/// Every message name in the catalogue, as a JS array of strings.
#[wasm_bindgen]
pub fn catalogue_all() -> Array {
    crate::catalogue::all()
        .iter()
        .map(|e| JsValue::from_str(e.message_name))
        .collect()
}

// ------------------------------------------------------- headers / metadata ---

/// Business Application Header fields as JSON, or `undefined` if no header.
#[wasm_bindgen]
pub fn parse_app_hdr(xml: &str) -> Option<String> {
    crate::app_hdr::parse_business_header(xml).map(|header| crate::wasm_compat::header(&header))
}

/// Build a `head.001` `<AppHdr>` XML from header fields.
#[wasm_bindgen]
pub fn build_app_hdr(
    from: Option<String>,
    to: Option<String>,
    biz_msg_idr: Option<String>,
    msg_def_idr: Option<String>,
    cre_dt: Option<String>,
) -> String {
    crate::app_hdr::BusinessHeader {
        from,
        to,
        biz_msg_idr,
        msg_def_idr,
        cre_dt,
    }
    .to_app_hdr_xml()
}

/// Extract business metadata from a message, as JSON.
#[wasm_bindgen]
pub fn extract_metadata(xml: &str) -> String {
    crate::wasm_compat::metadata(&crate::metadata::extract(xml))
}

/// Read a full business message (header + detected type + metadata) as JSON:
/// `{messageName, header, metadata}`.
#[wasm_bindgen]
pub fn read_business_message(xml: &str) -> String {
    crate::wasm_compat::business_message(&crate::read_business_message(xml))
}

// --------------------------------------------------------------- generic tree ---

/// Text at a `/`-separated path of local element names, e.g.
/// `"FIToFICstmrCdtTrf/GrpHdr/MsgId"` — read any message without the model.
#[wasm_bindgen]
pub fn node_text(xml: &str, path: &str) -> Option<String> {
    let root = crate::MxNode::parse(xml)?;
    let segs: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    root.at(&segs).and_then(|n| n.text()).map(str::to_string)
}

/// Text of the first descendant element with the given local name.
#[wasm_bindgen]
pub fn node_find(xml: &str, local: &str) -> Option<String> {
    let root = crate::MxNode::parse(xml)?;
    root.find(local).and_then(|n| n.text()).map(str::to_string)
}

/// Value of an attribute on the element at a `/`-separated path of local names,
/// e.g. the `Ccy` of `"FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt"`.
#[wasm_bindgen]
pub fn node_attr(xml: &str, path: &str, attr: &str) -> Option<String> {
    let root = crate::MxNode::parse(xml)?;
    let segs: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    root.at(&segs)
        .and_then(|n| n.attr(attr))
        .map(str::to_string)
}

/// Value of an attribute on the first descendant element with the given local
/// name, e.g. `node_find_attr(xml, "IntrBkSttlmAmt", "Ccy")` → `"EUR"`.
#[wasm_bindgen]
pub fn node_find_attr(xml: &str, local: &str, attr: &str) -> Option<String> {
    let root = crate::MxNode::parse(xml)?;
    root.find(local)
        .and_then(|n| n.attr(attr))
        .map(str::to_string)
}

/// The whole parsed message tree as JSON, recursively:
/// `{name, value, attributes: {…}, children: [...]}`. Lets JS walk any message
/// without the typed model.
#[wasm_bindgen]
pub fn node_to_json(xml: &str) -> Option<String> {
    crate::MxNode::parse(xml)
        .as_ref()
        .map(crate::wasm_compat::node)
}

/// The texts of every descendant element with the given local name.
#[wasm_bindgen]
pub fn node_find_all(xml: &str, local: &str) -> Array {
    match crate::MxNode::parse(xml) {
        Some(root) => root
            .find_all(local)
            .into_iter()
            .filter_map(|n| n.text())
            .map(JsValue::from_str)
            .collect(),
        None => Array::new(),
    }
}
