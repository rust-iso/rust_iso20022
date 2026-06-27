//! WebAssembly / JavaScript bindings for the identification, catalogue,
//! Business Application Header and metadata layers.
//!
//! Compiled only for `wasm32` builds done with `--cfg direct_wasm` (see
//! `scripts/build-wasm.sh`), so they never affect native builds. The generated
//! message model is intentionally not exposed to JS (722 large types); the
//! data/identification layer is what is useful in the browser. Structured values
//! are returned as JSON strings (`JSON.parse` them on the JS side).

#![cfg(all(direct_wasm, target_arch = "wasm32"))]

use js_sys::Array;
use wasm_bindgen::prelude::*;

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
    let id = crate::MxId::parse(namespace_or_name).ok()?;
    Some(format!(
        "{{\"messageName\":{},\"namespace\":{},\"businessArea\":{},\"functionality\":{},\"variant\":{},\"version\":{}}}",
        js(&id.message_name()),
        js(&id.namespace()),
        js(id.business_area.code()),
        js(&id.functionality),
        js(&id.variant),
        js(&id.version),
    ))
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
    let e = crate::catalogue::from_message_name(message_name)?;
    Some(format!(
        "{{\"messageName\":{},\"namespace\":{},\"businessArea\":{},\"hasModel\":{}}}",
        js(e.message_name),
        js(e.namespace),
        js(e.business_area),
        e.has_model,
    ))
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
    crate::app_hdr::parse_business_header(xml).map(|h| json_header(&h))
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
    json_metadata(&crate::metadata::extract(xml))
}

/// Read a full business message (header + detected type + metadata) as JSON:
/// `{messageName, header, metadata}`.
#[wasm_bindgen]
pub fn read_business_message(xml: &str) -> String {
    let bm = crate::read_business_message(xml);
    format!(
        "{{\"messageName\":{},\"header\":{},\"metadata\":{}}}",
        bm.id.map(|i| js(&i.message_name())).unwrap_or_else(|| "null".into()),
        bm.header
            .map(|h| json_header(&h))
            .unwrap_or_else(|| "null".into()),
        json_metadata(&bm.metadata),
    )
}

// -------------------------------------------------------------------- utils ---

/// JSON-encode a string value (with quotes and minimal escaping).
fn js(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// JSON-encode an optional string field.
fn jso(v: &Option<String>) -> String {
    match v {
        Some(s) => js(s),
        None => "null".to_string(),
    }
}

fn json_header(h: &crate::app_hdr::BusinessHeader) -> String {
    format!(
        "{{\"from\":{},\"to\":{},\"bizMsgIdr\":{},\"msgDefIdr\":{},\"creDt\":{}}}",
        jso(&h.from),
        jso(&h.to),
        jso(&h.biz_msg_idr),
        jso(&h.msg_def_idr),
        jso(&h.cre_dt),
    )
}

fn json_metadata(m: &crate::metadata::MessageMetadata) -> String {
    format!(
        "{{\"messageId\":{},\"creationDateTime\":{},\"numberOfTransactions\":{},\"amount\":{},\"currency\":{},\"valueDate\":{}}}",
        jso(&m.message_id),
        jso(&m.creation_date_time),
        jso(&m.number_of_transactions),
        jso(&m.amount),
        jso(&m.currency),
        jso(&m.value_date),
    )
}
