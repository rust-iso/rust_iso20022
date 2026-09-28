//! Read-only normalized descriptions of XSD declarations.
//!
//! These records contain schema provenance and field structure only. They never
//! contain message-instance values and are not an alternative canonical model;
//! generated structs remain the sole ISO 20022 data model.

use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::Path;

use inflector::Inflector;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldKind {
    Element,
    Attribute,
    SimpleContent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldDescriptor {
    pub logical_path: String,
    pub namespace: String,
    pub wire_name: String,
    pub rust_name: String,
    pub type_name: String,
    pub min_occurs: u32,
    /// `None` represents `maxOccurs="unbounded"`.
    pub max_occurs: Option<u32>,
    pub enum_values: Vec<String>,
    pub documentation: Option<String>,
    pub choice_group: Option<u32>,
    pub kind: FieldKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaDescriptor {
    pub identity: String,
    pub business_area: String,
    pub namespace: String,
    pub root_element: String,
    pub root_type: String,
    pub description: Option<String>,
    pub schema_path: String,
    pub schema_sha256: String,
    pub generated_module: String,
    pub required_feature: String,
    pub fields: Vec<FieldDescriptor>,
}

pub fn describe_schema(path: &Path) -> Result<SchemaDescriptor, String> {
    let bytes = fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    let text = std::str::from_utf8(&bytes)
        .map_err(|error| format!("schema {} is not UTF-8: {error}", path.display()))?;
    let document = roxmltree::Document::parse(text)
        .map_err(|error| format!("parse schema {}: {error}", path.display()))?;
    let schema = document.root_element();
    let namespace = schema
        .attribute("targetNamespace")
        .ok_or_else(|| format!("schema {} has no targetNamespace", path.display()))?
        .to_owned();
    let root = schema
        .children()
        .find(|node| node.is_element() && node.tag_name().name() == "element")
        .ok_or_else(|| format!("schema {} has no root element", path.display()))?;
    let root_element = required_attribute(root, "name", path)?;
    let root_type = required_attribute(root, "type", path)?;

    let basename = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("non-UTF-8 schema basename: {}", path.display()))?;
    let identity = normalized_identity(basename)
        .or_else(|| namespace.rsplit(':').next().map(str::to_owned))
        .ok_or_else(|| format!("cannot derive identity for {}", path.display()))?;
    let business_area = identity
        .split('.')
        .next()
        .ok_or_else(|| format!("invalid identity {identity}"))?
        .to_owned();

    let enum_types = enum_types(schema);
    let mut fields = Vec::new();
    for complex_type in schema
        .children()
        .filter(|node| node.is_element() && node.tag_name().name() == "complexType")
    {
        let owner = required_attribute(complex_type, "name", path)?;
        let mut next_choice_group = 0u32;
        collect_complex_fields(
            complex_type,
            &owner,
            &namespace,
            &enum_types,
            &mut next_choice_group,
            &mut fields,
        )?;
    }

    Ok(SchemaDescriptor {
        identity: identity.clone(),
        business_area: business_area.clone(),
        namespace,
        root_element,
        root_type,
        description: direct_documentation(schema),
        schema_path: format!("xsds/{basename}"),
        schema_sha256: format!("{:x}", Sha256::digest(&bytes)),
        generated_module: format!(
            "src/generated/{}/{}.rs",
            business_area,
            identity.replace('.', "_")
        ),
        required_feature: format!("model-{business_area}"),
        fields,
    })
}

pub fn write_runtime_metadata(
    descriptors: &[SchemaDescriptor],
    rust_path: &Path,
    data_path: &Path,
) -> Result<(), String> {
    let mut sorted = descriptors.to_vec();
    sorted.sort_by(|left, right| left.identity.cmp(&right.identity));
    let mut data = String::new();
    for descriptor in &sorted {
        let message_values = [
            descriptor.identity.as_str(),
            descriptor.business_area.as_str(),
            descriptor.namespace.as_str(),
            descriptor.root_element.as_str(),
            descriptor.root_type.as_str(),
            descriptor.description.as_deref().unwrap_or(""),
            descriptor.schema_path.as_str(),
            descriptor.schema_sha256.as_str(),
            descriptor.generated_module.as_str(),
            descriptor.required_feature.as_str(),
        ];
        validate_tsv_values(&message_values)?;
        data.push_str("M\t");
        data.push_str(&message_values.join("\t"));
        data.push('\n');
        for field in &descriptor.fields {
            let max_occurs = field
                .max_occurs
                .map(|value| value.to_string())
                .unwrap_or_default();
            let choice_group = field
                .choice_group
                .map(|value| value.to_string())
                .unwrap_or_default();
            let enum_values = field.enum_values.join("\u{1f}");
            let min_occurs = field.min_occurs.to_string();
            let kind = match field.kind {
                FieldKind::Element => "element",
                FieldKind::Attribute => "attribute",
                FieldKind::SimpleContent => "simple_content",
            };
            let field_values = [
                field.logical_path.as_str(),
                field.namespace.as_str(),
                field.wire_name.as_str(),
                field.rust_name.as_str(),
                field.type_name.as_str(),
                min_occurs.as_str(),
                max_occurs.as_str(),
                enum_values.as_str(),
                field.documentation.as_deref().unwrap_or(""),
                choice_group.as_str(),
                kind,
            ];
            validate_tsv_values(&field_values)?;
            data.push_str("F\t");
            data.push_str(&field_values.join("\t"));
            data.push('\n');
        }
    }
    let mut compressed = Vec::new();
    {
        let mut compressor = brotli::CompressorWriter::new(&mut compressed, 4096, 11, 22);
        compressor
            .write_all(data.as_bytes())
            .map_err(|error| format!("compress runtime metadata: {error}"))?;
    }
    fs::write(data_path, compressed)
        .map_err(|error| format!("write {}: {error}", data_path.display()))?;
    fs::write(rust_path, RUNTIME_METADATA_SOURCE)
        .map_err(|error| format!("write {}: {error}", rust_path.display()))
}

fn validate_tsv_values(values: &[&str]) -> Result<(), String> {
    for value in values {
        if value.contains(['\t', '\n', '\r']) {
            return Err("descriptor text contains a non-normalized TSV delimiter".to_owned());
        }
    }
    Ok(())
}

const RUNTIME_METADATA_SOURCE: &str = r#"// @generated by rust_iso20022_codegen 0.1.0; do not edit.
// Source: xsds/schema-manifest.json and the normalized schema descriptor IR.

use std::io::Read;
use std::sync::OnceLock;

use super::{FieldDescriptor, FieldKind, MessageDescriptor};

const DATA: &[u8] = include_bytes!("generated.bin");
static DESCRIPTORS: OnceLock<Box<[MessageDescriptor]>> = OnceLock::new();

pub fn all() -> &'static [MessageDescriptor] {
    DESCRIPTORS.get_or_init(build).as_ref()
}

