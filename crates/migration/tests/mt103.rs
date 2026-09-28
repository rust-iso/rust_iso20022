use std::collections::BTreeMap;

use rust_iso20022::generated::pacs::pacs_008_001_08 as pacs;
use rust_iso20022_migration::{mt103, report::MappingClassification};

const INPUT: &str = include_str!("../../../fixtures/migration/mt103/input.mt");
const EXPECTED_XML: &str = include_str!("../../../fixtures/migration/mt103/expected.xml");
const EXPECTED_REPORT: &str =
    include_str!("../../../fixtures/migration/mt103/expected-report.json");

#[test]
fn non_trivial_fixture_maps_field_by_field_to_generated_pacs008() {
    let actual = mt103::convert(INPUT).unwrap();
    let expected: pacs::Document = rust_iso20022::from_xml(EXPECTED_XML).unwrap();
    let actual_message = &actual.message.fi_to_fi_cstmr_cdt_trf;
    let expected_message = &expected.fi_to_fi_cstmr_cdt_trf;
    assert_eq!(
        actual_message.grp_hdr.msg_id,
        expected_message.grp_hdr.msg_id
    );
    assert_eq!(
        actual_message.grp_hdr.nb_of_txs,
        expected_message.grp_hdr.nb_of_txs
    );
    let actual_tx = &actual_message.cdt_trf_tx_inf[0];
    let expected_tx = &expected_message.cdt_trf_tx_inf[0];
    assert_eq!(actual_tx.pmt_id, expected_tx.pmt_id);
    assert_eq!(actual_tx.intr_bk_sttlm_amt, expected_tx.intr_bk_sttlm_amt);
    assert_eq!(actual_tx.intr_bk_sttlm_dt, expected_tx.intr_bk_sttlm_dt);
    assert_eq!(actual_tx.dbtr.nm, expected_tx.dbtr.nm);
    assert_eq!(actual_tx.dbtr_acct.id.iban, expected_tx.dbtr_acct.id.iban);
    assert_eq!(
        actual_tx.dbtr_agt.fin_instn_id.bicfi,
        expected_tx.dbtr_agt.fin_instn_id.bicfi
    );
    assert_eq!(
        actual_tx.cdtr_agt.fin_instn_id.bicfi,
        expected_tx.cdtr_agt.fin_instn_id.bicfi
    );
    assert_eq!(actual_tx.cdtr.nm, expected_tx.cdtr.nm);
    assert_eq!(actual_tx.cdtr_acct.id.iban, expected_tx.cdtr_acct.id.iban);
    assert_eq!(actual_tx.chrg_br, expected_tx.chrg_br);
    assert_eq!(actual_tx.rmt_inf.ustrd, expected_tx.rmt_inf.ustrd);
}

#[test]
fn report_matches_fixture_and_every_source_field_is_visible() {
    let conversion = mt103::convert(INPUT).unwrap();
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
        ["23B", "32A", "72"]
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
            .any(|entry| entry.classification == MappingClassification::Lossy)
    );
}

#[test]
fn unsupported_ambiguous_lossy_and_malformed_inputs_never_disappear_or_panic() {
    let conversion = mt103::convert(INPUT).unwrap();
    assert_eq!(conversion.mapping_report.entries().len(), 10);
    assert_eq!(conversion.mapping_report.unmapped_fields().len(), 3);
    for bad in [
        "{4:\n:20:X\n-}",
        "{4:\n:20:X\n:32A:éééééééééé\n-}",
        "{4:\n:20:SECRET\n:32A:260928EURX\n-}",
    ] {
        let outcome = std::panic::catch_unwind(|| mt103::convert(bad));
        assert!(outcome.is_ok());
        let error = outcome.unwrap().unwrap_err();
        assert!(!error.to_string().contains("SECRET"));
    }
}

#[test]
fn invalid_financial_values_return_typed_value_free_errors() {
    let cases = [
        ("{4:\n:20:SECRET\n:32A:261399EUR1,00\n-}", "32A"),
        (
            "{4:\n:20:SECRET\n:32A:260928EUR1,00\n:50K:/DE001234\nNAME\n-}",
            "50K",
        ),
        (
            "{4:\n:20:SECRET\n:32A:260928EUR1,00\n:52A:NOT-A-BIC\n-}",
            "52A",
        ),
        (
            "{4:\n:20:SECRET\n:32A:260928EUR1,00\n:71A:INVALID\n-}",
            "71A",
        ),
    ];
    for (input, expected_tag) in cases {
        let outcome = std::panic::catch_unwind(|| mt103::convert(input));
        assert!(outcome.is_ok());
        let error = outcome.unwrap().unwrap_err();
        assert!(matches!(
            error,
            mt103::Mt103Error::InvalidField { tag } if tag == expected_tag
        ));
        assert!(!error.to_string().contains("SECRET"));
    }
}

#[test]
fn converted_message_round_trips_and_passes_claimed_l2_rules() {
    let conversion = mt103::convert(INPUT).unwrap();
    let report =
        rust_iso20022::validation::bindings::validate_pacs_008_001_08(&conversion.message).unwrap();
    assert!(report.valid());
    let xml = rust_iso20022::to_xml(&conversion.message).unwrap();
    let parsed: pacs::Document = rust_iso20022::from_xml(&xml).unwrap();
    assert_eq!(parsed, conversion.message);
}
