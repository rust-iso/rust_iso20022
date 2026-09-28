#[path = "../src/descriptor.rs"]
#[allow(dead_code)]
mod descriptor;

use std::path::Path;

use descriptor::{describe_schema, FieldKind};

fn field<'a>(
    descriptor: &'a descriptor::SchemaDescriptor,
    path: &str,
) -> &'a descriptor::FieldDescriptor {
    descriptor
        .fields
        .iter()
        .find(|field| field.logical_path == path)
        .unwrap_or_else(|| panic!("missing field {path}"))
}

#[test]
fn pacs_008_descriptor_preserves_normalized_field_graph() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let descriptor =
        describe_schema(&root.join("xsds/pacs.008.001.08.xsd")).expect("describe pacs.008.001.08");

    assert_eq!(descriptor.identity, "pacs.008.001.08");
    assert_eq!(descriptor.business_area, "pacs");
    assert_eq!(
        descriptor.namespace,
        "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08"
    );
    assert_eq!(descriptor.root_element, "Document");
    assert_eq!(descriptor.root_type, "Document");
    assert_eq!(descriptor.required_feature, "model-pacs");
    assert_eq!(
        descriptor.generated_module,
        "src/generated/pacs/pacs_008_001_08.rs"
    );

    let root_field = field(&descriptor, "Document/FIToFICstmrCdtTrf");
    assert_eq!(root_field.wire_name, "FIToFICstmrCdtTrf");
    assert_eq!(root_field.rust_name, "fi_to_fi_cstmr_cdt_trf");
    assert_eq!(root_field.type_name, "FIToFICustomerCreditTransferV08");
    assert_eq!(root_field.namespace, descriptor.namespace);
    assert_eq!((root_field.min_occurs, root_field.max_occurs), (1, Some(1)));

    let optional_enum = field(&descriptor, "PaymentTypeInformation28/InstrPrty");
    assert_eq!(
        (optional_enum.min_occurs, optional_enum.max_occurs),
        (0, Some(1))
    );
    assert_eq!(optional_enum.enum_values, ["HIGH", "NORM"]);

    let repeated = field(&descriptor, "CreditTransferTransaction39/InstrForCdtrAgt");
    assert_eq!((repeated.min_occurs, repeated.max_occurs), (0, None));

    let choice = field(&descriptor, "AccountIdentification4Choice/IBAN");
    assert!(choice.choice_group.is_some());
    assert_eq!(choice.kind, FieldKind::Element);

    let value = field(&descriptor, "ActiveCurrencyAndAmount/$value");
    assert_eq!(value.kind, FieldKind::SimpleContent);
    let currency = field(&descriptor, "ActiveCurrencyAndAmount/@Ccy");
    assert_eq!(currency.kind, FieldKind::Attribute);
    assert_eq!(currency.rust_name, "ccy");
}

#[test]
fn header_and_namespace_exception_are_preserved_verbatim() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let head = describe_schema(&root.join("xsds/head.001.001.04.xsd")).expect("describe head");
    assert_eq!(head.root_element, "AppHdr");
    assert_eq!(head.root_type, "BusinessApplicationHeaderV04");
    assert_eq!(head.required_feature, "model-head");

    let semt = describe_schema(&root.join("xsds/semt.001.001.04.xsd")).expect("describe semt");
    assert_eq!(semt.namespace, "urn:swift:xsd:semt.001.001.04");
}

#[test]
fn schema_documentation_and_keyword_mapping_are_descriptive() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/golden.xsd");
    let descriptor = describe_schema(&fixture).expect("describe golden fixture");
    assert_eq!(
        descriptor.description.as_deref(),
        Some("Golden message documentation.")
    );
    let keyword = field(&descriptor, "Document/type");
    assert_eq!(keyword.wire_name, "type");
    assert_eq!(keyword.rust_name, "_type");
    assert_eq!(keyword.documentation, None);
}
