use rust_iso20022_wasm::{api, dto::ErrorCode};

const CANARY: &str = "SECRET-ACCOUNT-DE89370400440532013000";

fn sized_message(bytes: usize) -> String {
    const PREFIX: &str = r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">"#;
    const SUFFIX: &str = "</Document>";
    const COMMENT_OVERHEAD: usize = 7;
    const MAX_COMMENT_TEXT: usize = 1024 * 1024;
    assert!(bytes >= PREFIX.len() + SUFFIX.len());
    let mut message = String::with_capacity(bytes);
    message.push_str(PREFIX);
    let mut remaining = bytes - PREFIX.len() - SUFFIX.len();
    while remaining >= COMMENT_OVERHEAD {
        let text = (remaining - COMMENT_OVERHEAD).min(MAX_COMMENT_TEXT);
        message.push_str("<!--");
        message.push_str(&"x".repeat(text));
        message.push_str("-->");
        remaining -= text + COMMENT_OVERHEAD;
    }
    message.push_str(&" ".repeat(remaining));
    message.push_str(SUFFIX);
    assert_eq!(message.len(), bytes);
    message
}

#[test]
fn malformed_oversized_and_unknown_inputs_are_bounded_and_redacted() {
    let malformed = format!("<Document>{CANARY}");
    let error = api::detect_message(&malformed).unwrap_err();
    assert_eq!(error.code, ErrorCode::InvalidInput);
    assert!(!serde_json::to_string(&error).unwrap().contains(CANARY));

    let oversized = format!(
        "<Document>{CANARY}{}</Document>",
        "x".repeat(rust_iso20022::ParseLimits::DEFAULT.max_input_bytes + 1)
    );
    let error = api::parse_message(&oversized).unwrap_err();
    assert_eq!(error.code, ErrorCode::InvalidInput);
    assert!(!serde_json::to_string(&error).unwrap().contains(CANARY));
}

#[test]
fn wasm_message_limit_is_inclusive_at_the_core_default() {
    let maximum = rust_iso20022::ParseLimits::DEFAULT.max_input_bytes;
    for length in [maximum - 1, maximum] {
        assert!(api::detect_message(&sized_message(length)).is_ok());
    }
    let error = api::detect_message(&sized_message(maximum + 1)).unwrap_err();
    assert_eq!(error.code, ErrorCode::InvalidInput);
}

#[test]
fn unsupported_generated_families_fail_before_deserialization() {
    let xml = r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:camt.053.001.09"/>"#;
    let error = api::parse_message(xml).unwrap_err();
    assert_eq!(error.code, ErrorCode::UnsupportedFeature);
    assert_eq!(error.required_feature.as_deref(), Some("model-camt"));
}

#[test]
fn unavailable_validation_layers_are_explicit() {
    let error = api::validate_message("not parsed for unavailable layers", "l1").unwrap_err();
    assert_eq!(error.code, ErrorCode::ValidationUnavailable);
    assert_eq!(error.message, "requested validation layer is unavailable");
}