fn build() -> Box<[MessageDescriptor]> {
    let mut decompressed = Vec::new();
    brotli_decompressor::Decompressor::new(DATA, 4096)
        .read_to_end(&mut decompressed)
        .expect("validated generated metadata compression");
    let data: &'static str = Box::leak(
        String::from_utf8(decompressed)
            .expect("validated generated metadata UTF-8")
            .into_boxed_str(),
    );
    let mut messages = Vec::new();
    let mut current: Option<[&'static str; 10]> = None;
    let mut fields = Vec::new();
    for line in data.lines() {
        let mut columns = line.split('\t');
        match columns.next() {
            Some("M") => {
                finish_message(&mut messages, current.take(), &mut fields);
                current = Some([
                    required(&mut columns), required(&mut columns), required(&mut columns),
                    required(&mut columns), required(&mut columns), required(&mut columns),
                    required(&mut columns), required(&mut columns), required(&mut columns),
                    required(&mut columns),
                ]);
            }
            Some("F") => {
                let logical_path = required(&mut columns);
                let namespace = required(&mut columns);
                let wire_name = required(&mut columns);
                let rust_name = required(&mut columns);
                let type_name = required(&mut columns);
                let min_occurs = required(&mut columns).parse().expect("generated minOccurs");
                let max = required(&mut columns);
                let enum_text = required(&mut columns);
                let documentation = optional(required(&mut columns));
                let choice = required(&mut columns);
                let kind = match required(&mut columns) {
                    "element" => FieldKind::Element,
                    "attribute" => FieldKind::Attribute,
                    "simple_content" => FieldKind::SimpleContent,
                    _ => unreachable!("validated generated field kind"),
                };
                let enum_values = if enum_text.is_empty() {
                    &[][..]
                } else {
                    Box::leak(enum_text.split('\u{1f}').collect::<Vec<_>>().into_boxed_slice())
                };
                fields.push(FieldDescriptor {
                    logical_path,
                    namespace,
                    wire_name,
                    rust_name,
                    type_name,
                    min_occurs,
                    max_occurs: if max.is_empty() { None } else { Some(max.parse().expect("generated maxOccurs")) },
                    enum_values,
                    documentation,
                    choice_group: if choice.is_empty() { None } else { Some(choice.parse().expect("generated choice group")) },
                    kind,
                });
            }
            _ => unreachable!("validated generated metadata row"),
        }
    }
    finish_message(&mut messages, current, &mut fields);
    messages.into_boxed_slice()
}

