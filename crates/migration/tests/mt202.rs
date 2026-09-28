use std::collections::BTreeMap;

use rust_iso20022::generated::pacs::pacs_009_001_08 as pacs;
use rust_iso20022_migration::{mt202, report::MappingClassification};

const INPUT: &str = include_str!("../../../fixtures/migration/mt202/input.mt");
const EXPECTED_XML: &str = include_str!("../../../fixtures/migration/mt202/expected.xml");
const EXPECTED_REPORT: &str =
    include_str!("../../../fixtures/migration/mt202/expected-report.json");

#[test]
fn non_trivial_fixture_maps_field_by_field_to_generated_pacs009() {
    let actual = mt202::convert(INPUT).unwrap();
    let expected: pacs::Document = rust_iso20022::from_xml(EXPECTED_XML).unwrap();
    assert_eq!(
        actual.message.fi_cdt_trf.grp_hdr,
        expected.fi_cdt_trf.grp_hdr
    );
    let actual_tx = &actual.message.fi_cdt_trf.cdt_trf_tx_inf[0];
    let expected_tx = &expected.fi_cdt_trf.cdt_trf_tx_inf[0];
    assert_eq!(actual_tx.pmt_id, expected_tx.pmt_id);
    assert_eq!(actual_tx.intr_bk_sttlm_amt, expected_tx.intr_bk_sttlm_amt);
    assert_eq!(actual_tx.intr_bk_sttlm_dt, expected_tx.intr_bk_sttlm_dt);
    assert_eq!(actual_tx.instg_agt, expected_tx.instg_agt);
    assert_eq!(actual_tx.intrmy_agt_1, expected_tx.intrmy_agt_1);
    assert_eq!(actual_tx.dbtr, expected_tx.dbtr);
    assert_eq!(actual_tx.cdtr_agt, expected_tx.cdtr_agt);
    assert_eq!(actual_tx.cdtr, expected_tx.cdtr);
}

#[test]
fn report_fixture_accounts_for_every_source_and_every_non_exact_mapping() {
    let conversion = mt202::convert(INPUT).unwrap();
    let expected: serde_json::Value = serde_json::from_str(EXPECTED_REPORT).unwrap();
    let report = &conversion.mapping_report;
    assert_eq!(
        report.entries().len() as u64,
        expected["source_field_count"].as_u64().unwrap()
    );
    let classifications: BTreeMap<_, _> = report
        .entries()
        .iter()
        .map(|entry| {
            (
                entry.source.tag.as_str(),
                serde_json::to_value(entry.classification).unwrap(),
            )
        })
        .collect();
    for (tag, class) in expected["classifications"].as_object().unwrap() {
        assert_eq!(classifications[tag.as_str()], *class);
    }
    assert_eq!(
        report
            .unmapped_fields()
            .iter()
            .map(|field| field.tag.as_str())
            .collect::<Vec<_>>(),
        ["21", "23E", "32A", "53A", "72"]
    );
    let warning_codes: Vec<_> = report
        .warnings()
        .iter()
        .map(|warning| warning.code.as_str())
        .collect();
    for code in expected["warning_codes"].as_array().unwrap() {
        assert!(warning_codes.contains(&code.as_str().unwrap()));
    }
    assert_eq!(expected["lossless"], false);
    assert!(
        report
            .entries()
            .iter()
            .any(|entry| entry.classification == MappingClassification::Unsupported)
    );
}

#[test]
fn malformed_and_invalid_financial_values_are_typed_value_free_and_panic_free() {
    let cases = [
        ("{4:\n:20:SECRET\n-}", "32A"),
        ("{4:\n:20:SECRET\n:32A:261399USD1,00\n-}", "32A"),
        (
            "{4:\n:20:SECRET\n:32A:260928USD1,00\n:52A:NOT-A-BIC\n-}",
            "52A",
        ),
        (
            "{4:\n:20:SECRET\n:32A:260928USD1,00\n:58A:INVALID\n-}",
            "58A",
        ),
    ];
    for (input, expected_tag) in cases {
        let outcome = std::panic::catch_unwind(|| mt202::convert(input));
        assert!(outcome.is_ok());
        let error = outcome.unwrap().unwrap_err();
        assert!(matches!(
            error,
            mt202::Mt202Error::MissingField { tag } | mt202::Mt202Error::InvalidField { tag }
                if tag == expected_tag
        ));
        assert!(!error.to_string().contains("SECRET"));
    }
}

#[test]
fn converted_generated_message_round_trips_without_claiming_losslessness() {
    let conversion = mt202::convert(INPUT).unwrap();
    let validation =
        rust_iso20022::validation::bindings::validate_pacs_009_001_08(&conversion.message).unwrap();
    assert!(validation.valid());
    let xml = rust_iso20022::to_xml(&conversion.message).unwrap();
    let parsed: pacs::Document = rust_iso20022::from_xml(&xml).unwrap();
    assert_eq!(parsed, conversion.message);
    assert!(!conversion.mapping_report.unmapped_fields().is_empty());
}
