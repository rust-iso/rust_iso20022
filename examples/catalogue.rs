//! Look up exact, schema-derived message metadata without enabling a model.

use rust_iso20022::catalogue;

fn main() {
    let message = catalogue::lookup_descriptor("pacs.008.001.08")
        .expect("pacs.008.001.08 is present in the generated catalogue");
    assert_eq!(message.required_feature, "model-pacs");

    let versions = catalogue::versions("pacs.008.001");
    let latest = catalogue::latest("pacs.008.001").expect("at least one version");
    println!("{} versions; latest is {}", versions.len(), latest.identity);
}
