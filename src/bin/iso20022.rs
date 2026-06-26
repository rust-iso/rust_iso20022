//! `iso20022` — command-line lookup over the ISO 20022 message catalogue.
//!
//! ```text
//! iso20022 pacs.008        # all pacs.008 versions
//! iso20022 pacs            # everything in the pacs business area
//! iso20022 008.001.08      # match on functionality/variant/version
//! iso20022                 # list the whole catalogue
//! ```
//!
//! Requires the `cli` feature:
//! ```bash
//! cargo run --features cli --bin iso20022 -- pacs.008
//! ```

use prettytable::{row, Table};

fn main() {
    let mut args = std::env::args();
    let script = args.next().unwrap_or_default();
    let query = args.next().unwrap_or_default().to_lowercase();

    eprintln!("Usage: {script} [query]   (matches message name, area or namespace)");

    let mut table = Table::new();
    table.add_row(row!["Message", "Area", "Description", "Model", "Namespace"]);

    let mut count = 0usize;
    for e in rust_iso20022::catalogue::all() {
        let area_desc = rust_iso20022::BusinessArea::from_code(e.business_area)
            .map(|a| a.description())
            .unwrap_or("");
        let matches = query.is_empty()
            || e.message_name.to_lowercase().contains(&query)
            || e.business_area.to_lowercase().contains(&query)
            || e.namespace.to_lowercase().contains(&query)
            || area_desc.to_lowercase().contains(&query);
        if matches {
            table.add_row(row![
                e.message_name,
                e.business_area,
                area_desc,
                if e.has_model { "yes" } else { "no" },
                e.namespace,
            ]);
            count += 1;
        }
    }

    table.printstd();
    eprintln!("{count} message(s) matched.");
}
