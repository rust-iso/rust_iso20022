#!/usr/bin/env bash
#
# Capture and compare the additive public API against crates.io 0.1.1.
#
# Usage:
#   scripts/check-public-api.sh [output-directory]
set -euo pipefail

cd "$(dirname "$0")/.."

api_output_dir="${1:-evidence/compatibility}"
mkdir -p "$api_output_dir"

for tool in cargo-public-api cargo-semver-checks node; do
    command -v "$tool" >/dev/null 2>&1 || {
        echo "error: required tool not found: $tool" >&2
        exit 1
    }
done

{
    cargo +nightly public-api --version
    cargo +stable semver-checks --version
    rustup run nightly rustc --version
    rustc +stable --version
    cargo +stable --version
} >"$api_output_dir/tool-versions.txt"

cargo +stable metadata --locked --format-version 1 --no-deps |
    node -e '
        let input = "";
        process.stdin.setEncoding("utf8");
        process.stdin.on("data", chunk => input += chunk);
        process.stdin.on("end", () => {
            const metadata = JSON.parse(input);
            const pkg = metadata.packages.find(p => p.name === "rust_iso20022");
            const result = {
                format_version: 1,
                package_version: pkg.version,
                edition: pkg.edition,
                rust_version: pkg.rust_version,
                features: Object.fromEntries(
                    Object.entries(pkg.features).sort(([a], [b]) => a.localeCompare(b))
                )
            };
            process.stdout.write(JSON.stringify(result, null, 2) + "\n");
        });
    ' >"$api_output_dir/current-features.json"

cargo +stable info rust_iso20022@0.1.1 --verbose \
    >"$api_output_dir/published-0.1.1-info.txt"

cargo +nightly public-api \
    --no-default-features \
    -sss \
    --color never \
    >"$api_output_dir/current-core.txt"

cargo +nightly public-api \
    --no-default-features \
    --features model-pacs,serde,convert \
    -sss \
    --color never \
    >"$api_output_dir/current-pacs-serde-convert.txt"

cargo +nightly public-api \
    --no-default-features \
    -sss \
    --color never \
    diff --deny removed --deny changed 0.1.1 \
    >"$api_output_dir/public-api-diff-0.1.1.txt"

# cargo-semver-checks 0.50 itself requires Rust >=1.93. Run this audit tool on
# stable while the product and code generator remain independently tested on
# the declared Rust 1.85 MSRV.
cargo +stable semver-checks check-release \
    --manifest-path Cargo.toml \
    --baseline-version 0.1.1 \
    --default-features \
    --color never \
    >"$api_output_dir/semver-checks-0.1.1.txt" 2>&1

echo ">> Public API and SemVer baseline passed"
