use rust_iso20022::privacy::{RedactionPolicy, SensitiveKind};
use rust_iso20022::{Error, ParseError, ParseLimits, validate_xml};

const XML: &str = "<Document><IBAN>DE89370400440532013000</IBAN><Nm>Jane Secret</Nm><Adr>1 Private Street</Adr><Ustrd>Invoice SECRET-REF-42</Ustrd></Document>";

fn assert_canaries_absent(rendered: &str) {
    for canary in [
        XML,
        "DE89370400440532013000",
        "Jane Secret",
        "1 Private Street",
        "SECRET-REF-42",
    ] {
        assert!(!rendered.contains(canary), "sensitive canary leaked");
    }
}

#[test]
fn default_policy_masks_identifiers_and_suppresses_free_text() {
    let policy = RedactionPolicy::default();
    assert_eq!(
        policy.redact(SensitiveKind::Iban, "DE89370400440532013000"),
        "DE89************013000"
    );
    assert_eq!(
        policy.redact(SensitiveKind::Account, "1234567890123456"),
        "1234******123456"
    );
    assert_eq!(
        policy.redact(SensitiveKind::Name, "Jane Secret"),
        "[REDACTED]"
    );
    assert_eq!(
        policy.redact(SensitiveKind::Remittance, "Invoice SECRET-REF-42"),
        "[REDACTED]"
    );
}

#[test]
fn error_display_and_debug_never_echo_source_xml_or_values() {
    let errors = [
        format!(
            "{} {:?}",
            Error::Deserialize(XML.into()),
            Error::Deserialize(XML.into())
        ),
        format!(
            "{} {:?}",
            ParseError::Deserialize {
                message_id: "pacs.008.001.08".into(),
                reason: XML.into(),
            },
            ParseError::Deserialize {
                message_id: "pacs.008.001.08".into(),
                reason: XML.into(),
            }
        ),
    ];
    for rendered in errors {
        assert_canaries_absent(&rendered);
    }
}

#[test]
fn bounded_parser_failure_is_safe_for_display_and_debug() {
    let error =
        validate_xml(XML, ParseLimits::default().with_max_input_bytes(1)).expect_err("oversize");
    assert_canaries_absent(&format!("{error} {error:?}"));
}

#[test]
fn unsafe_full_logging_is_not_available_in_default_build() {
    #[cfg(not(feature = "unsafe-full-logging"))]
    const {
        assert!(!cfg!(feature = "unsafe-full-logging"))
    };
    assert!(!RedactionPolicy::default().allows_full_values());
}

#[cfg(feature = "unsafe-full-logging")]
#[test]
fn unsafe_full_logging_requires_runtime_acknowledgement_and_exposes_warning() {
    use rust_iso20022::privacy::{UNSAFE_FULL_LOGGING_WARNING, UnsafeLoggingPermit};

    // SAFETY: the permit is confined to this test and no value is logged.
    let permit = unsafe { UnsafeLoggingPermit::acknowledge_risk() };
    assert_eq!(permit.warning(), UNSAFE_FULL_LOGGING_WARNING);
    assert!(UNSAFE_FULL_LOGGING_WARNING.contains("financial values"));
    let policy = RedactionPolicy::default().with_unsafe_full_logging(permit);
    assert!(policy.allows_full_values());
}
