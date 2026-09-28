use serde_json::json;

fn main() {
    let document = json!({
        "schema_version": 1,
        "mcp_protocol": "2026-07-28",
        "tools": rust_iso20022_mcp::Iso20022Mcp::new().listed_tools(),
    });
    println!("{}", serde_json::to_string_pretty(&document).unwrap());
}
