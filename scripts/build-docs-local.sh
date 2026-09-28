#!/usr/bin/env bash
# Build the representative documentation feature set configured for docs.rs.
# The generated target/doc HTML is disposable and is not committed.
set -euo pipefail

cd "$(dirname "$0")/.."

target_dir="${CARGO_TARGET_DIR:-target/docs-local}"
jobs="${DOCS_JOBS:-1}"

RUSTDOCFLAGS="${RUSTDOCFLAGS:---cfg docsrs -D rustdoc::broken_intra_doc_links}" \
  CARGO_TARGET_DIR="$target_dir" \
  cargo +stable doc --locked --no-deps \
    --features model-head,model-pacs,model-pain,serde,convert \
    --jobs "$jobs"

echo "local documentation written to $target_dir/doc/rust_iso20022/index.html"
