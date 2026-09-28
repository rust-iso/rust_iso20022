//! Compatibility shim for the historical root-package catalogue binary.
//!
//! `detect` is routed through the exact standalone CLI runner. Every other
//! invocation retains the legacy catalogue-query behavior.

use prettytable::{Table, row};

#[path = "../../crates/cli/src/commands/mod.rs"]
mod commands;
#[path = "../../crates/cli/src/output.rs"]
mod output;
#[path = "../../crates/cli/src/runner.rs"]
mod runner;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("detect") {
        let code = runner::run(args.into_iter());
        if code != output::ExitCode::Success {
            std::process::exit(code as i32);
        }
        return;
    }
    legacy_catalogue(args.first().map_or("", String::as_str));
}

fn legacy_catalogue(query: &str) {
    eprintln!("Usage: iso20022 [query]   (matches message name, area or namespace)");
    let query = query.to_lowercase();
    let mut table = Table::new();
    table.add_row(row!["Message", "Area", "Description", "Model", "Namespace"]);
    let mut count = 0usize;
    for entry in rust_iso20022::catalogue::all() {
        let area_description = rust_iso20022::BusinessArea::from_code(entry.business_area)
            .map(|area| area.description())
            .unwrap_or("");
        let matches = query.is_empty()
            || entry.message_name.to_lowercase().contains(&query)
            || entry.business_area.to_lowercase().contains(&query)
            || entry.namespace.to_lowercase().contains(&query)
            || area_description.to_lowercase().contains(&query);
        if matches {
            table.add_row(row![
                entry.message_name,
                entry.business_area,
                area_description,
                if entry.has_model { "yes" } else { "no" },
                entry.namespace,
            ]);
            count += 1;
        }
    }
    table.printstd();
    eprintln!("{count} message(s) matched.");
}
