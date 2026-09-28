use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const SCHEMA_MANIFEST_FORMAT: u32 = 1;
pub const SCHEMA_SET_ID: &str = "rust-iso20022-legacy-baseline-1130";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaManifest {
    pub format_version: u32,
    pub schema_set_id: String,
    pub schema_count: usize,
    pub schemas: Vec<SchemaRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaRecord {
    pub identity: String,
    pub path: String,
    pub provenance_status: String,
    pub source_url: Option<String>,
    pub schema_release: Option<String>,
    pub sha256: String,
    pub byte_length: u64,
    pub namespace: String,
    pub root_element: String,
    pub root_type: String,
    pub generated_module: String,
    pub generation_status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaSet {
    pub format_version: u32,
    pub schema_set_id: String,
    pub schema_count: usize,
    pub provenance_status: String,
    pub source: SchemaSource,
    pub exceptions: Vec<SchemaException>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaSource {
    pub kind: String,
    pub url: Option<String>,
    pub release: Option<String>,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaException {
    pub identity: String,
    pub kind: String,
    pub detail: String,
}

pub fn build_schema_manifest(
    input: &Path,
    workspace_root: &Path,
) -> Result<SchemaManifest, String> {
    let mut paths: Vec<PathBuf> = fs::read_dir(input)
        .map_err(|error| format!("read schema directory {}: {error}", input.display()))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<_, _>>()
        .map_err(|error| format!("read schema directory entry: {error}"))?;
    paths.retain(|path| path.extension().is_some_and(|extension| extension == "xsd"));
    paths.sort();

    let mut identities = BTreeSet::new();
    let mut schemas = Vec::with_capacity(paths.len());
    for path in paths {
        let basename = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| format!("non-UTF-8 schema path: {}", path.display()))?;
        let (family, stem, identity) = normalized_identity(basename)
            .ok_or_else(|| format!("non-canonical schema filename: {basename}"))?;
        if !identities.insert(identity.clone()) {
            return Err(format!("duplicate normalized schema identity: {identity}"));
        }

        let bytes =
            fs::read(&path).map_err(|error| format!("read schema {}: {error}", path.display()))?;
        let text = std::str::from_utf8(&bytes)
            .map_err(|error| format!("schema {} is not UTF-8: {error}", path.display()))?;
        let namespace = extract_attribute(text, "targetNamespace")
            .ok_or_else(|| format!("schema {basename} has no targetNamespace"))?;
        let root_tag = first_element_tag(text)
            .ok_or_else(|| format!("schema {basename} has no root element"))?;
        let root_element = extract_attribute(root_tag, "name")
            .ok_or_else(|| format!("schema {basename} root has no name"))?;
        let root_type = extract_attribute(root_tag, "type")
            .ok_or_else(|| format!("schema {basename} root has no type"))?;
        let relative_path = if path.is_relative() {
            path.clone()
        } else {
            let canonical_root = workspace_root.canonicalize().map_err(|error| {
                format!(
                    "canonicalize workspace {}: {error}",
                    workspace_root.display()
                )
            })?;
            path.strip_prefix(&canonical_root)
                .map_err(|_| {
                    format!(
                        "schema {} is outside workspace {}",
                        path.display(),
                        workspace_root.display()
                    )
                })?
                .to_path_buf()
        };
        let relative = relative_path.to_string_lossy().replace('\\', "/");

        schemas.push(SchemaRecord {
            identity,
            path: relative,
            provenance_status: "unverified".to_owned(),
            source_url: None,
            schema_release: None,
            sha256: format!("{:x}", Sha256::digest(&bytes)),
            byte_length: bytes.len() as u64,
            namespace,
            root_element,
            root_type,
            generated_module: format!("src/generated/{family}/{stem}.rs"),
            generation_status: "generated".to_owned(),
        });
    }

    Ok(SchemaManifest {
        format_version: SCHEMA_MANIFEST_FORMAT,
        schema_set_id: SCHEMA_SET_ID.to_owned(),
        schema_count: schemas.len(),
        schemas,
    })
}

pub fn schema_set(schema_count: usize) -> SchemaSet {
    SchemaSet {
        format_version: SCHEMA_MANIFEST_FORMAT,
        schema_set_id: SCHEMA_SET_ID.to_owned(),
        schema_count,
        provenance_status: "unverified".to_owned(),
        source: SchemaSource {
            kind: "repository-legacy-snapshot".to_owned(),
            url: None,
            release: None,
            note: "The repository did not retain authoritative download/release evidence; provenance is intentionally not inferred from schema comments.".to_owned(),
        },
        exceptions: vec![
            SchemaException {
                identity: "acmt.017.001.03".to_owned(),
                kind: "normalized-filename".to_owned(),
                detail: "Input basename acmt.017.001.03_0.xsd normalizes to the canonical message identity.".to_owned(),
            },
            SchemaException {
                identity: "semt.001.001.04".to_owned(),
                kind: "namespace".to_owned(),
                detail: "The schema declares urn:swift:xsd:semt.001.001.04 and that namespace is preserved verbatim.".to_owned(),
            },
            SchemaException {
                identity: "root-elements".to_owned(),
                kind: "root-distribution".to_owned(),
                detail: "The baseline contains 1127 Document roots, two AppHdr roots, and one Xchg root.".to_owned(),
            },
        ],
    }
}

pub fn write_manifest_files(input: &Path, workspace_root: &Path) -> Result<(), String> {
    let manifest = build_schema_manifest(input, workspace_root)?;
    let set = schema_set(manifest.schema_count);
    write_json(&workspace_root.join("xsds/schema-set.json"), &set)?;
    write_json(&workspace_root.join("xsds/schema-manifest.json"), &manifest)?;
    Ok(())
}

pub fn verify_manifest_files(input: &Path, workspace_root: &Path) -> Result<(), String> {
    let manifest_path = input.join("schema-manifest.json");
    let set_path = input.join("schema-set.json");
    let tracked_manifest: SchemaManifest = serde_json::from_slice(
        &fs::read(&manifest_path)
            .map_err(|error| format!("read {}: {error}", manifest_path.display()))?,
    )
    .map_err(|error| format!("parse {}: {error}", manifest_path.display()))?;
    let tracked_set: SchemaSet = serde_json::from_slice(
        &fs::read(&set_path).map_err(|error| format!("read {}: {error}", set_path.display()))?,
    )
    .map_err(|error| format!("parse {}: {error}", set_path.display()))?;
    let actual_manifest = build_schema_manifest(input, workspace_root)?;
    let actual_set = schema_set(actual_manifest.schema_count);

    if tracked_manifest != actual_manifest {
        return Err(format!(
            "{} does not match the current raw schema inputs; run --write-manifests only after reviewing the schema change",
            manifest_path.display()
        ));
    }
    if tracked_set != actual_set {
        return Err(format!(
            "{} does not match the declared schema set",
            set_path.display()
        ));
    }
    Ok(())
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let mut bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| format!("serialize {}: {error}", path.display()))?;
    bytes.push(b'\n');
    fs::write(path, bytes).map_err(|error| format!("write {}: {error}", path.display()))
}

fn normalized_identity(basename: &str) -> Option<(String, String, String)> {
    let name = basename.strip_suffix(".xsd")?;
    let parts: Vec<&str> = name.split('.').collect();
    if parts.len() != 4 {
        return None;
    }
    let version = parts[3].split('_').next().unwrap_or(parts[3]);
    let canonical = parts[0].len() == 4
        && parts[0].bytes().all(|byte| byte.is_ascii_lowercase())
        && parts[1].len() == 3
        && parts[2].len() == 3
        && version.len() == 2
        && parts[1..3]
            .iter()
            .all(|part| part.bytes().all(|byte| byte.is_ascii_digit()))
        && version.bytes().all(|byte| byte.is_ascii_digit());
    canonical.then(|| {
        let family = parts[0].to_owned();
        let identity = format!("{}.{}.{}.{}", parts[0], parts[1], parts[2], version);
        let stem = identity.replace('.', "_");
        (family, stem, identity)
    })
}

fn extract_attribute(text: &str, name: &str) -> Option<String> {
    for quote in ['"', '\''] {
        let marker = format!("{name}={quote}");
        if let Some(start) = text.find(&marker) {
            let value = &text[start + marker.len()..];
            let end = value.find(quote)?;
            return Some(value[..end].to_owned());
        }
    }
    None
}

fn first_element_tag(text: &str) -> Option<&str> {
    let xs = text.find("<xs:element");
    let xsd = text.find("<xsd:element");
    let start = match (xs, xsd) {
        (Some(left), Some(right)) => left.min(right),
        (Some(start), None) | (None, Some(start)) => start,
        (None, None) => return None,
    };
    let end = text[start..].find('>')? + start + 1;
    Some(&text[start..end])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_mirror_suffix() {
        assert_eq!(
            normalized_identity("acmt.017.001.03_0.xsd"),
            Some((
                "acmt".to_owned(),
                "acmt_017_001_03".to_owned(),
                "acmt.017.001.03".to_owned()
            ))
        );
    }

    #[test]
    fn extracts_first_top_level_convention_element() {
        let schema = r#"<xs:schema targetNamespace="urn:test"><xs:element name="Document" type="Document"/><xs:complexType name="Document"/></xs:schema>"#;
        let tag = first_element_tag(schema).expect("root tag");
        assert_eq!(extract_attribute(tag, "name").as_deref(), Some("Document"));
        assert_eq!(extract_attribute(tag, "type").as_deref(), Some("Document"));
    }
}
