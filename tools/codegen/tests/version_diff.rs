#[path = "../src/descriptor.rs"]
#[allow(dead_code)]
mod descriptor;

use std::fs;

use rust_iso20022::compare::{compare_descriptors, CompareError};
use rust_iso20022::metadata::{FieldDescriptor, FieldKind, MessageDescriptor};

const COMPACT: &str = r#"<?xml version="1.0"?>
<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema" targetNamespace="urn:iso:std:iso:20022:tech:xsd:mini.001.001.01">
  <xsd:element name="Document" type="Document"/>
  <xsd:complexType name="Document"><xsd:sequence><xsd:element name="Value" type="Text" minOccurs="0" maxOccurs="1"/></xsd:sequence></xsd:complexType>
  <xsd:simpleType name="Text"><xsd:restriction base="xsd:string"/></xsd:simpleType>
</xsd:schema>"#;

const FORMATTED: &str = r#"<?xml version="1.0"?>
<xsd:schema
    targetNamespace="urn:iso:std:iso:20022:tech:xsd:mini.001.001.02"
    xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <xsd:element type="Document" name="Document" />
  <xsd:complexType name="Document">
    <xsd:sequence>
      <xsd:element maxOccurs="1" minOccurs="0" type="Text" name="Value" />
    </xsd:sequence>
  </xsd:complexType>
  <xsd:simpleType name="Text">
    <xsd:restriction base="xsd:string" />
  </xsd:simpleType>
</xsd:schema>"#;

fn leak(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

fn runtime(value: descriptor::SchemaDescriptor) -> MessageDescriptor {
    let fields = value
        .fields
        .into_iter()
        .map(|field| FieldDescriptor {
            logical_path: leak(field.logical_path),
            namespace: leak(field.namespace),
            wire_name: leak(field.wire_name),
            rust_name: leak(field.rust_name),
            type_name: leak(field.type_name),
            min_occurs: field.min_occurs,
            max_occurs: field.max_occurs,
            enum_values: Box::leak(
                field
                    .enum_values
                    .into_iter()
                    .map(leak)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            documentation: field.documentation.map(leak),
            choice_group: field.choice_group,
            kind: match field.kind {
                descriptor::FieldKind::Element => FieldKind::Element,
                descriptor::FieldKind::Attribute => FieldKind::Attribute,
                descriptor::FieldKind::SimpleContent => FieldKind::SimpleContent,
            },
        })
        .collect::<Vec<_>>();
    MessageDescriptor {
        identity: leak(value.identity),
        business_area: leak(value.business_area),
        namespace: leak(value.namespace),
        root_element: leak(value.root_element),
        root_type: leak(value.root_type),
        description: value.description.map(leak),
        schema_path: leak(value.schema_path),
        schema_sha256: leak(value.schema_sha256),
        generated_module: leak(value.generated_module),
        required_feature: leak(value.required_feature),
        fields: Box::leak(fields.into_boxed_slice()),
    }
}

#[test]
fn formatting_only_schema_changes_produce_no_semantic_changes() {
    let directory = tempfile::tempdir().unwrap();
    let compact = directory.path().join("mini.001.001.01.xsd");
    let formatted = directory.path().join("mini.001.001.02.xsd");
    fs::write(&compact, COMPACT).unwrap();
    fs::write(&formatted, FORMATTED).unwrap();

    let from = runtime(descriptor::describe_schema(&compact).unwrap());
    let to = runtime(descriptor::describe_schema(&formatted).unwrap());
    assert_ne!(from.schema_sha256, to.schema_sha256);
    assert!(compare_descriptors(&from, &to).unwrap().changes.is_empty());

    let mut unrelated = to.clone();
    unrelated.identity = "other.001.001.02";
    assert!(matches!(
        compare_descriptors(&from, &unrelated),
        Err(CompareError::UnrelatedFamilies { .. })
    ));
}
