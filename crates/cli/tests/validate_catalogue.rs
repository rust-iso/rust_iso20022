use std::io::Write;
use std::process::{Command, Output, Stdio};

const VALID: &str = include_str!("../../../fixtures/iso/valid/pacs.008.001.08-cross-field.xml");
const INVALID: &str = include_str!("../../../fixtures/iso/invalid/pacs.008.001.08-cross-field.xml");
const PROFILE_KEY: &str = concat!(
    "test-network|2025.1|ig-2025.1|2025-01-01|2025-12-31|",
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
);

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
fn validation_valid_invalid_and_json_exit_contract_match_core() {
    let valid = run(&["validate", "--json", "--layer", "l2", "-"], Some(VALID));
    assert_eq!(valid.status.code(), Some(0));
    let report: serde_json::Value = serde_json::from_slice(&valid.stdout).unwrap();
    assert_eq!(report["data"]["report"]["valid"], true);

    let invalid = run(&["validate", "--json", "--layer", "l2", "-"], Some(INVALID));
    assert_eq!(invalid.status.code(), Some(4));
    let report: serde_json::Value = serde_json::from_slice(&invalid.stdout).unwrap();
    assert_eq!(report["data"]["report"]["valid"], false);
    assert_eq!(
        report["data"]["report"]["errors"].as_array().unwrap().len(),
        6
    );
    assert!(
        report["data"]["report"]["errors"]
            .as_array()
            .unwrap()
            .iter()
            .any(|issue| issue["rule_id"] == "ISO20022-L2-PACS008-TX-COUNT")
    );
    assert!(
        !String::from_utf8(invalid.stderr)
            .unwrap()
            .contains("SECRET-OTHER-ACCOUNT")
    );
}

#[test]
fn unavailable_layers_and_profiles_use_exit_five() {
    let l1 = run(&["validate", "--json", "--layer", "l1", "-"], Some(VALID));
    assert_eq!(l1.status.code(), Some(5));
    let value: serde_json::Value = serde_json::from_slice(&l1.stdout).unwrap();
    assert_eq!(value["error"]["code"], "layer_unavailable");

    let profile = run(
        &["validate", "--json", "--profile", PROFILE_KEY, "-"],
        Some(VALID),
    );
    assert_eq!(profile.status.code(), Some(5));
    let value: serde_json::Value = serde_json::from_slice(&profile.stdout).unwrap();
    assert_eq!(value["error"]["code"], "profile_unavailable");

    let unresolved = run(
        &["validate", "--profile", "cbpr-plus:2026", "-"],
        Some(VALID),
    );
    assert_eq!(unresolved.status.code(), Some(2));
}

#[test]
fn catalogue_and_versions_are_exact_core_views() {
    let catalog = run(&["catalog", "--json", "pacs.008.001"], None);
    assert_eq!(catalog.status.code(), Some(0));
    let value: serde_json::Value = serde_json::from_slice(&catalog.stdout).unwrap();
    let rows = value["data"]["messages"].as_array().unwrap();
    assert_eq!(
        rows.len(),
        rust_iso20022::catalogue::list_family("pacs.008.001").len()
    );
    assert!(rows.iter().all(|row| {
        row["message_id"]
            .as_str()
            .unwrap()
            .starts_with("pacs.008.001.")
    }));

    let versions = run(&["versions", "--json", "pacs.008.001"], None);
    assert_eq!(versions.status.code(), Some(0));
    let value: serde_json::Value = serde_json::from_slice(&versions.stdout).unwrap();
    let expected: Vec<_> = rust_iso20022::catalogue::versions("pacs.008.001")
        .iter()
        .map(|descriptor| descriptor.identity)
        .collect();
    assert_eq!(
        value["data"]["versions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap())
            .collect::<Vec<_>>(),
        expected
    );
}
