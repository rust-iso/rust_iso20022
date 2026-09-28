//! Hand-written core of `rust_iso20022`: message identification, business
//! areas, errors, and (de)serialization helpers. Everything else in the crate
//! (the `generated` and `catalogue` modules) is produced from the ISO 20022
//! XSD schemas.

pub mod app_hdr;
mod bounded_xml;
mod business_area;
mod envelope;
mod error;
mod message;
mod message_api;
pub mod metadata;
mod mx_id;
mod mx_message;
pub(crate) mod xml_scan;

pub use bounded_xml::{
    BoundedXmlReader, ParseLimits, XmlLimit, XmlReadError, XmlStats, validate_xml,
};
pub use business_area::BusinessArea;
pub use envelope::{BusinessMessage, Envelope, parse_envelope, read_business_message};
pub use error::{Error, Result};
#[cfg(feature = "serde")]
pub use message::{from_json, to_json};
pub use message::{from_xml, to_xml, to_xml_fragment};
#[cfg(all(feature = "__message_model", feature = "serde"))]
pub use message_api::parse_json;
pub use message_api::{
    DetectedMessage, DetectionError, MessageFamily, MessageIdentifier, MessageVersion, ParseError,
    SerializeError, detect_message,
};
#[cfg(feature = "__message_model")]
pub use message_api::{MessageRef, ParsedMessage, parse};
pub use mx_id::MxId;
pub use mx_message::{MxMessage, detect};
