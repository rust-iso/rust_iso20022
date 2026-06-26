//! Hand-written core of `rust_iso20022`: message identification, business
//! areas, errors, and (de)serialization helpers. Everything else in the crate
//! (the `generated` and `catalogue` modules) is produced from the ISO 20022
//! XSD schemas.

pub mod app_hdr;
pub mod metadata;
mod business_area;
mod error;
mod message;
mod mx_id;
mod mx_message;
pub(crate) mod xml_scan;

pub use business_area::BusinessArea;
pub use error::{Error, Result};
pub use message::{from_xml, to_xml, to_xml_fragment};
#[cfg(feature = "serde")]
pub use message::{from_json, to_json};
pub use mx_id::MxId;
pub use mx_message::{detect, MxMessage};
