#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(value) = std::str::from_utf8(data) {
        let parsed = rust_iso20022::MxId::parse(value);
        let compatibility = rust_iso20022::from_namespace(value);
        match parsed {
            Ok(id) => {
                assert_eq!(compatibility.as_ref(), Some(&id));
                assert_eq!(rust_iso20022::MxId::parse(&id.message_name()), Ok(id));
            }
            Err(_) => assert!(compatibility.is_none()),
        }
    }
});
