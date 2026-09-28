use rmcp::handler::server::wrapper::Parameters;
use rust_iso20022_mcp::{
    Iso20022Mcp,
    dto::{MessageInput, ValidateInput},
};

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

fn error_of<T>(result: Result<T, String>) -> String {
    match result {
        Ok(_) => panic!("request unexpectedly succeeded"),
        Err(error) => error,
    }
}

#[test]
fn manifest_and_adapter_exclude_prohibited_capabilities() {
    let manifest = include_str!("../Cargo.toml");
    for forbidden in [
        "reqwest",
        "hyper",
        "transport-streamable-http",
        "client",
        "auth",
    ] {
        assert!(
            !manifest.contains(forbidden),
            "forbidden dependency/capability: {forbidden}"
        );
    }

    let source = concat!(
        include_str!("../src/lib.rs"),
        include_str!("../src/tools.rs"),
        include_str!("../src/main.rs"),
    );
    for forbidden in [
        "std::fs",
        "tokio::fs",
        "std::net",
        "tokio::net",
        "std::process",
        "tokio::process",
        "Command::new",
        "println!",
        "eprintln!",
        "tracing::",
    ] {
        assert!(
            !source.contains(forbidden),
            "forbidden adapter capability: {forbidden}"
        );
    }
    assert!(source.contains("new_with_max_length(MAX_REQUEST_BYTES)"));
    assert!(source.contains("ParseLimits::DEFAULT.max_input_bytes"));
}

#[test]
fn schemas_accept_values_not_paths_urls_commands_or_uploads() {
    let tools = serde_json::to_value(Iso20022Mcp::new().listed_tools()).unwrap();
    for tool in tools.as_array().unwrap() {
        let properties = tool["inputSchema"]["properties"].as_object().unwrap();
        for name in properties.keys() {
            assert!(
                !["file_path", "filepath", "url", "command", "shell", "upload"]
                    .contains(&name.as_str()),
                "forbidden schema capability: {name}"
            );
        }
    }
}

#[test]
fn malformed_oversized_and_profile_errors_never_echo_financial_payloads() {
    let service = Iso20022Mcp::new();
    let malformed = format!("<Document>{CANARY}");
    let error = error_of(service.detect_message(Parameters(MessageInput { message: malformed })));
    assert_eq!(error, "message is malformed, forbidden, or unsupported");
    assert!(!error.contains(CANARY));

    let oversized = format!(
        "<Document>{CANARY}{}</Document>",
        "x".repeat(rust_iso20022::ParseLimits::DEFAULT.max_input_bytes + 1)
    );
    let error = error_of(service.detect_message(Parameters(MessageInput { message: oversized })));
    assert_eq!(error, "message is malformed, forbidden, or unsupported");
    assert!(!error.contains(CANARY));

    let profile = format!("cbpr-plus|{CANARY}");
    let error = error_of(service.validate_message(Parameters(ValidateInput {
        message: CANARY.to_owned(),
        layer: None,
        profile_release: Some(profile),
    })));
    assert_eq!(error, "profile release key is incomplete or invalid");
    assert!(!error.contains(CANARY));
}

#[test]
fn mcp_message_limit_is_inclusive_at_the_core_default() {
    let service = Iso20022Mcp::new();
    let maximum = rust_iso20022::ParseLimits::DEFAULT.max_input_bytes;
    for length in [maximum - 1, maximum] {
        let result = service.detect_message(Parameters(MessageInput {
            message: sized_message(length),
        }));
        assert!(result.is_ok(), "length {length}");
    }
    let error = error_of(service.detect_message(Parameters(MessageInput {
        message: sized_message(maximum + 1),
    })));
    assert_eq!(error, "message is malformed, forbidden, or unsupported");
}
