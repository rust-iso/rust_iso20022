#!/usr/bin/env python3
"""Test large model families in disposable source snapshots, without API changes.

Only module declarations, generated smoke-test declarations and dispatch cfgs
are masked in the snapshot. Message implementations, builders, core, features
and the dependency lockfile are the original files. Every message is tested;
CAMT's three builder dependencies are also compiled in every CAMT shard.
"""

import argparse
from collections import Counter
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile


ROOT = Path(__file__).resolve().parents[1]
AREAS = ("caaa", "camt")
MAX_SOURCE_BYTES = 2 * 1024 * 1024
CAMT_DEPENDENCIES = {"camt_052_001_09", "camt_053_001_09", "camt_054_001_09"}
MODEL_CFG = re.compile(r'(?m)^[ \t]*#\[cfg\([^\n]*model-[^\n]*\)\]\n')


def inventory(root, area):
    folder = root / "src/generated" / area
    files = {p.stem: p.stat().st_size for p in folder.glob(f"{area}_*.rs")}
    index = (folder / "mod.rs").read_text()
    declarations = re.findall(r"(?m)^pub mod (\w+);$", index)
    smoke_tests = re.findall(r"(?m)^    fn (\w+)\(\)", index)
    if not files or Counter(declarations) != Counter(files.keys()):
        raise ValueError(f"{area}: module declarations do not match model files")
    if Counter(smoke_tests) != Counter(files.keys()):
        raise ValueError(f"{area}: every model must have exactly one smoke test")
    return files


def shards(root, area):
    files = inventory(root, area)
    required = CAMT_DEPENDENCIES if area == "camt" else set()
    if not required <= files.keys():
        raise ValueError("CAMT builder dependencies are missing")
    base_size = sum(files[name] for name in required)
    if base_size > MAX_SOURCE_BYTES:
        raise ValueError(f"{area}: builder dependencies exceed the shard budget")
    groups, sizes = [], []
    # Largest-first packing keeps the bound independent of message numbering.
    for name in sorted(files.keys() - required, key=lambda n: (-files[n], n)):
        if base_size + files[name] > MAX_SOURCE_BYTES:
            raise ValueError(f"{name}: exceeds shard budget; review runner capacity")
        candidates = [i for i, size in enumerate(sizes)
                      if size + files[name] <= MAX_SOURCE_BYTES]
        if candidates:
            i = min(candidates, key=lambda i: sizes[i])
        else:
            i = len(groups)
            groups.append(set(required))
            sizes.append(base_size)
        groups[i].add(name)
        sizes[i] += files[name]
    if not groups:
        groups = [set(required)]
    counts = Counter(name for group in groups for name in group if name not in required)
    if counts != Counter(files.keys() - required):
        raise ValueError(f"{area}: shards lost or duplicated messages")
    return groups


def mask_index(source, selected):
    def declaration(match):
        return ("#[cfg(any())]\n" if match[1] not in selected else "") + match[0]

    def smoke_test(match):
        return ("    #[cfg(any())]\n" if match[1] not in selected else "") + match[0]

    source = re.sub(r"(?m)^pub mod (\w+);$", declaration, source)
    return re.sub(r"(?m)^    #\[test\]\n    fn (\w+)\(\)", smoke_test, source)


def mask_dispatch(source, area, selected, models):
    guards = list(MODEL_CFG.finditer(source))
    edits, counts = [], Counter()
    for i, guard in enumerate(guards):
        if f'model-{area}"' not in guard[0]:
            continue
        end = guards[i + 1].start() if i + 1 < len(guards) else len(source)
        body = source[guard.end():end]
        names = {name.lower().replace(".", "_") for name in re.findall(
            rf"\b{area}[_.]\d{{3}}[_.]\d{{3}}[_.]\d{{2}}\b", body, re.I)}
        if len(names) != 1 or not names <= models:
            raise ValueError(f"{area}: unrecognized dispatch guard: {guard[0].strip()}")
        name = names.pop()
        counts[name] += 1
        if name not in selected:
            indent = re.match(r"[ \t]*", guard[0])[0]
            edits.append((guard.start(), indent + "#[cfg(any())]\n"))
    if counts.keys() != models or len(set(counts.values())) != 1:
        raise ValueError(f"{area}: incomplete or inconsistent dispatch coverage")
    for start, extra_guard in reversed(edits):
        source = source[:start] + extra_guard + source[start:]
    return source


