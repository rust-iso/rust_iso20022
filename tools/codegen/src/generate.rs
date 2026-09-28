use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Stage {
    root: PathBuf,
    output: PathBuf,
    catalogue: PathBuf,
    manifest: PathBuf,
}

impl Stage {
    pub fn new(output: &Path) -> Result<Self, String> {
        let parent = output.parent().unwrap_or_else(|| Path::new("."));
        fs::create_dir_all(parent)
            .map_err(|error| format!("create output parent {}: {error}", parent.display()))?;
        let root = parent.join(format!(".codegen-stage-{}", std::process::id()));
        if root.exists() {
            fs::remove_dir_all(&root)
                .map_err(|error| format!("remove stale stage {}: {error}", root.display()))?;
        }
        let staged_output_name = output
            .file_name()
            .ok_or_else(|| format!("output has no directory name: {}", output.display()))?;
        let staged_output = root.join(staged_output_name);
        fs::create_dir_all(&staged_output)
            .map_err(|error| format!("create stage {}: {error}", staged_output.display()))?;
        Ok(Self {
            catalogue: root.join("catalogue-data.rs"),
            manifest: root.join("generator-manifest.json"),
            root,
            output: staged_output,
        })
    }

    pub fn output(&self) -> &Path {
        &self.output
    }

    pub fn catalogue(&self) -> &Path {
        &self.catalogue
    }

    pub fn write_manifest(&self, bytes: &[u8]) -> Result<(), String> {
        fs::write(&self.manifest, bytes)
            .map_err(|error| format!("write {}: {error}", self.manifest.display()))
    }

    pub fn artifact(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }

    pub fn format_rust(&self, edition: &str) -> Result<(), String> {
        let mut files = Vec::new();
        collect_files(&self.root, &mut files)?;
        files.retain(|path| path.extension().is_some_and(|extension| extension == "rs"));
        files.sort();
        for chunk in files.chunks(64) {
            let output = Command::new("rustfmt")
                .arg("--edition")
                .arg(edition)
                .args(chunk)
                .output()
                .map_err(|error| format!("run rustfmt: {error}"))?;
            if !output.status.success() {
                return Err(format!(
                    "rustfmt failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
        Ok(())
    }

    pub fn verify_matches(
        &self,
        output: &Path,
        catalogue: &Path,
        manifest: Option<&Path>,
        artifacts: &[(PathBuf, PathBuf)],
    ) -> Result<(), String> {
        let expected = tree_bytes(&self.output)?;
        let actual = tree_bytes(output)
            .map_err(|error| format!("generated output is missing or unreadable: {error}"))?;
        if expected != actual {
            return Err(format!(
                "generated output {} is stale, missing, extra, or tampered",
                output.display()
            ));
        }
        let expected_catalogue =
            fs::read(&self.catalogue).map_err(|error| format!("read staged catalogue: {error}"))?;
        let actual_catalogue = fs::read(catalogue)
            .map_err(|error| format!("read catalogue {}: {error}", catalogue.display()))?;
        if expected_catalogue != actual_catalogue {
            return Err(format!(
                "catalogue {} is stale or tampered",
                catalogue.display()
            ));
        }
        if let Some(manifest) = manifest {
            let expected_manifest = fs::read(&self.manifest)
                .map_err(|error| format!("read staged generator manifest: {error}"))?;
            let actual_manifest = fs::read(manifest)
                .map_err(|error| format!("read {}: {error}", manifest.display()))?;
            if expected_manifest != actual_manifest {
                return Err(format!(
                    "generator manifest {} is stale or tampered",
                    manifest.display()
                ));
            }
        }
        for (staged, target) in artifacts {
            let expected = fs::read(staged)
                .map_err(|error| format!("read staged artifact {}: {error}", staged.display()))?;
            let actual = fs::read(target)
                .map_err(|error| format!("read artifact {}: {error}", target.display()))?;
            if expected != actual {
                return Err(format!(
                    "generated artifact {} is stale or tampered",
                    target.display()
                ));
            }
        }
        Ok(())
    }

    pub fn install(
        mut self,
        output: &Path,
        catalogue: &Path,
        manifest: Option<&Path>,
        artifacts: Vec<(PathBuf, PathBuf)>,
    ) -> Result<(), String> {
        let suffix = format!("codegen-backup-{}", std::process::id());
        let mut installs = vec![
            (self.output.clone(), output.to_path_buf()),
            (self.catalogue.clone(), catalogue.to_path_buf()),
        ];
        if let Some(manifest) = manifest {
            if !self.manifest.is_file() {
                return Err("staged generator manifest is missing".to_owned());
            }
            installs.push((self.manifest.clone(), manifest.to_path_buf()));
        }
        installs.extend(artifacts);
        let backups: Vec<_> = installs
            .iter()
            .map(|(_, target)| target.with_extension(&suffix))
            .collect();
        for backup in &backups {
            remove_existing(backup)?;
        }
        for (_, target) in &installs {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)
                    .map_err(|error| format!("create {}: {error}", parent.display()))?;
            }
        }
        for index in 0..installs.len() {
            let target = &installs[index].1;
            if target.exists() {
                if let Err(error) = fs::rename(target, &backups[index]) {
                    for restore_index in 0..index {
                        restore(&backups[restore_index], &installs[restore_index].1);
                    }
                    return Err(format!("move {} to backup: {error}", target.display()));
                }
            }
        }
        for index in 0..installs.len() {
            if let Err(error) = fs::rename(&installs[index].0, &installs[index].1) {
                for installed in installs.iter().take(index) {
                    remove_existing(&installed.1)?;
                }
                for (backup, (_, target)) in backups.iter().zip(&installs) {
                    restore(backup, target);
                }
                return Err(format!("install {}: {error}", installs[index].1.display()));
            }
        }
        for backup in &backups {
            remove_existing(backup)?;
        }
        fs::remove_dir_all(&self.root)
            .map_err(|error| format!("remove stage {}: {error}", self.root.display()))?;
        self.root = PathBuf::new();
        Ok(())
    }
}

impl Drop for Stage {
    fn drop(&mut self) {
        if !self.root.as_os_str().is_empty() && self.root.exists() {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

fn collect_files(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let mut entries: Vec<_> = fs::read_dir(root)
        .map_err(|error| format!("read tree {}: {error}", root.display()))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<_, _>>()
        .map_err(|error| format!("read tree entry: {error}"))?;
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

fn tree_bytes(root: &Path) -> Result<Vec<(PathBuf, Vec<u8>)>, String> {
    let mut files = Vec::new();
    collect_files(root, &mut files)?;
    files
        .into_iter()
        .map(|path| {
            let relative = path
                .strip_prefix(root)
                .map_err(|error| format!("relative path {}: {error}", path.display()))?
                .to_path_buf();
            let bytes = fs::read(&path)
                .map_err(|error| format!("read generated file {}: {error}", path.display()))?;
            Ok((relative, bytes))
        })
        .collect()
}

fn remove_existing(path: &Path) -> Result<(), String> {
    if path.is_dir() {
        fs::remove_dir_all(path).map_err(|error| format!("remove {}: {error}", path.display()))
    } else if path.exists() {
        fs::remove_file(path).map_err(|error| format!("remove {}: {error}", path.display()))
    } else {
        Ok(())
    }
}

fn restore(backup: &Path, destination: &Path) {
    if backup.exists() {
        let _ = remove_existing(destination);
        let _ = fs::rename(backup, destination);
    }
}
