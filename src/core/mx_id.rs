//! Identification of an ISO 20022 MX message.

use crate::core::{BusinessArea, Error};

/// Identifies an ISO 20022 message by its four components:
/// business area, message functionality, variant and version.
///
/// For `pacs.008.001.08`: area `pacs`, functionality `008`, variant `001`,
/// version `08`.
///
/// # Examples
/// ```
/// use rust_iso20022::{MxId, BusinessArea};
///
/// let id: MxId = "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08".parse().unwrap();
/// assert_eq!(id.business_area, BusinessArea::pacs);
/// assert_eq!(id.functionality, "008");
/// assert_eq!(id.variant, "001");
/// assert_eq!(id.version, "08");
/// assert_eq!(id.message_name(), "pacs.008.001.08");
/// assert_eq!(id.namespace(), "urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MxId {
    /// The business area, e.g. `pacs`.
    pub business_area: BusinessArea,
    /// The message functionality (message type), e.g. `"008"`.
    pub functionality: String,
    /// The variant, e.g. `"001"`.
    pub variant: String,
    /// The version, e.g. `"08"`.
    pub version: String,
}

impl MxId {
    /// Build from the four components.
    pub fn new(
        business_area: BusinessArea,
        functionality: impl Into<String>,
        variant: impl Into<String>,
        version: impl Into<String>,
    ) -> MxId {
        MxId {
            business_area,
            functionality: functionality.into(),
            variant: variant.into(),
            version: version.into(),
        }
    }

    /// The canonical message name, e.g. `"pacs.008.001.08"`.
    pub fn message_name(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.business_area.code(),
            self.functionality,
            self.variant,
            self.version
        )
    }

    /// The ISO 20022 XSD namespace, e.g.
    /// `"urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08"`.
    pub fn namespace(&self) -> String {
        format!("urn:iso:std:iso:20022:tech:xsd:{}", self.message_name())
    }

    /// Parse from a full namespace, a partial namespace, or a bare message name.
    ///
    /// Accepts anything containing an `aaaa.999.999.99` token, so both
    /// `"urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08"` and `"pacs.008.001.08"`
    /// work.
    pub fn parse(s: &str) -> Result<MxId, Error> {
        // Find the message-name token. We scan for the rightmost occurrence of
        // a `aaaa.ddd.ddd.dd` shaped substring to tolerate namespace prefixes.
        let candidate = s.rsplit(':').next().unwrap_or(s).trim();
        let parts: Vec<&str> = candidate.split('.').collect();
        if parts.len() != 4 {
            return Err(Error::InvalidMxId(s.to_string()));
        }
        let area = BusinessArea::from_code(parts[0])
            .ok_or_else(|| Error::UnknownBusinessArea(parts[0].to_string()))?;
        let valid = parts[0].len() == 4
            && parts[1].len() == 3
            && parts[2].len() == 3
            && parts[3].len() == 2
            && parts[1..].iter().all(|p| p.bytes().all(|b| b.is_ascii_digit()));
        if !valid {
            return Err(Error::InvalidMxId(s.to_string()));
        }
        Ok(MxId::new(area, parts[1], parts[2], parts[3]))
    }
}

impl core::fmt::Display for MxId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.message_name())
    }
}

impl core::str::FromStr for MxId {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MxId::parse(s)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for MxId {
    /// Serializes to the canonical message name, e.g. `"pacs.008.001.08"`.
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.message_name())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MxId {
    /// Deserializes from a message name or namespace via [`MxId::parse`].
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        MxId::parse(&s).map_err(serde::de::Error::custom)
    }
}