def snapshot(root, destination, area, selected):
    paths = subprocess.check_output(
        ["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard"], cwd=root
    ).decode().split("\0")
    for relative in filter(None, paths):
        source = root / relative
        if source.is_file():
            target = destination / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source, target)
    index = destination / "src/generated" / area / "mod.rs"
    index.write_text(mask_index(index.read_text(), selected))
    dispatch = destination / "src/generated/any.rs"
    dispatch.write_text(mask_dispatch(dispatch.read_text(), area, selected,
                                      set(inventory(root, area))))
    # Never allow dependency drift during this CI-only operation.
    if (root / "Cargo.lock").read_bytes() != (destination / "Cargo.lock").read_bytes():
        raise ValueError("snapshot changed Cargo.lock")


def source_digest(root):
    digest = hashlib.sha256()
    for path in sorted((root / "src").rglob("*.rs")):
        digest.update(str(path.relative_to(root)).encode())
        digest.update(path.read_bytes())
    for name in ("Cargo.toml", "Cargo.lock"):
        digest.update((root / name).read_bytes())
    return digest.hexdigest()


def run_shard(area, number):
    groups = shards(ROOT, area)
    if not 0 <= number < len(groups):
        raise ValueError(f"{area}: invalid shard {number}; expected 0..{len(groups) - 1}")
    selected = groups[number]
    files = inventory(ROOT, area)
    report = {"area": area, "shard": number, "shards": len(groups),
              "source_bytes": sum(files[name] for name in selected),
              "messages": sorted(selected)}
    print(json.dumps(report, indent=2), flush=True)
    before = source_digest(ROOT)
    env = dict(os.environ)
    for key, value in {"CARGO_BUILD_JOBS": "1", "CARGO_INCREMENTAL": "0",
                       "CARGO_PROFILE_DEV_DEBUG": "0", "CARGO_PROFILE_TEST_DEBUG": "0"}.items():
        env[key] = value
    env["CARGO_TARGET_DIR"] = str(ROOT / "target/model-shards")
    try:
        with tempfile.TemporaryDirectory(prefix="iso-model-shard-") as temp:
            copy = Path(temp)
            snapshot(ROOT, copy, area, selected)
            for serde in (False, True):
                features = f"model-{area}" + (",serde" if serde else "")
                args = ["cargo", "+stable", "test", "--locked", "-p", "rust_iso20022",
                        "--no-default-features", "--features", features, "--lib"]
                # Run real builder fixtures once, with their canonical model types.
                if area == "camt" and number == 0 and serde:
                    args += ["--test", "camt_builders", "--test", "model_shard_dispatch"]
                print(">> " + " ".join(args), flush=True)
                subprocess.run(args, cwd=copy, env=env, check=True)
    finally:
        if before != source_digest(ROOT):
            raise RuntimeError("CI sharding modified production sources or manifests")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--matrix", action="store_true")
    parser.add_argument("--area", choices=AREAS)
    parser.add_argument("--shard", type=int)
    args = parser.parse_args()
    if args.matrix:
        plans = {area: shards(ROOT, area) for area in AREAS}
        rows = [{"area": area, "shard": i}
                for i in range(max(map(len, plans.values())))
                for area in AREAS if i < len(plans[area])]
        # Validate all generated dispatch sections before launching matrix jobs.
        source = (ROOT / "src/generated/any.rs").read_text()
        for area in AREAS:
            models = set(inventory(ROOT, area))
            mask_dispatch(source, area, models, models)
        print(json.dumps({"include": rows}, separators=(",", ":")))
    elif args.area is not None and args.shard is not None:
        run_shard(args.area, args.shard)
    else:
        parser.error("use --matrix or both --area and --shard")


if __name__ == "__main__":
    main()
