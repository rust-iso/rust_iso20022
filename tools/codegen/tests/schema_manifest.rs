mod common;

use std::collections::BTreeSet;
use std::fs;

use serde_json::Value;
use sha2::{Digest, Sha256};

#[test]
fn manifest_covers_every_schema_with_verified_raw_identity() {
    let root = common::workspace_root();
    let manifest_path = root.join("xsds/schema-manifest.json");
    let value: Value = serde_json::from_slice(
        &fs::read(&manifest_path).expect("xsds/schema-manifest.json must be tracked"),
    )
    .expect("schema manifest must be valid JSON");
    let records = value["schemas"]
        .as_array()
        .expect("schema manifest must contain a schemas array");

    let mut disk_paths: Vec<_> = fs::read_dir(root.join("xsds"))
        .expect("read xsds")
        .map(|entry| entry.expect("read xsd entry").path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "xsd"))
        .collect();
    disk_paths.sort();
    assert_eq!(
        disk_paths.len(),
        1_130,
        "the frozen baseline has 1,130 XSDs"
    );
    assert_eq!(
        records.len(),
        disk_paths.len(),
        "manifest/XSD coverage differs"
    );

    let mut paths = BTreeSet::new();
    let mut identities = BTreeSet::new();
    let mut generated_modules = BTreeSet::new();
    let mut roots = [0usize; 3];
    for record in records {
        let relative_path = record["path"].as_str().expect("schema path");
        let identity = record["identity"].as_str().expect("normalized identity");
        let namespace = record["namespace"].as_str().expect("target namespace");
        let root_element = record["root_element"].as_str().expect("root element");
        let expected_hash = record["sha256"].as_str().expect("raw SHA-256");
        let expected_len = record["byte_length"].as_u64().expect("byte length");
        let generated_module = record["generated_module"]
            .as_str()
            .expect("generated module path");

        assert!(
            paths.insert(relative_path.to_owned()),
            "duplicate path {relative_path}"
        );
        assert!(
            identities.insert(identity.to_owned()),
            "duplicate identity {identity}"
        );
        assert!(
            generated_modules.insert(generated_module.to_owned()),
            "duplicate generated module {generated_module}"
        );
        assert!(!namespace.is_empty(), "empty namespace for {relative_path}");
        assert!(!root_element.is_empty(), "empty root for {relative_path}");

        let bytes = fs::read(root_path(&root, relative_path)).expect("read manifested XSD");
        assert_eq!(
            bytes.len() as u64,
            expected_len,
            "length mismatch: {relative_path}"
        );
        assert_eq!(
            format!("{:x}", Sha256::digest(&bytes)),
            expected_hash,
            "hash mismatch: {relative_path}"
        );
        assert!(
            root.join(generated_module).is_file(),
            "missing generated module {generated_module}"
        );

        match root_element {
            "Document" => roots[0] += 1,
            "AppHdr" => roots[1] += 1,
            "Xchg" => roots[2] += 1,
            other => panic!("unexpected root {other} in {relative_path}"),
        }
    }

    assert_eq!(roots, [1_127, 2, 1]);
    assert_eq!(paths.len(), 1_130);
    let mut disk_modules = BTreeSet::new();
    for family in fs::read_dir(root.join("src/generated")).expect("read generated root") {
        let family = family.expect("read generated entry").path();
        if !family.is_dir() {
            continue;
        }
        for module in fs::read_dir(&family).expect("read generated family") {
            let module = module.expect("read generated module").path();
            if module
                .extension()
                .is_some_and(|extension| extension == "rs")
                && module.file_name().is_some_and(|name| name != "mod.rs")
            {
                disk_modules.insert(
                    module
                        .strip_prefix(&root)
                        .expect("workspace-relative generated module")
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }
    assert_eq!(
        generated_modules, disk_modules,
        "manifest/output coverage differs"
    );
    assert!(
        identities.contains("acmt.017.001.03"),
        "mirror suffix must normalize"
    );

    let semt = records
        .iter()
        .find(|record| record["identity"] == "semt.001.001.04")
        .expect("semt namespace exception");
    assert!(
        semt["namespace"]
            .as_str()
            .expect("semt namespace")
            .starts_with("urn:swift:xsd:"),
        "the real SWIFT namespace exception must not be normalized away"
    );

    fn root_path(root: &std::path::Path, relative: &str) -> std::path::PathBuf {
        root.join(relative)
    }
}

#[test]
fn schema_set_records_provenance_without_invention() {
    let root = common::workspace_root();
    let value: Value = serde_json::from_slice(
        &fs::read(root.join("xsds/schema-set.json")).expect("schema-set.json must be tracked"),
    )
    .expect("schema set must be valid JSON");
    assert_eq!(value["schema_count"], 1_130);
    assert!(matches!(
        value["provenance_status"].as_str(),
        Some("verified" | "partial" | "unverified")
    ));
    assert!(
        value.get("source").is_some(),
        "source status must be explicit"
    );
}
