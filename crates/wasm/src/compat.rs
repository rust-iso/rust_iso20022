//! Native-testable access to the serde-backed legacy WASM presentation shims.

pub use rust_iso20022::wasm_compat::{
    business_message, catalogue_entry, header, metadata, mx_id, node,
};
