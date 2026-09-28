mod common;

use std::fs;

#[test]
fn fixture_preserves_schema_shapes_and_metadata() {
    let temp = tempfile::tempdir().expect("temporary directory");
    let input = temp.path().join("input");
    let output = temp.path().join("generated");
    fs::create_dir_all(&input).expect("create input directory");
    fs::copy(
        common::workspace_root().join("tools/codegen/tests/fixtures/golden.xsd"),
        input.join("gold.001.001.01.xsd"),
    )
    .expect("copy golden XSD");

    let result = common::run_codegen(temp.path(), &input, &output, &[]);
    assert!(result.status.success(), "{}", common::output_text(&result));
    let generated =
        fs::read_to_string(output.join("gold/gold_001_001_01.rs")).expect("golden Rust output");

    for expected in [
        "urn:example:golden.001.001.01",
        "schema manifest: xsds/schema-manifest.json",
        "source: xsds/gold.001.001.01.xsd; sha256:",
        "pub struct Document",
        "Golden message documentation.",
        "pub enum StatusCode",
        "pub optional: Option<String>",
        "pub repeated: Vec<String>",
        "pub struct Choice",
        "pub value: String",
        "pub ccy: String",
        "pub _type: Option<String>",
        "impl crate::MxMessage for Document",
    ] {
        assert!(
            generated.contains(expected),
            "missing {expected:?}\n{generated}"
        );
    }
}
