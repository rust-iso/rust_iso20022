#!/usr/bin/env python3
"""Fail if bounded CI omits messages, alters implementations or hides new guards."""

from collections import Counter
import importlib.util
from pathlib import Path
import re
import sys
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[1]
sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location("model_shards", ROOT / "scripts/test-model-shards.py")
sharding = importlib.util.module_from_spec(spec)
spec.loader.exec_module(sharding)


class ModelShardContracts(unittest.TestCase):
    def test_workflow_covers_every_model_family(self):
        workflow = (ROOT / ".github/workflows/ci.yml").read_text()
        ordinary = workflow.split("  generated-area:\n", 1)[1].split("  model-shard-plan:\n", 1)[0]
        areas = re.findall(r"(?m)^          - ([a-z]{4})$", ordinary)
        all_areas = {p.name for p in (ROOT / "src/generated").iterdir()
                     if p.is_dir() and (p / "mod.rs").is_file()}
        self.assertEqual(len(areas), len(set(areas)))
        self.assertFalse(set(areas) & set(sharding.AREAS))
        self.assertEqual(set(areas) | set(sharding.AREAS), all_areas)

    def test_every_message_has_bounded_coverage(self):
        for area in sharding.AREAS:
            with self.subTest(area=area):
                files = sharding.inventory(ROOT, area)
                groups = sharding.shards(ROOT, area)
                repeated = sharding.CAMT_DEPENDENCIES if area == "camt" else set()
                counts = Counter(name for group in groups for name in group)
                self.assertEqual(set(counts), set(files))
                for name, count in counts.items():
                    self.assertEqual(count, len(groups) if name in repeated else 1)
                for group in groups:
                    self.assertLessEqual(sum(files[name] for name in group),
                                         sharding.MAX_SOURCE_BYTES)
                    self.assertTrue(repeated <= group)

    def test_every_dispatch_section_is_assigned_to_its_message(self):
        source = (ROOT / "src/generated/any.rs").read_text()
        for area in sharding.AREAS:
            models = set(sharding.inventory(ROOT, area))
            self.assertEqual(sharding.mask_dispatch(source, area, models, models), source)
            for selected in sharding.shards(ROOT, area):
                masked = sharding.mask_dispatch(source, area, selected, models)
                # Every false cfg belongs to an unselected message, never a generic API.
                matches = list(sharding.MODEL_CFG.finditer(masked))
                for i, match in enumerate(matches):
                    if f'model-{area}"' not in match[0]:
                        continue
                    end = matches[i + 1].start() if i + 1 < len(matches) else len(masked)
                    names = {name.lower().replace(".", "_") for name in re.findall(
                        rf"\b{area}[_.]\d{{3}}[_.]\d{{3}}[_.]\d{{2}}\b",
                        masked[match.end():end], re.I)}
                    self.assertEqual(len(names), 1)
                    disabled = masked[:match.start()].rstrip().endswith("#[cfg(any())]")
                    self.assertEqual(disabled, not names <= selected)

    def test_index_masks_both_modules_and_their_smoke_tests(self):
        source = "pub mod caaa_001_001_10;\npub mod caaa_002_001_10;\n" + (
            "    #[test]\n    fn caaa_001_001_10() {}\n"
            "    #[test]\n    fn caaa_002_001_10() {}\n")
        masked = sharding.mask_index(source, {"caaa_001_001_10"})
        self.assertIn("#[cfg(any())]\npub mod caaa_002_001_10;", masked)
        self.assertIn("    #[cfg(any())]\n    #[test]\n    fn caaa_002_001_10()", masked)
        self.assertEqual(masked.count("#[cfg(any())]"), 2)

    def test_unknown_or_missing_dispatch_sections_fail_closed(self):
        source = '#[cfg(feature = "model-caaa")]\n"caaa.001.001.10" => true,\n'
        with self.assertRaises(ValueError):
            sharding.mask_dispatch(source, "caaa", set(), {"caaa_002_001_10"})
        with self.assertRaises(ValueError):
            sharding.mask_dispatch(source.replace("caaa.001.001.10", "generic_api"),
                                    "caaa", set(), {"caaa_001_001_10"})
        with self.assertRaises(ValueError):
            sharding.mask_dispatch(source, "caaa", set(),
                                    {"caaa_001_001_10", "caaa_002_001_10"})

    def test_snapshot_changes_only_two_indexes_and_preserves_checkout(self):
        before = sharding.source_digest(ROOT)
        with tempfile.TemporaryDirectory() as temp:
            copy = Path(temp)
            sharding.snapshot(ROOT, copy, "camt", sharding.shards(ROOT, "camt")[0])
            changed = {str(path.relative_to(ROOT)) for path in (ROOT / "src").rglob("*.rs")
                       if path.read_bytes() != (copy / path.relative_to(ROOT)).read_bytes()}
            self.assertEqual(changed, {"src/generated/camt/mod.rs", "src/generated/any.rs"})
            for name in ("Cargo.toml", "Cargo.lock"):
                self.assertEqual((ROOT / name).read_bytes(), (copy / name).read_bytes())
        self.assertEqual(before, sharding.source_digest(ROOT))


if __name__ == "__main__":
    unittest.main()
