use rust_iso20022::catalogue::{self, CoverageState};

#[test]
fn all_schema_derived_lookup_modes_are_complete() {
    let descriptor = catalogue::lookup_descriptor("pacs.008.001.10").expect("lookup by id");
    assert_eq!(
        descriptor.namespace,
        "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.10"
    );
    assert_eq!(descriptor.root_element, "Document");

    let family = catalogue::list_family("pacs.008.001");
    let identities: Vec<_> = family
        .iter()
        .map(|descriptor| descriptor.identity)
        .collect();
    assert_eq!(
        identities,
        ["pacs.008.001.08", "pacs.008.001.10", "pacs.008.001.14"]
    );
    assert_eq!(catalogue::versions("pacs.008.001"), family);
    assert_eq!(
        catalogue::latest("pacs.008.001")
            .expect("latest version")
            .identity,
        "pacs.008.001.14"
    );
    assert_eq!(
        catalogue::by_namespace(descriptor.namespace)
            .expect("namespace lookup")
            .identity,
        descriptor.identity
    );
    assert_eq!(
        catalogue::by_root(descriptor.namespace, "Document")
            .expect("namespace/root lookup")
            .identity,
        descriptor.identity
    );
}

#[test]
fn unknowns_and_coverage_state_are_explicit() {
    assert!(catalogue::lookup_descriptor("pacs.999.001.01").is_none());
    assert!(catalogue::list_family("pacs.999.001").is_empty());
    assert!(catalogue::latest("pacs.999.001").is_none());
    assert!(catalogue::by_namespace("urn:example:unknown").is_none());
    assert!(catalogue::by_root("urn:example:unknown", "Document").is_none());
    assert_eq!(
        catalogue::coverage("pacs.008.001.10"),
        CoverageState::Available {
            required_feature: "model-pacs"
        }
    );
    assert_eq!(
        catalogue::coverage("pacs.999.001.01"),
        CoverageState::Unknown
    );
    let known_without_model = CoverageState::KnownUnavailable;
    assert_ne!(known_without_model, CoverageState::Unknown);
}

#[test]
fn every_index_points_to_the_same_static_descriptor() {
    let by_id = catalogue::lookup_descriptor("semt.001.001.04").expect("id");
    let by_namespace = catalogue::by_namespace("urn:swift:xsd:semt.001.001.04").expect("namespace");
    let by_root = catalogue::by_root("urn:swift:xsd:semt.001.001.04", "Document").expect("root");
    assert!(std::ptr::eq(by_id, by_namespace));
    assert!(std::ptr::eq(by_id, by_root));
}

#[test]
fn legacy_catalogue_remains_consistent_with_descriptors() {
    assert_eq!(catalogue::all().len(), rust_iso20022::metadata::all().len());
    for legacy in catalogue::all() {
        let descriptor = catalogue::lookup_descriptor(legacy.message_name)
            .expect("legacy entry must resolve to generated descriptor");
        assert_eq!(legacy.namespace, descriptor.namespace);
        assert_eq!(legacy.business_area, descriptor.business_area);
        assert!(legacy.has_model);
    }
}
