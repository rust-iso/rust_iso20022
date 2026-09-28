//! Uniform additive API over concrete generated ISO 20022 root values.
//!
//! [`ParsedMessage`] owns exactly one generated root through
//! [`crate::generated::any::AnyMessage`]. It does not copy schema fields into a
//! parallel high-level message model.

use core::fmt;

use crate::metadata::MessageDescriptor;

/// Numeric message version that preserves its original fixed-width spelling.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageVersion {
    text: String,
    numeric: u32,
}

impl MessageVersion {
    fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            numeric: text.parse().unwrap_or(0),
        }
    }

    /// Fixed-width schema version, such as `"08"`.
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// Numeric form used for version ordering.
    pub fn numeric(&self) -> u32 {
        self.numeric
    }
}

impl fmt::Display for MessageVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.text)
    }
}

/// A message family without its version component, e.g. `pacs.008.001`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageFamily(String);

impl MessageFamily {
    pub fn parse(value: &str) -> Option<Self> {
        let parts: Vec<_> = value.split('.').collect();
        let valid = parts.len() == 3
            && parts[0].len() == 4
            && parts[0].bytes().all(|byte| byte.is_ascii_lowercase())
            && parts[1..]
                .iter()
                .all(|part| part.len() == 3 && part.bytes().all(|byte| byte.is_ascii_digit()));
        valid.then(|| Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MessageFamily {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Stable identity of one ISO 20022 message version.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageIdentifier {
    canonical: String,
    business_area: String,
    functionality: String,
    variant: String,
    family: String,
    version: MessageVersion,
}

impl MessageIdentifier {
    /// Construct an identifier from immutable schema-derived metadata.
    pub fn from_descriptor(descriptor: &MessageDescriptor) -> Self {
        let mut parts = descriptor.identity.split('.');
        let business_area = parts.next().unwrap_or_default().to_owned();
        let functionality = parts.next().unwrap_or_default().to_owned();
        let variant = parts.next().unwrap_or_default().to_owned();
        let version = MessageVersion::new(parts.next().unwrap_or_default());
        let family = format!("{business_area}.{functionality}.{variant}");
        Self {
            canonical: descriptor.identity.to_owned(),
            business_area,
            functionality,
            variant,
            family,
            version,
        }
    }

    /// Canonical dotted identifier.
    pub fn as_str(&self) -> &str {
        &self.canonical
    }

    pub fn business_area(&self) -> &str {
        &self.business_area
    }

    pub fn functionality(&self) -> &str {
        &self.functionality
    }

    pub fn variant(&self) -> &str {
        &self.variant
    }

    /// Family without the version component, e.g. `pacs.008.001`.
    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn message_family(&self) -> MessageFamily {
        MessageFamily(self.family.clone())
    }

    pub fn version(&self) -> &MessageVersion {
        &self.version
    }
}

impl fmt::Display for MessageIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.canonical)
    }
}

/// Typed failures while identifying or deserializing a message.
#[derive(Clone, PartialEq, Eq)]
pub enum ParseError {
    /// No schema-derived message identity was found.
    UnknownMessage,
    /// The namespace identifies a message but the element using it has the
    /// wrong local root name.
    RootMismatch {
        message_id: String,
        expected: &'static str,
        found: String,
    },
    /// The message is known, but its per-area generated model feature is not
    /// enabled in this build.
    UnsupportedFeature {
        message_id: String,
        required_feature: &'static str,
    },
    /// The selected concrete generated root rejected the XML.
    Deserialize { message_id: String, reason: String },
}

impl fmt::Debug for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownMessage => formatter.write_str("ParseError::UnknownMessage"),
            Self::RootMismatch {
                message_id,
                expected,
                ..
            } => formatter
                .debug_struct("ParseError::RootMismatch")
                .field("message_id", message_id)
                .field("expected", expected)
                .field("found", &"[REDACTED]")
                .finish(),
            Self::UnsupportedFeature {
                message_id,
                required_feature,
            } => formatter
                .debug_struct("ParseError::UnsupportedFeature")
                .field("message_id", message_id)
                .field("required_feature", required_feature)
                .finish(),
            Self::Deserialize { message_id, .. } => formatter
                .debug_struct("ParseError::Deserialize")
                .field("message_id", message_id)
                .field("reason", &"[REDACTED]")
                .finish(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownMessage => formatter.write_str("unknown ISO 20022 message"),
            Self::RootMismatch {
                message_id,
                expected,
                found,
            } => write!(
                formatter,
                "root mismatch for {message_id}: expected {expected}, found {found}"
            ),
            Self::UnsupportedFeature {
                message_id,
                required_feature,
            } => write!(
                formatter,
                "message {message_id} requires feature {required_feature}"
            ),
            Self::Deserialize { message_id, .. } => {
                write!(
                    formatter,
                    "failed to deserialize generated message {message_id}"
                )
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// Schema-derived identity returned by bounded detection without deserializing
/// a generated message model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedMessage {
    descriptor: &'static MessageDescriptor,
    identifier: MessageIdentifier,
}

impl DetectedMessage {
    pub fn message_id(&self) -> &MessageIdentifier {
        &self.identifier
    }

    pub fn descriptor(&self) -> &'static MessageDescriptor {
        self.descriptor
    }

    pub fn namespace(&self) -> &'static str {
        self.descriptor.namespace
    }

