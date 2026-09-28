#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(xml) = std::str::from_utf8(data) {
        let limits = rust_iso20022::ParseLimits::DEFAULT;
        let structured = rust_iso20022::detect_message(xml, limits);
        let compatibility = rust_iso20022::detect(xml);
        if let Ok(detected) = structured {
            let compatible = compatibility.expect("structured detection implies compatibility");
            assert_eq!(detected.message_id().as_str(), compatible.message_name());
        }
    }
});
