use std::io::Write;
use std::process::{Command, Output, Stdio};

use rust_iso20022::builders::Pacs008Builder;
use rust_iso20022::helpers::Money;

fn run(args: &[&str], stdin: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_iso20022"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

fn xml() -> String {
    let document = Pacs008Builder::new()
        .message_id("CLI-INSPECT")
        .transaction_id("CLI-TX")
        .settlement(Money::parse("EUR", "10.25").unwrap())
        .build()
        .unwrap();
    rust_iso20022::to_xml(&document).unwrap()
}

#[test]
fn inspect_human_and_json_match_core_descriptor() {
    let xml = xml();
    let core = rust_iso20022::detect_message(&xml, rust_iso20022::ParseLimits::DEFAULT).unwrap();

    let human = run(&["inspect", "-"], &xml);
    assert_eq!(human.status.code(), Some(0));
    let human = String::from_utf8(human.stdout).unwrap();
    assert!(human.contains("message_id: pacs.008.001.08"));
    assert!(human.contains("root_element: Document"));

    let output = run(&["inspect", "--json", "-"], &xml);
    assert_eq!(output.status.code(), Some(0));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["data"]["message_id"], core.message_id().as_str());
    assert_eq!(value["data"]["namespace"], core.namespace());
    assert_eq!(
        value["data"]["field_count"].as_u64().unwrap() as usize,
        core.descriptor().fields.len()
    );
}

#[test]
fn xml_to_json_matches_core_and_round_trips_back_to_xml() {
    let xml = xml();
    let core = rust_iso20022::parse(&xml).unwrap();
    let expected: serde_json::Value = serde_json::from_str(&core.to_json().unwrap()).unwrap();

    let converted = run(&["to-json", "-"], &xml);
    assert_eq!(converted.status.code(), Some(0));
    let actual: serde_json::Value = serde_json::from_slice(&converted.stdout).unwrap();
    assert_eq!(actual, expected);

    let converted = run(&["to-json", "--json", "-"], &xml);
    assert_eq!(converted.status.code(), Some(0));
    let envelope: serde_json::Value = serde_json::from_slice(&converted.stdout).unwrap();
    assert_eq!(envelope["data"]["message_id"], "pacs.008.001.08");
    assert_eq!(envelope["data"]["document"], expected);

    let json = serde_json::to_string(&expected).unwrap();
    let converted = run(&["to-xml", "pacs.008.001.08", "-"], &json);
    assert_eq!(converted.status.code(), Some(0));
    let roundtrip = String::from_utf8(converted.stdout).unwrap();
    let reparsed = rust_iso20022::parse(&roundtrip).unwrap();
    assert_eq!(reparsed.message_id().as_str(), "pacs.008.001.08");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&reparsed.to_json().unwrap()).unwrap(),
        expected
    );
}

#[test]
fn serialization_errors_and_diagnostics_do_not_echo_input() {
    let canary = "DE89370400440532013000";
    let invalid = format!(r#"{{"account":"{canary}"}}"#);
    let output = run(&["to-xml", "pacs.008.001.08", "--json", "-"], &invalid);
    assert_eq!(output.status.code(), Some(3));
    assert!(!String::from_utf8(output.stderr).unwrap().contains(canary));
    let envelope: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(envelope["error"]["code"], "deserialize_failed");
}
