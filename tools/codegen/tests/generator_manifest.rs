mod common;

use std::fs;

use serde_json::Value;
use sha2::{Digest, Sha256};

#[test]
fn generator_manifest_is_path_stable_and_matches_every_output() {
    let root = common::workspace_root();
    let path = root.join("tools/codegen/generator-manifest.json");
    let bytes = fs::read(&path).expect("generator manifest must be tracked");
    let text = std::str::from_utf8(&bytes).expect("generator manifest UTF-8");
    assert!(!text.contains(&root.to_string_lossy().to_string()));
    for forbidden in ["timestamp", "hostname", "generated_at"] {
        assert!(
            !text.contains(forbidden),
            "nondeterministic field {forbidden}"
        );
    }

    let manifest: Value = serde_json::from_slice(&bytes).expect("generator manifest JSON");
    assert_eq!(manifest["format_version"], 1);
    assert_eq!(
        manifest["generator"]["xsd_parser_revision"],
        "d476e854b28b197442096c268d79263c50300c9e"
    );
    let outputs = manifest["outputs"].as_array().expect("outputs array");
    assert_eq!(
        outputs.len(),
        1_168,
        "1,164 generated Rust files, two catalogue artifacts, and two metadata artifacts"
    );

    for output in outputs {
        let relative = output["path"].as_str().expect("logical output path");
        assert!(
            !relative.starts_with('/'),
            "absolute output path {relative}"
        );
        let raw = fs::read(root.join(relative)).expect("read manifested output");
        assert_eq!(
            output["byte_length"],
            raw.len() as u64,
            "length: {relative}"
        );
        assert_eq!(
            output["sha256"],
            format!("{:x}", Sha256::digest(&raw)),
            "digest: {relative}"
        );
    }
}
