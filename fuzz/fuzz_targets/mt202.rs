#![no_main]

use libfuzzer_sys::fuzz_target;
use rust_iso20022_migration::{mt::MtDocument, mt202};

fuzz_target!(|data: &[u8]| {
    let Ok(input) = std::str::from_utf8(data) else {
        return;
    };
    if let (Ok(document), Ok(converted)) = (MtDocument::parse(input), mt202::convert(input)) {
        assert_complete(&document, converted.mapping_report.entries());
    }
});

fn assert_complete(
    document: &MtDocument,
    entries: &[rust_iso20022_migration::report::MappingEntry],
) {
    assert_eq!(entries.len(), document.fields().len());
    for (index, entry) in entries.iter().enumerate() {
        assert_eq!(entry.source.index, index);
    }
}
