#![no_main]

use libfuzzer_sys::fuzz_target;
use rust_iso20022::{ParseLimits, validate_xml};

fuzz_target!(|data: &[u8]| {
    if let Ok(xml) = std::str::from_utf8(data) {
        let _ = validate_xml(xml, ParseLimits::DEFAULT);
        let _ = rust_iso20022::MxNode::parse(xml);
    }
});
