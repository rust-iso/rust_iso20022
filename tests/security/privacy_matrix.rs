use rust_iso20022::privacy::{RedactionPolicy, SensitiveKind};
use rust_iso20022::validation::{
    RuleRegistry, ValidationContext, ValidationTarget, financial_rules,
};
use rust_iso20022::{Error, ParseError};

const IBAN: &str = "DE89370400440532013000";
const ACCOUNT: &str = "SECRET-ACCOUNT-998877";
const NAME: &str = "Jane Canary Person";
const ADDRESS: &str = "77 Canary Private Street";
const REFERENCE: &str = "SECRET-REFERENCE-2026";
const REMITTANCE: &str = "Invoice CANARY-4242";
const XML: &str = "<Document><IBAN>DE89370400440532013000</IBAN></Document>";

fn assert_no_canary(rendered: &str) {
    for canary in [IBAN, ACCOUNT, NAME, ADDRESS, REFERENCE, REMITTANCE, XML] {
        assert!(!rendered.contains(canary), "sensitive canary leaked");
    }
}

#[test]
fn core_errors_rules_and_default_redaction_do_not_disclose_canaries() {
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
        assert_no_canary(&rendered);
    }

    let registry = RuleRegistry::new(financial_rules()).unwrap();
    let fields = [
        ("$financial.iban", IBAN),
        ("$financial.account", "INVALID@ACCOUNT"),
        ("$financial.bic", NAME),
        ("$financial.amount_currency", "EUR"),
        ("$financial.amount", REFERENCE),
    ];
    let report = registry.validate(
        &ValidationTarget::from_pairs(&fields),
        &ValidationContext::default(),
    );
    assert_no_canary(&format!("{report:?}"));

    let policy = RedactionPolicy::default();
    let rendered = [
        policy.redact(SensitiveKind::Iban, IBAN),
        policy.redact(SensitiveKind::Account, ACCOUNT),
        policy.redact(SensitiveKind::Name, NAME),
        policy.redact(SensitiveKind::Address, ADDRESS),
        policy.redact(SensitiveKind::TransactionReference, REFERENCE),
        policy.redact(SensitiveKind::Remittance, REMITTANCE),
        policy.redact(SensitiveKind::MessageXml, XML),
    ]
    .join(" ");
    assert_no_canary(&rendered);
}

#[test]
fn adapters_install_the_redacted_panic_hook_and_have_no_unsafe_logging_default() {
    let wasm_sources = concat!(
        include_str!("../../crates/wasm/src/lib.rs"),
        include_str!("../../src/wasm.rs"),
    );
    assert!(!wasm_sources.contains("console_error_panic_hook::set_once"));
    assert_eq!(
        wasm_sources.matches("install_redacted_panic_hook").count(),
        2
    );

    for manifest in [
        include_str!("../../crates/cli/Cargo.toml"),
        include_str!("../../crates/mcp/Cargo.toml"),
        include_str!("../../crates/wasm/Cargo.toml"),
    ] {
        assert!(!manifest.contains("unsafe-full-logging"));
    }
    assert!(!RedactionPolicy::default().allows_full_values());
    assert!(rust_iso20022::privacy::UNSAFE_FULL_LOGGING_WARNING.contains("financial values"));
}
