use rust_iso20022::{generated::camt::camt_053_001_09 as camt, validate::Validate};
use rust_iso20022_migration::{mt940, report::MappingClassification};

const INPUT: &str = include_str!("../../../fixtures/migration/mt940/input.mt");
const EXPECTED_XML: &str = include_str!("../../../fixtures/migration/mt940/expected.xml");
const EXPECTED_REPORT: &str =
    include_str!("../../../fixtures/migration/mt940/expected-report.json");

#[test]
fn non_trivial_fixture_maps_repeated_entries_and_balances_to_generated_camt053() {
    let actual = mt940::convert(INPUT).unwrap();
    let expected: camt::Document = rust_iso20022::from_xml(EXPECTED_XML).unwrap();
    assert_eq!(actual.message, expected);
    let statement = &actual.message.bk_to_cstmr_stmt.stmt[0];
    assert_eq!(statement.ntry.len(), 2);
    assert_eq!(statement.bal.len(), 4);
    assert_eq!(statement.ntry[0].addtl_ntry_inf.0, "Invoice 2026-09-A");
    assert_eq!(statement.ntry[1].acct_svcr_ref.0, "BANKREF2");
}

#[test]
fn report_accounts_for_every_occurrence_and_every_non_exact_mapping() {
    let conversion = mt940::convert(INPUT).unwrap();
    let expected: serde_json::Value = serde_json::from_str(EXPECTED_REPORT).unwrap();
    let report = &conversion.mapping_report;
    assert_eq!(
        report.entries().len() as u64,
        expected["source_field_count"]
    );
    let actual_classes: Vec<_> = report
        .entries()
        .iter()
        .map(|entry| serde_json::to_value(entry.classification).unwrap())
        .collect();
    assert_eq!(
        actual_classes,
        *expected["classifications_by_index"].as_array().unwrap()
    );
    let actual_unmapped: Vec<_> = report
        .unmapped_fields()
        .iter()
        .map(|field| field.tag.as_str())
        .collect();
    let expected_unmapped: Vec<_> = expected["unmapped_tags"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect();
    assert_eq!(actual_unmapped, expected_unmapped);
    let warning_codes: Vec<_> = report
        .warnings()
        .iter()
        .map(|warning| warning.code.as_str())
        .collect();
    for code in expected["warning_codes"].as_array().unwrap() {
        assert!(warning_codes.contains(&code.as_str().unwrap()));
    }
    assert!(
        report
            .entries()
            .iter()
            .any(|entry| entry.classification == MappingClassification::Lossy)
    );
    assert!(
        report
            .entries()
            .iter()
            .any(|entry| entry.classification == MappingClassification::Unsupported)
    );
    assert_eq!(expected["lossless"], false);
}

#[test]
fn invalid_financial_values_and_orphan_information_are_typed_and_panic_free() {
    let cases = [
        (
            "{4:\n:20:SECRET\n:25:DE89370400440532013000\n:28C:1\n-}",
            "60F/62F",
        ),
        (
            "{4:\n:20:SECRET\n:25:INVALID@ACCOUNT\n:28C:1\n:60F:C260927EUR1,00\n-}",
            "25",
        ),
        (
            "{4:\n:20:SECRET\n:25:DE89370400440532013000\n:28C:1\n:60F:C261399EUR1,00\n-}",
            "60F",
        ),
        (
            "{4:\n:20:SECRET\n:25:DE89370400440532013000\n:28C:1\n:60F:C260927EUR1,00\n:61:260928D1,999NTRFREF\n-}",
            "61",
        ),
    ];
    for (input, expected_tag) in cases {
        let outcome = std::panic::catch_unwind(|| mt940::convert(input));
        assert!(outcome.is_ok());
        let error = outcome.unwrap().unwrap_err();
        assert!(matches!(
            error,
            mt940::Mt940Error::MissingField { tag } | mt940::Mt940Error::InvalidField { tag }
                if tag == expected_tag
        ));
        assert!(!error.to_string().contains("SECRET"));
    }
    let orphan =
        "{4:\n:20:SECRET\n:25:DE89370400440532013000\n:28C:1\n:60F:C260927EUR1,00\n:86:ORPHAN\n-}";
    assert!(matches!(
        mt940::convert(orphan).unwrap_err(),
        mt940::Mt940Error::OrphanInformation
    ));
}

#[test]
fn generated_message_validates_and_round_trips_without_lossless_claim() {
    let conversion = mt940::convert(INPUT).unwrap();
    conversion.message.validate().unwrap();
    let xml = rust_iso20022::to_xml(&conversion.message).unwrap();
    let parsed: camt::Document = rust_iso20022::from_xml(&xml).unwrap();
    assert_eq!(parsed, conversion.message);
    assert!(rust_iso20022::generated::any::supports("camt.053.001.09"));
    let detected = rust_iso20022::generated::any::parse_auto(&xml).unwrap();
    assert!(detected.as_camt_053_001_09().is_some());
    assert!(!conversion.mapping_report.unmapped_fields().is_empty());
}

#[test]
fn unicode_in_every_fixed_width_balance_position_returns_a_typed_error() {
    for tag in ["60F", "60M", "62F", "62M", "64", "65"] {
        for offset in 0..10 {
            for character in ['é', '\u{33a}', '💶'] {
                let mut balance = "C260927EUR1,00".to_owned();
                balance.replace_range(offset..offset + 1, &character.to_string());
                let input = format!(
                    "{{4:\n:20:UNICODE-CASE\n:25:DE89370400440532013000\n:28C:1\n:60F:C260927EUR1,00\n:{tag}:{balance}\n-}}"
                );
                assert!(
                    matches!(mt940::convert(&input), Err(mt940::Mt940Error::InvalidField { tag: found }) if found == tag),
                    "tag={tag}, offset={offset}, character={character:?}"
                );
            }
        }
    }
}

#[test]
fn unicode_information_text_is_preserved() {
    let input = INPUT.replace("Invoice 2026-09-A", "发票-é");
    let conversion = mt940::convert(&input).unwrap();
    assert_eq!(
        conversion.message.bk_to_cstmr_stmt.stmt[0].ntry[0]
            .addtl_ntry_inf
            .0,
        "发票-é"
    );
}
