use rust_iso20022::profiles::{
    ProfileDate, ProfileDispatchError, ProfileRegistry, ProfileRelease, ProfileReleaseKey,
    ProfileScheme, ReleaseKeyParseError,
};
use rust_iso20022::validation::{
    FieldPath, Rule, RuleDescriptor, RuleId, RuleSet, Severity, ValidationContext, ValidationIssue,
    ValidationLayer, ValidationTarget,
};

const DIGEST_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const DIGEST_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

struct TestRule;

static TEST_RULE: TestRule = TestRule;
static TEST_DESCRIPTOR: RuleDescriptor = RuleDescriptor {
    id: RuleId::new("TESTNETWORK-2025-R0001"),
    layer: ValidationLayer::Profile,
    title: "Synthetic framework dispatch rule",
    reason: "Proves exact-release L3 dispatch without shipping a market rule.",
    source: Some("tests/profiles/fixtures/source-manifest.json#TEST-SECTION"),
    profile: Some("test-network"),
    profile_version: Some("2025.1/ig-2025.1"),
    affected_messages: &["pacs.008.001.08"],
};

impl Rule for TestRule {
    fn descriptor(&self) -> &'static RuleDescriptor {
        &TEST_DESCRIPTOR
    }

    fn field_paths(&self) -> Vec<&'static str> {
        vec!["Document.test"]
    }

    fn evaluate(
        &self,
        target: &ValidationTarget<'_>,
        _context: &ValidationContext<'_>,
    ) -> Vec<ValidationIssue> {
        (!target.present("Document.test"))
            .then(|| {
                ValidationIssue::new(
                    &TEST_DESCRIPTOR,
                    Severity::Error,
                    FieldPath::new("Document.test"),
                    "required synthetic field is absent",
                )
            })
            .into_iter()
            .collect()
    }
}

fn key_2025() -> ProfileReleaseKey {
    ProfileReleaseKey::parse(&format!(
        "test-network|2025.1|ig-2025.1|2025-01-01|2025-12-31|{DIGEST_A}"
    ))
    .unwrap()
}

fn key_2026() -> ProfileReleaseKey {
    ProfileReleaseKey::parse(&format!(
        "test-network|2026.1|ig-2026.1|2026-01-01|2026-12-31|{DIGEST_B}"
    ))
    .unwrap()
}

#[test]
fn composite_release_keys_parse_round_trip_and_order() {
    let old = key_2025();
    let new = key_2026();
    assert_eq!(
        old.to_string(),
        format!("test-network|2025.1|ig-2025.1|2025-01-01|2025-12-31|{DIGEST_A}")
    );
    assert!(old < new);
    assert_eq!(old.scheme().as_str(), "test-network");
    assert_eq!(old.rule_set_version(), "2025.1/ig-2025.1");
    assert_eq!(old.source_digests(), &[DIGEST_A]);

    assert_eq!(
        ProfileReleaseKey::parse("cbpr-plus:2026"),
        Err(ReleaseKeyParseError::Incomplete)
    );
    assert!(ProfileReleaseKey::parse(
        "test-network|2025|ig|2025-02-29|2025-12-31|aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    )
    .is_err());
}

#[test]
fn as_of_resolution_keeps_historical_releases_addressable() {
    let old = ProfileRelease::unavailable(
        key_2025(),
        &["pacs.008.001.08"],
        "framework-only historical fixture",
        "fixture-index-v1",
        "evidence-v1",
    )
    .unwrap();
    let new = ProfileRelease::unavailable(
        key_2026(),
        &["pacs.008.001.08", "pacs.009.001.08"],
        "framework-only current fixture",
        "fixture-index-v2",
        "evidence-v2",
    )
    .unwrap();
    let registry = ProfileRegistry::new(vec![new, old]).unwrap();
    let scheme = ProfileScheme::parse("test-network").unwrap();

    assert_eq!(
        registry
            .resolve_as_of(&scheme, ProfileDate::parse("2025-06-01").unwrap())
            .unwrap()
            .key(),
        &key_2025()
    );
    assert_eq!(
        registry
            .resolve_as_of(&scheme, ProfileDate::parse("2026-06-01").unwrap())
            .unwrap()
            .key(),
        &key_2026()
    );
    assert_eq!(
        registry.exact(&key_2025()).unwrap().evidence_ref(),
        "evidence-v1"
    );
    assert_eq!(registry.releases()[0].key(), &key_2025());
}

#[test]
fn support_and_unavailable_dispatch_are_explicit() {
    let release = ProfileRelease::unavailable(
        key_2025(),
        &["pacs.008.001.08"],
        "authoritative rule pack not installed",
        "fixture-index-v1",
        "evidence-v1",
    )
    .unwrap();
    let registry = ProfileRegistry::new(vec![release]).unwrap();

    assert!(registry.supports(&key_2025(), "pacs.008.001.08").is_ok());
    assert!(matches!(
        registry.supports(&key_2025(), "pacs.008.001.09"),
        Err(ProfileDispatchError::UnsupportedMessage { .. })
    ));
    assert!(matches!(
        registry.rule_set_for(&key_2025(), "pacs.008.001.08"),
        Err(ProfileDispatchError::RulesUnavailable { .. })
    ));
}

#[test]
fn exact_release_dispatches_only_its_l3_rule_set() {
    let rules = RuleSet::new(
        "test-network",
        Some("2025.1/ig-2025.1"),
        ValidationLayer::Profile,
        &[&TEST_RULE],
    )
    .unwrap();
    let release = ProfileRelease::available(
        key_2025(),
        &["pacs.008.001.08"],
        rules,
        "fixture-index-v1",
        "evidence-v1",
    )
    .unwrap();
    let registry = ProfileRegistry::new(vec![release]).unwrap();
    let rule_set = registry
        .rule_set_for(&key_2025(), "pacs.008.001.08")
        .unwrap();
    assert_eq!(rule_set.name, "test-network");
    assert_eq!(rule_set.version, Some("2025.1/ig-2025.1"));
}
