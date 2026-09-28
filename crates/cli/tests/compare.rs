use std::process::{Command, Output, Stdio};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_iso20022"))
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap()
}

#[test]
fn json_is_object_equal_to_core_version_diff() {
    let output = run(&["compare", "--json", "pacs.008.001.08", "pacs.008.001.10"]);
    assert_eq!(output.status.code(), Some(0));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let core =
        rust_iso20022::compare::compare_versions("pacs.008.001.08", "pacs.008.001.10").unwrap();
    assert_eq!(value["data"], serde_json::to_value(core).unwrap());
}

#[test]
fn human_output_is_deterministic_and_errors_are_typed_and_safe() {
    let human = run(&["compare", "pacs.008.001.08", "pacs.008.001.10"]);
    assert_eq!(human.status.code(), Some(0));
    let stdout = String::from_utf8(human.stdout).unwrap();
    assert!(stdout.starts_with("pacs.008.001.08 -> pacs.008.001.10\n"));

    let canary = "SECRET-ACCOUNT-VERSION";
    let unknown = run(&["compare", "--json", "pacs.008.001.08", canary]);
    assert_eq!(unknown.status.code(), Some(5));
    let value: serde_json::Value = serde_json::from_slice(&unknown.stdout).unwrap();
    assert_eq!(value["error"]["code"], "unknown_message");
    assert!(!value.to_string().contains(canary));
    assert!(!String::from_utf8(unknown.stderr).unwrap().contains(canary));

    let unrelated = run(&["compare", "--json", "pacs.008.001.08", "pacs.009.001.08"]);
    assert_eq!(unrelated.status.code(), Some(2));
    let value: serde_json::Value = serde_json::from_slice(&unrelated.stdout).unwrap();
    assert_eq!(value["error"]["code"], "unrelated_families");
}