fn finish_message(
    messages: &mut Vec<MessageDescriptor>,
    values: Option<[&'static str; 10]>,
    fields: &mut Vec<FieldDescriptor>,
) {
    let Some(values) = values else { return };
    let message_fields = Box::leak(std::mem::take(fields).into_boxed_slice());
    messages.push(MessageDescriptor {
        identity: values[0],
        business_area: values[1],
        namespace: values[2],
        root_element: values[3],
        root_type: values[4],
        description: optional(values[5]),
        schema_path: values[6],
        schema_sha256: values[7],
        generated_module: values[8],
        required_feature: values[9],
        fields: message_fields,
    });
}

fn required(columns: &mut std::str::Split<'static, char>) -> &'static str {
    columns.next().expect("validated generated metadata column")
}

fn optional(value: &'static str) -> Option<&'static str> {
    (!value.is_empty()).then_some(value)
}
"#;

fn collect_complex_fields(
    complex_type: roxmltree::Node<'_, '_>,
    owner: &str,
    namespace: &str,
    enum_types: &BTreeMap<String, Vec<String>>,
    next_choice_group: &mut u32,
    fields: &mut Vec<FieldDescriptor>,
) -> Result<(), String> {
    for child in complex_type.children().filter(roxmltree::Node::is_element) {
        match child.tag_name().name() {
            "sequence" | "all" => collect_particle_fields(
                child,
                owner,
                namespace,
                enum_types,
                None,
                next_choice_group,
                fields,
            )?,
            "choice" => {
                let group = *next_choice_group;
                *next_choice_group += 1;
                collect_particle_fields(
                    child,
                    owner,
                    namespace,
                    enum_types,
                    Some(group),
                    next_choice_group,
                    fields,
                )?;
            }
            "simpleContent" => collect_simple_content(child, owner, namespace, enum_types, fields)?,
            "attribute" => fields.push(attribute_descriptor(child, owner, namespace, enum_types)?),
            _ => {}
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn collect_particle_fields(
    particle: roxmltree::Node<'_, '_>,
    owner: &str,
    namespace: &str,
    enum_types: &BTreeMap<String, Vec<String>>,
    inherited_choice: Option<u32>,
    next_choice_group: &mut u32,
    fields: &mut Vec<FieldDescriptor>,
) -> Result<(), String> {
    for child in particle.children().filter(roxmltree::Node::is_element) {
        match child.tag_name().name() {
            "element" => fields.push(element_descriptor(
                child,
                owner,
                namespace,
                enum_types,
                inherited_choice,
            )?),
            "sequence" | "all" => collect_particle_fields(
                child,
                owner,
                namespace,
                enum_types,
                inherited_choice,
                next_choice_group,
                fields,
            )?,
            "choice" => {
                let group = *next_choice_group;
                *next_choice_group += 1;
                collect_particle_fields(
                    child,
                    owner,
                    namespace,
                    enum_types,
                    Some(group),
                    next_choice_group,
                    fields,
                )?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn collect_simple_content(
    simple_content: roxmltree::Node<'_, '_>,
    owner: &str,
    namespace: &str,
    enum_types: &BTreeMap<String, Vec<String>>,
    fields: &mut Vec<FieldDescriptor>,
) -> Result<(), String> {
    let extension = simple_content
        .children()
        .find(|node| {
            node.is_element() && matches!(node.tag_name().name(), "extension" | "restriction")
        })
        .ok_or_else(|| format!("simpleContent {owner} has no extension/restriction"))?;
    let type_name = extension
        .attribute("base")
        .unwrap_or("xs:string")
        .to_owned();
    fields.push(FieldDescriptor {
        logical_path: format!("{owner}/$value"),
        namespace: namespace.to_owned(),
        wire_name: "$value".to_owned(),
        rust_name: "value".to_owned(),
        enum_values: enum_types.get(&type_name).cloned().unwrap_or_default(),
        type_name,
        min_occurs: 1,
        max_occurs: Some(1),
        documentation: direct_documentation(simple_content),
        choice_group: None,
        kind: FieldKind::SimpleContent,
    });
    for attribute in extension
        .children()
        .filter(|node| node.is_element() && node.tag_name().name() == "attribute")
    {
        fields.push(attribute_descriptor(
            attribute, owner, namespace, enum_types,
        )?);
    }
    Ok(())
}

fn element_descriptor(
    element: roxmltree::Node<'_, '_>,
    owner: &str,
    namespace: &str,
    enum_types: &BTreeMap<String, Vec<String>>,
    choice_group: Option<u32>,
) -> Result<FieldDescriptor, String> {
    let wire_name = required_node_attribute(element, "name", owner)?;
    let type_name = element.attribute("type").unwrap_or("anonymous").to_owned();
    Ok(FieldDescriptor {
        logical_path: format!("{owner}/{wire_name}"),
        namespace: namespace.to_owned(),
        rust_name: rust_name(&wire_name),
        wire_name,
        enum_values: enum_types.get(&type_name).cloned().unwrap_or_default(),
        type_name,
        min_occurs: parse_occurs(element.attribute("minOccurs"), 1, owner)?,
        max_occurs: parse_max_occurs(element.attribute("maxOccurs"), owner)?,
        documentation: direct_documentation(element),
        choice_group,
        kind: FieldKind::Element,
    })
}

fn attribute_descriptor(
    attribute: roxmltree::Node<'_, '_>,
    owner: &str,
    namespace: &str,
    enum_types: &BTreeMap<String, Vec<String>>,
) -> Result<FieldDescriptor, String> {
    let wire_name = required_node_attribute(attribute, "name", owner)?;
    let type_name = attribute
        .attribute("type")
        .unwrap_or("xs:string")
        .to_owned();
    Ok(FieldDescriptor {
        logical_path: format!("{owner}/@{wire_name}"),
        namespace: namespace.to_owned(),
        rust_name: rust_name(&wire_name),
        wire_name,
        enum_values: enum_types.get(&type_name).cloned().unwrap_or_default(),
        type_name,
        min_occurs: u32::from(attribute.attribute("use") == Some("required")),
        max_occurs: Some(1),
        documentation: direct_documentation(attribute),
        choice_group: None,
        kind: FieldKind::Attribute,
    })
}

fn enum_types(schema: roxmltree::Node<'_, '_>) -> BTreeMap<String, Vec<String>> {
    let mut result = BTreeMap::new();
    for simple_type in schema
        .children()
        .filter(|node| node.is_element() && node.tag_name().name() == "simpleType")
    {
        let Some(name) = simple_type.attribute("name") else {
            continue;
        };
        let values: Vec<String> = simple_type
            .descendants()
            .filter(|node| node.is_element() && node.tag_name().name() == "enumeration")
            .filter_map(|node| node.attribute("value").map(str::to_owned))
            .collect();
        if !values.is_empty() {
            result.insert(name.to_owned(), values);
        }
    }
    result
}

fn direct_documentation(node: roxmltree::Node<'_, '_>) -> Option<String> {
    node.children()
        .find(|child| child.is_element() && child.tag_name().name() == "annotation")?
        .descendants()
        .find(|child| child.is_element() && child.tag_name().name() == "documentation")?
        .text()
        .map(|text| text.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|text| !text.is_empty())
}

fn normalized_identity(basename: &str) -> Option<String> {
    let name = basename.strip_suffix(".xsd")?;
    let parts: Vec<&str> = name.split('.').collect();
    if parts.len() != 4 {
        return None;
    }
    let version = parts[3].split('_').next().unwrap_or(parts[3]);
    (parts[0].len() == 4 && parts[1].len() == 3 && parts[2].len() == 3 && version.len() == 2)
        .then(|| format!("{}.{}.{}.{}", parts[0], parts[1], parts[2], version))
}

fn rust_name(wire_name: &str) -> String {
    let snake = wire_name.to_snake_case();
    if is_keyword(&snake) {
        format!("_{snake}")
    } else {
        snake
    }
}

fn is_keyword(value: &str) -> bool {
    matches!(
        value,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
            | "try"
            | "gen"
    )
}

fn parse_occurs(value: Option<&str>, default: u32, owner: &str) -> Result<u32, String> {
    value
        .map(|value| {
            value
                .parse()
                .map_err(|error| format!("invalid minOccurs {value} in {owner}: {error}"))
        })
        .unwrap_or(Ok(default))
}

fn parse_max_occurs(value: Option<&str>, owner: &str) -> Result<Option<u32>, String> {
    match value {
        Some("unbounded") => Ok(None),
        Some(value) => value
            .parse()
            .map(Some)
            .map_err(|error| format!("invalid maxOccurs {value} in {owner}: {error}")),
        None => Ok(Some(1)),
    }
}

fn required_attribute(
    node: roxmltree::Node<'_, '_>,
    name: &str,
    path: &Path,
) -> Result<String, String> {
    node.attribute(name)
        .map(str::to_owned)
        .ok_or_else(|| format!("{} node has no {name}", path.display()))
}

fn required_node_attribute(
    node: roxmltree::Node<'_, '_>,
    name: &str,
    owner: &str,
) -> Result<String, String> {
    node.attribute(name)
        .map(str::to_owned)
        .ok_or_else(|| format!("{owner} declaration has no {name}"))
}
