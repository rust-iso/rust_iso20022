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

/// Module entry point: install a panic hook that routes Rust panics to
/// `console.error`, so wasm failures show a readable message and stack trace
/// instead of an opaque `unreachable`. Runs automatically on module load.
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
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
    root.at(&segs).and_then(|n| n.attr(attr)).map(str::to_string)
}

/// Value of an attribute on the first descendant element with the given local
/// name, e.g. `node_find_attr(xml, "IntrBkSttlmAmt", "Ccy")` → `"EUR"`.
#[wasm_bindgen]
pub fn node_find_attr(xml: &str, local: &str, attr: &str) -> Option<String> {
    let root = crate::MxNode::parse(xml)?;
    root.find(local).and_then(|n| n.attr(attr)).map(str::to_string)
}

/// The whole parsed message tree as JSON, recursively:
/// `{name, value, attributes: {…}, children: [...]}`. Lets JS walk any message
/// without the typed model.
#[wasm_bindgen]
pub fn node_to_json(xml: &str) -> Option<String> {
    crate::MxNode::parse(xml).as_ref().map(json_node)
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

/// Recursively JSON-encode an `MxNode` tree.
fn json_node(n: &crate::MxNode) -> String {
    let mut out = String::from("{\"name\":");
    out.push_str(&js(&n.name));
    out.push_str(",\"value\":");
    out.push_str(&jso(&n.value));
    out.push_str(",\"attributes\":{");
    for (i, (k, v)) in n.attributes.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&js(k));
        out.push(':');
        out.push_str(&js(v));
    }
    out.push_str("},\"children\":[");
    for (i, c) in n.children.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&json_node(c));
    }
    out.push_str("]}");
    out
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
