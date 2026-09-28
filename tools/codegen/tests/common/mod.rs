#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("codegen crate is tools/codegen")
        .to_path_buf()
}

pub fn run_codegen(cwd: &Path, input: &Path, output: &Path, extra: &[&str]) -> Output {
    fs::create_dir_all(cwd.join("src/catalogue")).expect("create catalogue fixture directory");
    let mut command = Command::new(env!("CARGO_BIN_EXE_codegen"));
    command
        .current_dir(cwd)
        .arg("--input")
        .arg(input)
        .arg("--output")
        .arg(output)
        .arg("--allow-unmanifested-input")
        .args(extra);
    command.output().expect("run codegen")
}

pub fn output_text(output: &Output) -> String {
    format!(
        "status: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

pub fn tree_bytes(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    fn visit(base: &Path, path: &Path, files: &mut Vec<(PathBuf, Vec<u8>)>) {
        let mut entries: Vec<_> = fs::read_dir(path)
            .expect("read generated tree")
            .map(|entry| entry.expect("read directory entry").path())
            .collect();
        entries.sort();
        for entry in entries {
            if entry.is_dir() {
                visit(base, &entry, files);
            } else {
                files.push((
                    entry
                        .strip_prefix(base)
                        .expect("relative path")
                        .to_path_buf(),
                    fs::read(&entry).expect("read generated file"),
                ));
            }
        }
    }

    let mut files = Vec::new();
    visit(root, root, &mut files);
    files
}
