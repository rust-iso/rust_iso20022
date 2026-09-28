use std::collections::BTreeSet;
use std::path::Path;

use rust_iso20022::metadata;

#[test]
fn every_schema_and_generated_message_has_exactly_one_descriptor() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let descriptors = metadata::all();
    assert_eq!(descriptors.len(), 1_130);

    let mut identities = BTreeSet::new();
    let mut schema_paths = BTreeSet::new();
    let mut generated_modules = BTreeSet::new();
    for descriptor in descriptors {
        assert!(identities.insert(descriptor.identity));
        assert!(schema_paths.insert(descriptor.schema_path));
        assert!(generated_modules.insert(descriptor.generated_module));
        assert!(root.join(descriptor.schema_path).is_file());
        assert!(root.join(descriptor.generated_module).is_file());
        assert!(!descriptor.namespace.is_empty());
        assert!(!descriptor.root_element.is_empty());
        assert!(!descriptor.fields.is_empty());
    }

    let disk_schemas = std::fs::read_dir(root.join("xsds"))
        .expect("read schemas")
        .map(|entry| entry.expect("schema entry").path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "xsd"))
        .count();
    assert_eq!(schema_paths.len(), disk_schemas);

    let disk_modules = std::fs::read_dir(root.join("src/generated"))
        .expect("read generated root")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            path.is_dir().then_some(path)
        })
        .map(|family| {
            std::fs::read_dir(family)
                .expect("read family")
                .filter(|entry| {
                    let path = entry.as_ref().expect("module entry").path();
                    path.extension().is_some_and(|extension| extension == "rs")
                        && path.file_name().is_some_and(|name| name != "mod.rs")
                })
                .count()
        })
        .sum::<usize>();
    assert_eq!(generated_modules.len(), disk_modules);
    assert_eq!(
        descriptors
            .iter()
            .map(|descriptor| descriptor.fields.len())
            .sum::<usize>(),
        258_005,
        "normalized field-declaration coverage changed"
    );

    let head = descriptors
        .iter()
        .find(|descriptor| descriptor.identity == "head.001.001.04")
        .expect("head descriptor");
    assert_eq!(head.root_element, "AppHdr");
    let semt = descriptors
        .iter()
        .find(|descriptor| descriptor.identity == "semt.001.001.04")
        .expect("semt descriptor");
    assert_eq!(semt.namespace, "urn:swift:xsd:semt.001.001.04");
}

#[test]
fn descriptors_are_immutable_metadata_not_message_storage() {
    fn assert_static(_: &'static [metadata::MessageDescriptor]) {}
    assert_static(metadata::all());
    assert!(std::mem::size_of::<metadata::MessageDescriptor>() < 256);
}
