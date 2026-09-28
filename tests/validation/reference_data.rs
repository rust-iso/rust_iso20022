use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

#[test]
fn manifest_pins_reviewed_sources_licenses_releases_and_hashes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = root.join("reference-data/manifest.json");
    let bytes = fs::read(&manifest_path).expect("reference-data manifest");
    let manifest: serde_json::Value = serde_json::from_slice(&bytes).expect("valid manifest JSON");
    let datasets = manifest["datasets"].as_array().expect("datasets array");
    assert_eq!(datasets.len(), 3);

    for expected in ["iban-lengths", "iso-3166-alpha2", "iso-4217-active"] {
        let dataset = datasets
            .iter()
            .find(|dataset| dataset["id"] == expected)
            .unwrap_or_else(|| panic!("missing {expected}"));
        assert_eq!(dataset["reviewed"], true, "{expected}");
        for field in [
            "release",
            "source",
            "retrieved_at",
            "license_decision",
            "path",
            "sha256",
        ] {
            assert!(
                dataset[field]
                    .as_str()
                    .is_some_and(|value| !value.is_empty()),
                "{expected}.{field}"
            );
        }
        let relative = dataset["path"].as_str().unwrap();
        let snapshot = fs::read(root.join(relative)).expect("snapshot exists");
        assert_eq!(
            hex(&Sha256::digest(snapshot)),
            dataset["sha256"],
            "{expected}"
        );
    }

    assert_eq!(data_rows(root.join("reference-data/iban-lengths.csv")), 89);
    assert_eq!(
        data_rows(root.join("reference-data/iso3166-alpha2.csv")),
        249
    );
    assert_eq!(
        data_rows(root.join("reference-data/iso4217-active.csv")),
        178
    );

    let manifest_digest = hex(&Sha256::digest(&bytes));
    assert_eq!(
        rust_iso20022::helpers::REFERENCE_DATA_MANIFEST_SHA256,
        manifest_digest
    );
}

fn data_rows(path: impl AsRef<Path>) -> usize {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .skip(1)
        .filter(|line| !line.is_empty())
        .count()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut output, byte| {
        write!(output, "{byte:02x}").expect("writing to String cannot fail");
        output
    })
}
