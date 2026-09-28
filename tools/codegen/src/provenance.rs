use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const GENERATOR_MANIFEST_FORMAT: u32 = 1;
pub const PARSER_REVISION: &str = "d476e854b28b197442096c268d79263c50300c9e";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorManifest {
    pub format_version: u32,
    pub generator: GeneratorIdentity,
    pub inputs: GeneratorInputs,
    pub metadata_formats: MetadataFormats,
    pub output_tree_sha256: String,
    pub outputs: Vec<OutputRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorIdentity {
    pub package: String,
    pub version: String,
    pub source_sha256: String,
    pub xsd_parser_revision: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorInputs {
    pub schema_manifest_sha256: String,
    pub config_sha256: String,
    pub lockfile_sha256: String,
    pub toolchain_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataFormats {
    pub schema_manifest: u32,
    pub schema_descriptor: u32,
    pub generator_manifest: u32,
    pub generated_header: u32,
    pub catalogue: u32,
    pub output_digest: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputRecord {
    pub path: String,
    pub sha256: String,
    pub byte_length: u64,
}

pub fn build_generator_manifest(
    workspace_root: &Path,
    staged_output: &Path,
    logical_output: &Path,
    staged_catalogue: &Path,
    logical_catalogue: &Path,
    additional_outputs: &[(PathBuf, PathBuf)],
) -> Result<GeneratorManifest, String> {
    let source_paths = [
        Path::new("tools/codegen/Cargo.toml"),
        Path::new("tools/codegen/src/catalogue.rs"),
        Path::new("tools/codegen/src/descriptor.rs"),
        Path::new("tools/codegen/src/dispatch.rs"),
        Path::new("tools/codegen/src/generate.rs"),
        Path::new("tools/codegen/src/main.rs"),
        Path::new("tools/codegen/src/manifest.rs"),
        Path::new("tools/codegen/src/provenance.rs"),
    ];
    let source_sha256 = combined_digest(workspace_root, &source_paths)?;

    let mut staged_files = Vec::new();
    collect_files(staged_output, &mut staged_files)?;
    staged_files.sort();
    let mut outputs = Vec::with_capacity(staged_files.len() + 1);
    for path in staged_files {
        let relative = path
            .strip_prefix(staged_output)
            .map_err(|error| format!("relative output path {}: {error}", path.display()))?;
        outputs.push(output_record(&path, &logical_output.join(relative))?);
    }
    outputs.push(output_record(staged_catalogue, logical_catalogue)?);
    for (staged, logical) in additional_outputs {
        outputs.push(output_record(staged, logical)?);
    }
    outputs.sort_by(|left, right| left.path.cmp(&right.path));

    let mut tree_hasher = Sha256::new();
    for output in &outputs {
        tree_hasher.update(output.path.as_bytes());
        tree_hasher.update([0]);
        tree_hasher.update(output.sha256.as_bytes());
        tree_hasher.update([0]);
        tree_hasher.update(output.byte_length.to_le_bytes());
    }

    Ok(GeneratorManifest {
        format_version: GENERATOR_MANIFEST_FORMAT,
        generator: GeneratorIdentity {
            package: env!("CARGO_PKG_NAME").to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            source_sha256,
            xsd_parser_revision: PARSER_REVISION.to_owned(),
        },
        inputs: GeneratorInputs {
            schema_manifest_sha256: file_digest(&workspace_root.join("xsds/schema-manifest.json"))?,
            config_sha256: file_digest(
                &workspace_root.join("tools/codegen/generator-config.toml"),
            )?,
            lockfile_sha256: file_digest(&workspace_root.join("Cargo.lock"))?,
            toolchain_sha256: file_digest(&workspace_root.join("rust-toolchain.toml"))?,
        },
        metadata_formats: MetadataFormats {
            schema_manifest: 1,
            schema_descriptor: 1,
            generator_manifest: GENERATOR_MANIFEST_FORMAT,
            generated_header: 1,
            catalogue: 1,
            output_digest: 1,
        },
        output_tree_sha256: format!("{:x}", tree_hasher.finalize()),
        outputs,
    })
}

pub fn to_pretty_json(manifest: &GeneratorManifest) -> Result<Vec<u8>, String> {
    let mut bytes = serde_json::to_vec_pretty(manifest)
        .map_err(|error| format!("serialize generator manifest: {error}"))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn output_record(path: &Path, logical_path: &Path) -> Result<OutputRecord, String> {
    let bytes = fs::read(path)
        .map_err(|error| format!("read generated output {}: {error}", path.display()))?;
    Ok(OutputRecord {
        path: logical_path.to_string_lossy().replace('\\', "/"),
        sha256: format!("{:x}", Sha256::digest(&bytes)),
        byte_length: bytes.len() as u64,
    })
}

fn file_digest(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn combined_digest(root: &Path, paths: &[&Path]) -> Result<String, String> {
    let mut sorted: Vec<PathBuf> = paths.iter().map(|path| path.to_path_buf()).collect();
    sorted.sort();
    let mut hasher = Sha256::new();
    for relative in sorted {
        let bytes = fs::read(root.join(&relative))
            .map_err(|error| format!("read {}: {error}", relative.display()))?;
        hasher.update(relative.to_string_lossy().as_bytes());
        hasher.update([0]);
        hasher.update(bytes);
        hasher.update([0]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn collect_files(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let mut entries: Vec<_> = fs::read_dir(root)
        .map_err(|error| format!("read generated tree {}: {error}", root.display()))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<_, _>>()
        .map_err(|error| format!("read generated tree entry: {error}"))?;
    entries.sort();
    for entry in entries {
        if entry.is_dir() {
            collect_files(&entry, files)?;
        } else {
            files.push(entry);
        }
    }
    Ok(())
}
