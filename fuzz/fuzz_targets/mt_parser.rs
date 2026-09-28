#![no_main]

use libfuzzer_sys::fuzz_target;
use rust_iso20022_migration::mt::MtDocument;

fuzz_target!(|data: &[u8]| {
    let Ok(input) = std::str::from_utf8(data) else {
        return;
    };
    if let Ok(document) = MtDocument::parse(input) {
        for (index, field) in document.fields().iter().enumerate() {
            assert_eq!(field.index(), index);
            assert!(field.span().start <= field.span().end);
            assert!(field.span().end <= input.len());
        }
        for block in document.blocks() {
            assert!(block.span().start <= block.span().end);
            assert!(block.span().end <= input.len());
        }
    }
});