    pub fn root_element(&self) -> &'static str {
        self.descriptor.root_element
    }
}

/// Typed, value-free failure from bounded message detection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectionError {
    Xml(crate::core::XmlReadError),
    UnknownMessage,
    RootMismatch {
        message_id: String,
        expected: &'static str,
    },
}

impl fmt::Display for DetectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Xml(error) => error.fmt(formatter),
            Self::UnknownMessage => formatter.write_str("unknown ISO 20022 message"),
            Self::RootMismatch {
                message_id,
                expected,
            } => write!(
                formatter,
                "root mismatch for {message_id}: expected {expected}"
            ),
        }
    }
}

impl std::error::Error for DetectionError {}

/// Apply bounded XML preflight and return schema-derived identity metadata.
///
/// This API does not compile or deserialize generated model modules and is the
/// single detection entry point used by adapters that need structured errors.
pub fn detect_message(
    xml: &str,
    limits: crate::core::ParseLimits,
) -> Result<DetectedMessage, DetectionError> {
    crate::core::validate_xml(xml, limits).map_err(DetectionError::Xml)?;
    let detected = crate::detect(xml).ok_or(DetectionError::UnknownMessage)?;
    let message_id = detected.message_name();
    let descriptor = crate::metadata::all()
        .iter()
        .find(|descriptor| descriptor.identity == message_id)
        .ok_or(DetectionError::UnknownMessage)?;
    let found_root =
        element_using_namespace(xml, descriptor.namespace).ok_or(DetectionError::UnknownMessage)?;
    if found_root != descriptor.root_element {
        return Err(DetectionError::RootMismatch {
            message_id,
            expected: descriptor.root_element,
        });
    }
    Ok(DetectedMessage {
        descriptor,
        identifier: MessageIdentifier::from_descriptor(descriptor),
    })
}

/// Typed serialization failure for a generated message value.
#[derive(Clone, PartialEq, Eq)]
pub struct SerializeError {
    pub message_id: String,
    pub reason: String,
}

impl fmt::Debug for SerializeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SerializeError")
            .field("message_id", &self.message_id)
            .field("reason", &"[REDACTED]")
            .finish()
    }
}

impl fmt::Display for SerializeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "failed to serialize generated message {}",
            self.message_id
        )
    }
}

impl std::error::Error for SerializeError {}

#[cfg(all(feature = "__message_model", feature = "serde"))]
use crate::generated::any::parse_json_auto;
#[cfg(feature = "__message_model")]
use crate::generated::any::{AnyMessage, parse_auto, supports};

/// An owned concrete generated message plus its schema-derived descriptor.
#[cfg(feature = "__message_model")]
#[derive(Debug)]
pub struct ParsedMessage {
    descriptor: &'static MessageDescriptor,
    identifier: MessageIdentifier,
    message: AnyMessage,
}

#[cfg(feature = "__message_model")]
impl ParsedMessage {
    pub fn descriptor(&self) -> &'static MessageDescriptor {
        self.descriptor
    }

    pub fn message_id(&self) -> &MessageIdentifier {
        &self.identifier
    }

    pub fn business_area(&self) -> &str {
        self.identifier.business_area()
    }

    pub fn family(&self) -> &str {
        self.identifier.family()
    }

    pub fn version(&self) -> &MessageVersion {
        self.identifier.version()
    }

    pub fn namespace(&self) -> &str {
        self.descriptor.namespace
    }

    pub fn root_element(&self) -> &str {
        self.descriptor.root_element
    }

    pub fn description(&self) -> &str {
        self.descriptor.description.unwrap_or("")
    }

    pub fn as_message_ref(&self) -> MessageRef<'_> {
        MessageRef {
            descriptor: self.descriptor,
            identifier: &self.identifier,
            message: &self.message,
        }
    }

    pub fn message(&self) -> &AnyMessage {
        &self.message
    }

    pub fn into_message(self) -> AnyMessage {
        self.message
    }

    pub fn to_xml(&self) -> Result<String, SerializeError> {
        self.message.to_xml().map_err(|error| SerializeError {
            message_id: self.identifier.to_string(),
            reason: error.to_string(),
        })
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Result<String, SerializeError> {
        self.message.to_json().map_err(|error| SerializeError {
            message_id: self.identifier.to_string(),
            reason: error.to_string(),
        })
    }
}

