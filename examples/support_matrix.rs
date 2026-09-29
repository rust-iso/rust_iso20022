//! Print the schema-derived business-area support matrix.
//!
//! ```bash
//! cargo run --example support_matrix
//! ```

use rust_iso20022::{BusinessArea, catalogue};

fn main() {
    println!("# Generated model support matrix\n");
    println!("| Area | Feature | Description | Message versions |");
    println!("|---|---|---|---:|");
    for area in BusinessArea::ALL {
        let count = catalogue::all()
            .iter()
            .filter(|entry| entry.business_area == area.code())
            .count();
        if count == 0 {
            continue;
        }
        println!(
            "| `{}` | `model-{}` | {} | {} |",
            area.code(),
            area.code(),
            area.description(),
            count
        );
    }
}
