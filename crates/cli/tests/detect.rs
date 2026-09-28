use std::io::Write;
use std::process::{Command, Output, Stdio};

const KNOWN: &str =
    r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08"></Document>"#;

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

fn run(args: &[&str], stdin: Option<&str>) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_iso20022"))
        .args(args)
        .stdin(if stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(input) = stdin {
        child
            .stdin
            .take()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
    }
    child.wait_with_output().unwrap()
}

#[test]
fn detects_stdin_in_human_and_versioned_json_forms() {
    let human = run(&["detect", "-"], Some(KNOWN));
    assert_eq!(human.status.code(), Some(0));
    assert_eq!(
        String::from_utf8(human.stdout).unwrap().trim(),
        "pacs.008.001.08"
    );
    assert!(human.stderr.is_empty());

    let json = run(&["detect", "--json", "-"], Some(KNOWN));
    assert_eq!(json.status.code(), Some(0));
    let value: serde_json::Value = serde_json::from_slice(&json.stdout).unwrap();
    assert_eq!(value["schema_version"], 1);
    assert_eq!(value["command"], "detect");
    assert_eq!(value["status"], "success");
    assert_eq!(value["data"]["message_id"], "pacs.008.001.08");
    assert_eq!(value["data"]["business_area"], "pacs");
    assert_eq!(value["data"]["version"], "08");
}

#[test]
fn detects_file_input_with_the_same_core_result() {
    let path = std::env::temp_dir().join(format!(
        "rust-iso20022-cli-detect-{}-{}.xml",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ));
    std::fs::write(&path, KNOWN).unwrap();
    let output = run(&["detect", "--json", path.to_str().unwrap()], None);
    std::fs::remove_file(path).unwrap();
    assert_eq!(output.status.code(), Some(0));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let core = rust_iso20022::detect_message(KNOWN, rust_iso20022::ParseLimits::DEFAULT).unwrap();
    assert_eq!(value["data"]["message_id"], core.message_id().as_str());
    assert_eq!(value["data"]["namespace"], core.namespace());
    assert_eq!(value["data"]["root_element"], core.root_element());
}

#[test]
fn unknown_mismatch_and_limit_fail_with_parse_exit_and_safe_diagnostics() {
    let canary = "DE89370400440532013000";
    let unknown_xml = format!(r#"<Private><Account>{canary}</Account></Private>"#);
    let unknown = run(&["detect", "-"], Some(&unknown_xml));
    assert_eq!(unknown.status.code(), Some(3));
    let unknown_stderr = String::from_utf8(unknown.stderr).unwrap();
    assert!(!unknown_stderr.contains(canary));
    assert!(unknown_stderr.contains("unknown ISO 20022 message"));

    let mismatch = run(
        &["detect", "--json", "-"],
        Some(r#"<Wrong xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08"/>"#),
    );
    assert_eq!(mismatch.status.code(), Some(3));
    let value: serde_json::Value = serde_json::from_slice(&mismatch.stdout).unwrap();
    assert_eq!(value["error"]["code"], "root_mismatch");
    assert!(
        !String::from_utf8(mismatch.stderr)
            .unwrap()
            .contains("<Wrong")
    );

    let oversized = format!("<Document>{}</Document>", "X".repeat(8 * 1024 * 1024));
    let limited = run(&["detect", "--json", "-"], Some(&oversized));
    assert_eq!(limited.status.code(), Some(3));
    let value: serde_json::Value = serde_json::from_slice(&limited.stdout).unwrap();
    assert_eq!(value["error"]["code"], "input_limit_exceeded");
}

#[test]
fn cli_input_limit_is_inclusive_at_the_core_default() {
    let maximum = rust_iso20022::ParseLimits::DEFAULT.max_input_bytes;
    for length in [maximum - 1, maximum] {
        let output = run(&["detect", "--json", "-"], Some(&sized_message(length)));
        assert_eq!(output.status.code(), Some(0), "length {length}");
    }
    let output = run(
        &["detect", "--json", "-"],
        Some(&sized_message(maximum + 1)),
    );
    assert_eq!(output.status.code(), Some(3));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["error"]["code"], "input_limit_exceeded");
}

#[test]
fn usage_is_exit_two_and_all_reserved_exit_codes_are_stable() {
    let usage = run(&[], None);
    assert_eq!(usage.status.code(), Some(2));
    assert!(usage.stdout.is_empty());

    assert_eq!(rust_iso20022_cli::ExitCode::Success as i32, 0);
    assert_eq!(rust_iso20022_cli::ExitCode::Usage as i32, 2);
    assert_eq!(rust_iso20022_cli::ExitCode::ParseFailure as i32, 3);
    assert_eq!(rust_iso20022_cli::ExitCode::ValidationFailure as i32, 4);
    assert_eq!(rust_iso20022_cli::ExitCode::Unavailable as i32, 5);
    assert_eq!(rust_iso20022_cli::ExitCode::InternalFailure as i32, 10);
}

#[test]
fn serialization_is_explicitly_unavailable_without_model_and_json_features() {
    let unavailable = run(&["to-json", "--json", "-"], Some(KNOWN));
    assert_eq!(unavailable.status.code(), Some(5));
    let value: serde_json::Value = serde_json::from_slice(&unavailable.stdout).unwrap();
    assert_eq!(value["error"]["code"], "feature_unavailable");
    assert!(
        !String::from_utf8(unavailable.stderr)
            .unwrap()
            .contains(KNOWN)
    );
}
