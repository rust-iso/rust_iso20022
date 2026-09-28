mod common;

use std::fs;

fn fixture_input(root: &std::path::Path) -> std::path::PathBuf {
    let input = root.join("input");
    fs::create_dir_all(&input).expect("create fixture input");
    fs::copy(
        common::workspace_root().join("tools/codegen/tests/fixtures/golden.xsd"),
        input.join("gold.001.001.01.xsd"),
    )
    .expect("copy fixture schema");
    input
}

#[test]
fn three_clean_generations_are_byte_identical() {
    let temp = tempfile::tempdir().expect("temporary directory");
    let input = fixture_input(temp.path());
    let mut trees = Vec::new();
    for index in 1..=3 {
        let cwd = temp.path().join(format!("run-{index}"));
        let output = cwd.join("generated");
        let result = common::run_codegen(&cwd, &input, &output, &[]);
        assert!(result.status.success(), "{}", common::output_text(&result));
        trees.push(common::tree_bytes(&output));
    }
    assert_eq!(trees[0], trees[1]);
    assert_eq!(trees[1], trees[2]);
}

#[test]
fn unknown_arguments_and_partial_generation_fail_nonzero() {
    let temp = tempfile::tempdir().expect("temporary directory");
    let input = fixture_input(temp.path());
    let unknown = common::run_codegen(
        &temp.path().join("unknown"),
        &input,
        &temp.path().join("unknown/generated"),
        &["--definitely-unknown"],
    );
    assert!(
        !unknown.status.success(),
        "unknown option was silently ignored"
    );

    fs::write(input.join("bad.001.001.01.xsd"), b"<not-a-schema>").expect("write malformed XSD");
    let partial = common::run_codegen(
        &temp.path().join("partial"),
        &input,
        &temp.path().join("partial/generated"),
        &[],
    );
    assert!(
        !partial.status.success(),
        "partial generation returned success"
    );
    assert!(
        !temp.path().join("partial/generated").exists(),
        "failed generation installed a partial output tree"
    );
}

#[test]
fn check_detects_tampering_missing_and_stale_outputs() {
    let temp = tempfile::tempdir().expect("temporary directory");
    let input = fixture_input(temp.path());
    let cwd = temp.path().join("check");
    let output = cwd.join("generated");
    let initial = common::run_codegen(&cwd, &input, &output, &[]);
    assert!(
        initial.status.success(),
        "{}",
        common::output_text(&initial)
    );

    let clean = common::run_codegen(&cwd, &input, &output, &["--check"]);
    assert!(
        clean.status.success(),
        "clean tree rejected: {}",
        common::output_text(&clean)
    );

    let generated = output.join("gold/gold_001_001_01.rs");
    fs::write(&generated, b"tampered").expect("tamper generated output");
    assert!(!common::run_codegen(&cwd, &input, &output, &["--check"])
        .status
        .success());

    fs::remove_file(&generated).expect("remove generated output");
    assert!(!common::run_codegen(&cwd, &input, &output, &["--check"])
        .status
        .success());

    fs::write(output.join("stale.rs"), b"stale").expect("write stale output");
    assert!(!common::run_codegen(&cwd, &input, &output, &["--check"])
        .status
        .success());
}

#[test]
fn only_mode_is_isolated_from_global_outputs() {
    let temp = tempfile::tempdir().expect("temporary directory");
    let input = fixture_input(temp.path());
    let cwd = temp.path().join("only");
    let output = cwd.join("generated");
    fs::create_dir_all(&output).expect("create sentinel output");
    fs::write(output.join("sentinel"), b"unchanged").expect("write sentinel");

    let result = common::run_codegen(&cwd, &input, &output, &["--only", "gold.001.001.01.xsd"]);
    assert!(result.status.success(), "{}", common::output_text(&result));
    assert_eq!(
        fs::read(output.join("sentinel")).expect("read sentinel"),
        b"unchanged"
    );
    assert!(
        !cwd.join("src/catalogue/data.rs").exists(),
        "diagnostic --only rewrote the global catalogue"
    );
}