/// Borrowed uniform view that retains direct access to the generated value.
#[cfg(feature = "__message_model")]
#[derive(Debug, Clone, Copy)]
pub struct MessageRef<'a> {
    descriptor: &'static MessageDescriptor,
    identifier: &'a MessageIdentifier,
    message: &'a AnyMessage,
}

#[cfg(feature = "__message_model")]
impl<'a> MessageRef<'a> {
    pub fn descriptor(self) -> &'static MessageDescriptor {
        self.descriptor
    }

    pub fn message_id(self) -> &'a MessageIdentifier {
        self.identifier
    }

    pub fn message(self) -> &'a AnyMessage {
        self.message
    }
}

/// Detect and deserialize XML into its concrete generated root type.
#[cfg(feature = "__message_model")]
pub fn parse(xml: &str) -> Result<ParsedMessage, ParseError> {
    let detected = crate::detect(xml).ok_or(ParseError::UnknownMessage)?;
    let message_id = detected.message_name();
    let descriptor = crate::metadata::all()
        .iter()
        .find(|descriptor| descriptor.identity == message_id)
        .ok_or(ParseError::UnknownMessage)?;
    let found_root =
        element_using_namespace(xml, descriptor.namespace).ok_or(ParseError::UnknownMessage)?;
    if found_root != descriptor.root_element {
        return Err(ParseError::RootMismatch {
            message_id,
            expected: descriptor.root_element,
            found: found_root.to_owned(),
        });
    }
    if !supports(descriptor.identity) {
        return Err(ParseError::UnsupportedFeature {
            message_id,
            required_feature: descriptor.required_feature,
        });
    }
    let owned_root = crate::core::xml_scan::outer_element(xml, descriptor.root_element);
    let generated_xml = owned_root.as_deref().unwrap_or(xml);
    let message = parse_auto(generated_xml).map_err(|error| ParseError::Deserialize {
        message_id: message_id.clone(),
        reason: error.to_string(),
    })?;
    Ok(ParsedMessage {
        descriptor,
        identifier: MessageIdentifier::from_descriptor(descriptor),
        message,
    })
}

/// Deserialize JSON into the exact enabled generated type selected by its
/// canonical schema-derived message identifier.
#[cfg(all(feature = "__message_model", feature = "serde"))]
pub fn parse_json(message_id: &str, json: &str) -> Result<ParsedMessage, ParseError> {
    let descriptor = crate::metadata::all()
        .iter()
        .find(|descriptor| descriptor.identity == message_id)
        .ok_or(ParseError::UnknownMessage)?;
    if !supports(descriptor.identity) {
        return Err(ParseError::UnsupportedFeature {
            message_id: message_id.to_owned(),
            required_feature: descriptor.required_feature,
        });
    }
    let message = parse_json_auto(message_id, json).map_err(|error| ParseError::Deserialize {
        message_id: message_id.to_owned(),
        reason: error.to_string(),
    })?;
    Ok(ParsedMessage {
        descriptor,
        identifier: MessageIdentifier::from_descriptor(descriptor),
        message,
    })
}

fn element_using_namespace<'a>(xml: &'a str, namespace: &str) -> Option<&'a str> {
    let mut offset = 0;
    while let Some(relative) = xml[offset..].find('<') {
        let start = offset + relative;
        let after = xml.get(start + 1..)?;
        if after.starts_with('/') || after.starts_with('!') || after.starts_with('?') {
            offset = start + 1;
            continue;
        }
        let end = xml[start..].find('>')? + start;
        let opening = &xml[start + 1..end];
        if opening.contains(namespace) {
            let name_end = opening
                .find(|character: char| character == '/' || character.is_whitespace())
                .unwrap_or(opening.len());
            return Some(crate::core::xml_scan::local_name(&opening[..name_end]));
        }
        offset = end + 1;
    }
    None
}
