use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn sha256(path: &Path) -> String {
    let bytes = fs::read(path).unwrap();
    format!("{:x}", Sha256::digest(bytes))
}

#[test]
fn source_manifest_schema_requires_traceability_and_rights_fields() {
    let schema: serde_json::Value =
        serde_json::from_slice(&fs::read(root().join("profiles/manifest.schema.json")).unwrap())
            .unwrap();
    let required = schema["$defs"]["source_document"]["required"]
        .as_array()
        .unwrap();
    for field in [
        "document_id",
        "publisher",
        "source_url",
        "sha256",
        "access_classification",
        "implementation_decision",
        "distribution_decision",
    ] {
        assert!(
            required.iter().any(|candidate| candidate == field),
            "{field}"
        );
    }
}

#[test]
fn synthetic_source_and_historical_fixtures_are_digest_pinned() {
    let manifest: serde_json::Value = serde_json::from_slice(
        &fs::read(root().join("tests/profiles/fixtures/source-manifest.json")).unwrap(),
    )
    .unwrap();
    let source = &manifest["sources"][0];
    let source_path = root().join(source["relative_path"].as_str().unwrap());
    assert_eq!(sha256(&source_path), source["sha256"].as_str().unwrap());
    assert_eq!(manifest["test_only"], true);

    let index: serde_json::Value = serde_json::from_slice(
        &fs::read(root().join("tests/profiles/fixtures/fixture-index.json")).unwrap(),
    )
    .unwrap();
    let fixtures = index["fixtures"].as_array().unwrap();
    assert_eq!(fixtures.len(), 2);
    for fixture in fixtures {
        let path = root().join(fixture["relative_path"].as_str().unwrap());
        assert_eq!(sha256(&path), fixture["sha256"].as_str().unwrap());
    }
    assert_ne!(fixtures[0]["sha256"], fixtures[1]["sha256"]);
}
