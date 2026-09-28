//! Executable schema-corpus inventory for XSD-backend qualification.
//!
//! This deliberately has no validator dependency: candidates that do not meet
//! the repository MSRV must not enter the product dependency graph merely to
//! run the qualification probe.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const CONSTRUCTS: &[(&str, &str)] = &[
    ("import", "<xs:import"),
    ("include", "<xs:include"),
    ("key", "<xs:key"),
    ("keyref", "<xs:keyref"),
    ("unique", "<xs:unique"),
    ("assert", "<xs:assert"),
    ("alternative", "<xs:alternative"),
    ("redefine", "<xs:redefine"),
    ("any", "<xs:any"),
    ("union", "<xs:union"),
    ("list", "<xs:list"),
];

fn main() -> Result<(), String> {
    let root = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("xsds"));
    let files = xsd_files(&root)?;
    if files.is_empty() {
        return Err(format!("no XSD files under {}", root.display()));
    }

    let mut counts = vec![0usize; CONSTRUCTS.len()];
    for path in &files {
        let schema = fs::read_to_string(path)
            .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
        for (index, (_, needle)) in CONSTRUCTS.iter().enumerate() {
            counts[index] += schema.matches(needle).count();
        }
    }

    println!("schemas={}", files.len());
    for ((name, _), count) in CONSTRUCTS.iter().zip(counts) {
        println!("{name}={count}");
    }
    Ok(())
}

fn xsd_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(root)
        .map_err(|error| format!("failed to list {}: {error}", root.display()))?
    {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.extension().and_then(|extension| extension.to_str()) == Some("xsd") {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::CONSTRUCTS;

    #[test]
    fn candidate_blocking_constructs_are_explicit() {
        for required in ["import", "include", "key", "keyref", "unique", "any"] {
            assert!(CONSTRUCTS.iter().any(|(name, _)| *name == required));
        }
    }
}
