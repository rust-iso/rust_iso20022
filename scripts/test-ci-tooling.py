#!/usr/bin/env python3
"""Exercise tooling scripts with strict CLI doubles, without downloads or builds."""

import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[1]
STUB = r'''#!/usr/bin/env python3
import json, os, pathlib, sys
args = sys.argv[1:]
mode = os.environ.get("STUB_MODE", "")
root = pathlib.Path.cwd()
with open(root / "calls.jsonl", "a") as log:
    log.write(json.dumps(args) + "\n")
toolchain = args.pop(0)
command = args.pop(0)
if command == "metadata":
    assert toolchain in ("+stable", "+nightly-2026-09-26") and "--locked" in args
    print(json.dumps({"workspace_members": ["core", "adapter"], "packages": [
        {"id": "core", "name": "rust_iso20022", "manifest_path": str(root / "Cargo.toml")},
        {"id": "adapter", "name": "adapter", "manifest_path": str(root / "adapter with spaces/Cargo.toml")}
    ]}))
elif command == "cyclonedx":
    assert toolchain == "+stable"
    if args == ["--version"]:
        print("cargo-cyclonedx " + ("0.5.90" if mode == "wrong-version" else "0.5.9"))
    else:
        prefix = args[args.index("--override-filename") + 1]
        assert "/" not in prefix and "\\" not in prefix
        assert "--all-features" in args and args[args.index("--target") + 1] == "all"
        for name, directory in [("rust_iso20022", root), ("adapter", root / "adapter with spaces")]:
            (directory / (prefix + ".json")).write_text(json.dumps({
                "bomFormat": "CycloneDX", "specVersion": "1.5",
                "metadata": {"component": {"name": "wrong" if mode == "bad-bom" else name}}
            }))
        if mode == "changed-lock":
            (root / "Cargo.lock").write_text("changed\n")
elif command == "fuzz":
    assert toolchain == "+nightly-2026-09-26"
    if args == ["--version"]:
        print("cargo-fuzz " + ("0.13.3" if mode == "wrong-version" else "0.13.2"))
    else:
        assert args[0] in ("run", "cmin")
        assert args[args.index("--codegen-units") + 1] == "16"
        assert args.index("--codegen-units") < args.index("--")
        assert os.environ["CARGO_BUILD_JOBS"] == "1"
        assert os.environ["CARGO_NET_OFFLINE"] == "true"
else:
    raise AssertionError(args)
'''


class ToolingContracts(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="iso tooling ")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        (self.root / "scripts").mkdir()
        (self.root / "bin").mkdir()
        (self.root / "adapter with spaces").mkdir()
        for name in ("generate-sbom.sh", "run-fuzz-baseline.sh"):
            shutil.copyfile(ROOT / "scripts" / name, self.root / "scripts" / name)
        (self.root / "Cargo.toml").write_text("")
        (self.root / "adapter with spaces/Cargo.toml").write_text("")
        (self.root / "Cargo.lock").write_text("original\n")
        cargo = self.root / "bin/cargo"
        cargo.write_text(STUB)
        cargo.chmod(0o755)
        (self.root / "bin/cargo-cyclonedx").symlink_to(cargo)

    def run_script(self, name, mode="", args=()):
        env = dict(os.environ, PATH=str(self.root / "bin") + os.pathsep + os.environ["PATH"],
                   STUB_MODE=mode, SOURCE_DATE_EPOCH="0", FUZZ_TOOLCHAIN="nightly-2026-09-26",
                   CARGO_BUILD_JOBS="1", FUZZ_CODEGEN_UNITS="16")
        return subprocess.run(["bash", str(self.root / "scripts" / name), *args],
                              cwd=self.root, env=env, capture_output=True, text=True)

    def test_collects_workspace_sboms_and_portable_checksums(self):
        result = self.run_script("generate-sbom.sh")
        self.assertEqual(result.returncode, 0, result.stderr)
        output = self.root / "evidence/release-baseline/sbom"
        for name in ("rust_iso20022", "adapter"):
            check = subprocess.run(["shasum", "-a", "256", "-c", name + ".cdx.json.sha256"],
                                   cwd=output, capture_output=True, text=True)
            self.assertEqual(check.returncode, 0, check.stderr)
        self.assertFalse(list(self.root.rglob("iso20022-sbom-*.json")))

    def test_rejects_wrong_tool_version_invalid_bom_and_changed_lock(self):
        for mode in ("wrong-version", "bad-bom", "changed-lock"):
            with self.subTest(mode=mode):
                result = self.run_script("generate-sbom.sh", mode)
                self.assertNotEqual(result.returncode, 0)
                self.assertFalse(list(self.root.rglob("iso20022-sbom-*.json")))

    def test_fuzz_requires_pinned_version_before_running_targets(self):
        result = self.run_script("run-fuzz-baseline.sh", "wrong-version")
        self.assertEqual(result.returncode, 2, result.stderr)
        calls = self.root / "calls.jsonl"
        self.assertEqual(len(calls.read_text().splitlines()), 1)
        calls.unlink()
        result = self.run_script("run-fuzz-baseline.sh")
        self.assertEqual(result.returncode, 0, result.stderr)
        runs = [json.loads(line) for line in calls.read_text().splitlines()]
        self.assertEqual([call[3] for call in runs if call[1:3] == ["fuzz", "run"]],
                         ["xml", "detect", "namespace", "financial", "mt_parser", "mt103", "mt202", "mt940"])

    def test_minimization_keeps_the_same_bounded_compiler_settings(self):
        result = self.run_script("run-fuzz-baseline.sh", args=("--minimize",))
        self.assertEqual(result.returncode, 0, result.stderr)
        runs = [json.loads(line) for line in (self.root / "calls.jsonl").read_text().splitlines()]
        self.assertEqual([call[3] for call in runs if call[1:3] == ["fuzz", "cmin"]],
                         ["xml", "detect", "namespace", "financial", "mt_parser", "mt103", "mt202", "mt940"])


if __name__ == "__main__":
    unittest.main()
