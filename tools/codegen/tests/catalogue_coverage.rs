#[path = "../src/catalogue.rs"]
#[allow(dead_code)]
mod catalogue;
#[path = "../src/descriptor.rs"]
#[allow(dead_code)]
mod descriptor;

use std::path::Path;

use catalogue::{verify_release_coverage, CoverageError};
use descriptor::describe_schema;

fn fixtures() -> (Vec<descriptor::SchemaDescriptor>, Vec<(String, String)>) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let descriptors = vec![
        describe_schema(&root.join("xsds/pacs.008.001.08.xsd")).expect("pacs descriptor"),
        describe_schema(&root.join("xsds/head.001.001.04.xsd")).expect("head descriptor"),
    ];
    let expected = descriptors
        .iter()
        .map(|descriptor| {
            (
                descriptor.identity.clone(),
                descriptor.generated_module.clone(),
            )
        })
        .collect();
    (descriptors, expected)
}

#[test]
fn exact_bidirectional_release_coverage_passes() {
    let (descriptors, expected) = fixtures();
    verify_release_coverage(&descriptors, &expected).expect("exact coverage");
}

#[test]
fn deleted_added_duplicate_and_misdirected_rows_fail() {
    let (descriptors, expected) = fixtures();

    assert!(matches!(
        verify_release_coverage(&descriptors[..1], &expected),
        Err(CoverageError::MissingDescriptor { .. })
    ));

    let mut added = descriptors.clone();
    let mut rogue = added[0].clone();
    rogue.identity = "pacs.999.001.01".to_owned();
    rogue.generated_module = "src/generated/pacs/pacs_999_001_01.rs".to_owned();
    added.push(rogue);
    assert!(matches!(
        verify_release_coverage(&added, &expected),
        Err(CoverageError::UnexpectedDescriptor { .. })
    ));

    let mut duplicate = descriptors.clone();
    duplicate.push(descriptors[0].clone());
    assert!(matches!(
        verify_release_coverage(&duplicate, &expected),
        Err(CoverageError::DuplicateDescriptor { .. })
    ));

    let mut misdirected = descriptors;
    misdirected[0].generated_module = "src/generated/pacs/wrong.rs".to_owned();
    assert!(matches!(
        verify_release_coverage(&misdirected, &expected),
        Err(CoverageError::GeneratedModuleMismatch { .. })
    ));
}
