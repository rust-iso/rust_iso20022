use std::io::Write;
use std::process::{Command, Stdio};

use serde_json::{Value, json};

const VALID: &str = include_str!("../../../fixtures/iso/valid/pacs.008.001.08-cross-field.xml");

#[test]
fn stdio_protocol_negotiates_lists_and_calls_tools_without_diagnostics() {
    let requests = [
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2026-07-28",
                "capabilities": {},
                "clientInfo": { "name": "contract-test", "version": "1.0" }
            }
        }),
        json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        }),
        json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list",
            "params": {}
        }),
        json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {
                "name": "detect_message",
                "arguments": { "message": VALID }
            }
        }),
    ];
    let mut child = Command::new(env!("CARGO_BIN_EXE_rust-iso20022-mcp"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    {
        let stdin = child.stdin.as_mut().unwrap();
        for request in requests {
            writeln!(stdin, "{}", serde_json::to_string(&request).unwrap()).unwrap();
        }
    }
    drop(child.stdin.take());
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let responses: Vec<Value> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let initialize = responses.iter().find(|value| value["id"] == 1).unwrap();
    assert_eq!(initialize["result"]["protocolVersion"], "2026-07-28");

    let listed = responses.iter().find(|value| value["id"] == 2).unwrap();
    assert_eq!(listed["result"]["tools"].as_array().unwrap().len(), 7);

    let called = responses.iter().find(|value| value["id"] == 3).unwrap();
    assert_eq!(
        called["result"]["structuredContent"]["message_id"],
        "pacs.008.001.08"
    );
    assert_eq!(called["result"]["isError"], false);
}
