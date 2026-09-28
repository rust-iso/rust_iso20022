#[test]
fn legacy_structured_json_shapes_are_serde_backed_and_stable() {
    let id: serde_json::Value =
        serde_json::from_str(&rust_iso20022::wasm_compat::mx_id("pacs.008.001.08").unwrap())
            .unwrap();
    assert_eq!(id["messageName"], "pacs.008.001.08");
    assert_eq!(id["businessArea"], "pacs");
    assert_eq!(id["version"], "08");

    let entry: serde_json::Value = serde_json::from_str(
        &rust_iso20022::wasm_compat::catalogue_entry("pacs.008.001.08").unwrap(),
    )
    .unwrap();
    assert_eq!(entry["messageName"], "pacs.008.001.08");
    assert_eq!(entry["businessArea"], "pacs");
    assert_eq!(entry["hasModel"], true);
}

#[test]
fn legacy_node_attributes_remain_a_json_object() {
    let node =
        rust_iso20022::MxNode::parse(r#"<Document><Amt Ccy="EUR">10.00</Amt></Document>"#).unwrap();
    let value: serde_json::Value =
        serde_json::from_str(&rust_iso20022::wasm_compat::node(&node)).unwrap();
    assert_eq!(value["children"][0]["attributes"]["Ccy"], "EUR");
    assert_eq!(value["children"][0]["value"], "10.00");
}
