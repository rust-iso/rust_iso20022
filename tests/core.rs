//! Tests for the always-available core API and catalogue (no `model` feature).

use rust_iso20022::{catalogue, from_namespace, BusinessArea, MxId};

#[test]
fn mxid_parses_full_namespace() {
    let id = MxId::parse("urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08").unwrap();
    assert_eq!(id.business_area, BusinessArea::pacs);
    assert_eq!(id.functionality, "008");
    assert_eq!(id.variant, "001");
    assert_eq!(id.version, "08");
    assert_eq!(id.message_name(), "pacs.008.001.08");
    assert_eq!(
        id.namespace(),
        "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08"
    );
}

#[test]
fn mxid_parses_bare_name_and_roundtrips() {
    let id: MxId = "camt.053.001.08".parse().unwrap();
    assert_eq!(id.business_area, BusinessArea::camt);
    assert_eq!(id.to_string(), "camt.053.001.08");
}

#[test]
fn mxid_rejects_garbage() {
    assert!(MxId::parse("not-a-message").is_err());
    assert!(MxId::parse("zzzz.008.001.08").is_err()); // unknown area
    assert!(MxId::parse("pacs.8.1.8").is_err()); // wrong digit widths
    assert!(from_namespace("nonsense").is_none());
}

#[test]
fn business_area_codes_and_lookup() {
    assert_eq!(BusinessArea::pacs.code(), "pacs");
    assert_eq!(
        BusinessArea::pacs.description(),
        "Payments Clearing and Settlement"
    );
    assert_eq!(BusinessArea::from_code("camt"), Some(BusinessArea::camt));
    assert_eq!(BusinessArea::from_code("zzzz"), None);
    assert_eq!(BusinessArea::ALL.len(), 37);
    // every code is unique and round-trips
    for a in BusinessArea::ALL {
        assert_eq!(BusinessArea::from_code(a.code()), Some(a));
    }
}

#[test]
fn catalogue_is_populated_and_consistent() {
    assert!(catalogue::all().len() >= 700, "catalogue too small");
    assert!(catalogue::contains("pacs.008.001.08"));
    // securities/trade families recovered from iso20022.org are present
    assert!(catalogue::contains("sese.001.001.10"));
    assert!(catalogue::contains("semt.002.001.02"));
    assert!(catalogue::contains("tsmt.001.001.03"));

    let e = catalogue::from_message_name("pacs.008.001.08").unwrap();
    assert_eq!(e.business_area, "pacs");
    assert!(e.has_model);
    assert_eq!(catalogue::from_namespace(e.namespace).unwrap(), e);

    // every catalogued message now has a generated model (the former
    // multi-choice gaps are handled by codegen disambiguation).
    let no_model = catalogue::all().iter().filter(|e| !e.has_model).count();
    assert_eq!(no_model, 0);
    assert!(catalogue::from_message_name("seev.030.001.01").unwrap().has_model);

    // every catalogued message name parses as an MxId
    for entry in catalogue::all() {
        assert!(
            MxId::parse(entry.message_name).is_ok(),
            "unparseable: {}",
            entry.message_name
        );
    }
}
