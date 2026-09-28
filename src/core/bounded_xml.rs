//! Finite-by-default XML preflight shared by every public XML entry point.
//!
//! This is a defensive well-formedness and resource-bound gate, not full XSD
//! validation. It performs no I/O, resolves no external resource, rejects DTD,
//! and XInclude, treats schema-location hints as inert data, and never includes
//! source XML in errors.

use core::fmt;

use xml::reader::{ParserConfig, XmlEvent};

/// Resource dimensions enforced while reading XML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum XmlLimit {
    InputBytes,
    Depth,
    Events,
    Elements,
    Attributes,
    TextBytes,
    DecodedBytes,
    CollectionItems,
}

impl fmt::Display for XmlLimit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InputBytes => "input bytes",
            Self::Depth => "nesting depth",
            Self::Events => "XML events",
            Self::Elements => "elements",
            Self::Attributes => "attributes",
            Self::TextBytes => "text event bytes",
            Self::DecodedBytes => "decoded text bytes",
            Self::CollectionItems => "child collection items",
        })
    }
}

/// Typed XML preflight failure. It deliberately carries no source fragment or
/// parser-provided message that might disclose financial data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XmlReadError {
    LimitExceeded { limit: XmlLimit, maximum: usize },
    Forbidden { construct: &'static str },
    Malformed { kind: &'static str },
}

impl fmt::Display for XmlReadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LimitExceeded { limit, maximum } => {
                write!(formatter, "XML {limit} limit exceeded (maximum {maximum})")
            }
            Self::Forbidden { construct } => {
                write!(formatter, "forbidden XML construct: {construct}")
            }
            Self::Malformed { kind } => write!(formatter, "malformed XML: {kind}"),
        }
    }
}

impl std::error::Error for XmlReadError {}

/// Finite resource policy for untrusted XML.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseLimits {
    pub max_input_bytes: usize,
    pub max_depth: usize,
    pub max_events: usize,
    pub max_elements: usize,
    pub max_attributes: usize,
    pub max_text_bytes: usize,
    pub max_decoded_bytes: usize,
    pub max_collection_items: usize,
}

impl ParseLimits {
    /// Safe library defaults. Adapters may lower these limits for their own
    /// trust boundary; raising them must be explicit.
    pub const DEFAULT: Self = Self {
        max_input_bytes: 8 * 1024 * 1024,
        max_depth: 64,
        max_events: 1_000_000,
        max_elements: 100_000,
        max_attributes: 500_000,
        max_text_bytes: 1024 * 1024,
        max_decoded_bytes: 8 * 1024 * 1024,
        max_collection_items: 100_000,
    };

    pub const fn with_max_input_bytes(mut self, maximum: usize) -> Self {
        self.max_input_bytes = maximum;
        self
    }

    pub const fn with_max_depth(mut self, maximum: usize) -> Self {
        self.max_depth = maximum;
        self
    }

    pub const fn with_max_events(mut self, maximum: usize) -> Self {
        self.max_events = maximum;
        self
    }

    pub const fn with_max_elements(mut self, maximum: usize) -> Self {
        self.max_elements = maximum;
        self
    }

    pub const fn with_max_attributes(mut self, maximum: usize) -> Self {
        self.max_attributes = maximum;
        self
    }

    pub const fn with_max_text_bytes(mut self, maximum: usize) -> Self {
        self.max_text_bytes = maximum;
        self
    }

    pub const fn with_max_decoded_bytes(mut self, maximum: usize) -> Self {
        self.max_decoded_bytes = maximum;
        self
    }

    pub const fn with_max_collection_items(mut self, maximum: usize) -> Self {
        self.max_collection_items = maximum;
        self
    }
}

impl Default for ParseLimits {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Non-sensitive counters returned by a successful preflight.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct XmlStats {
    pub input_bytes: usize,
    pub events: usize,
    pub elements: usize,
    pub attributes: usize,
    pub decoded_bytes: usize,
    pub max_depth: usize,
    /// Largest number of direct child elements observed under one parent.
    pub max_collection_items: usize,
}

/// Streaming preflight state for one XML input.
#[derive(Debug, Clone, Copy)]
pub struct BoundedXmlReader {
    limits: ParseLimits,
}

impl BoundedXmlReader {
    pub const fn new(limits: ParseLimits) -> Self {
        Self { limits }
    }

    pub fn validate(self, xml: &str) -> Result<XmlStats, XmlReadError> {
        check(self.limits, XmlLimit::InputBytes, xml.len())?;
        if xml.as_bytes().contains(&0) {
            return Err(XmlReadError::Malformed {
                kind: "forbidden Unicode scalar value",
            });
        }
        if xml.contains("<!DOCTYPE") || xml.contains("<!ENTITY") {
            return Err(XmlReadError::Forbidden {
                construct: "DTD or entity declaration",
            });
        }

        let parser = ParserConfig::new()
            .ignore_comments(false)
            .coalesce_characters(false)
            .max_entity_expansion_length(self.limits.max_decoded_bytes)
            .max_entity_expansion_depth(0)
            .create_reader(xml.as_bytes());
        let mut stats = XmlStats {
            input_bytes: xml.len(),
            ..XmlStats::default()
        };
        let mut depth = 0usize;
        let mut child_counts: Vec<usize> = Vec::new();

        for event in parser {
            stats.events = stats.events.saturating_add(1);
            check(self.limits, XmlLimit::Events, stats.events)?;
            let event = event.map_err(|_| XmlReadError::Malformed {
                kind: "not well-formed",
            })?;
            match event {
                XmlEvent::StartDocument { encoding, .. } => {
                    if !encoding.eq_ignore_ascii_case("utf-8")
                        && !encoding.eq_ignore_ascii_case("utf8")
                    {
                        return Err(XmlReadError::Forbidden {
                            construct: "non-UTF-8 encoding declaration",
                        });
                    }
                }
                XmlEvent::StartElement {
                    name, attributes, ..
                } => {
                    if name.local_name == "include"
                        && name.namespace.as_deref() == Some("http://www.w3.org/2001/XInclude")
                    {
                        return Err(XmlReadError::Forbidden {
                            construct: "XInclude",
                        });
                    }
                    if let Some(children) = child_counts.last_mut() {
                        *children = children.saturating_add(1);
                        check(self.limits, XmlLimit::CollectionItems, *children)?;
                        stats.max_collection_items = stats.max_collection_items.max(*children);
                    }
                    depth = depth.saturating_add(1);
                    check(self.limits, XmlLimit::Depth, depth)?;
                    stats.max_depth = stats.max_depth.max(depth);
                    stats.elements = stats.elements.saturating_add(1);
                    check(self.limits, XmlLimit::Elements, stats.elements)?;
                    stats.attributes = stats.attributes.saturating_add(attributes.len());
                    check(self.limits, XmlLimit::Attributes, stats.attributes)?;
                    child_counts.push(0usize);
                }
                XmlEvent::EndElement { .. } => {
                    depth = depth.checked_sub(1).ok_or(XmlReadError::Malformed {
                        kind: "unbalanced element",
                    })?;
                    child_counts.pop().ok_or(XmlReadError::Malformed {
                        kind: "unbalanced element",
                    })?;
                }
                XmlEvent::Characters(text)
                | XmlEvent::Whitespace(text)
                | XmlEvent::CData(text)
                | XmlEvent::Comment(text) => {
                    check(self.limits, XmlLimit::TextBytes, text.len())?;
                    stats.decoded_bytes = stats.decoded_bytes.saturating_add(text.len());
                    check(self.limits, XmlLimit::DecodedBytes, stats.decoded_bytes)?;
                }
                XmlEvent::ProcessingInstruction { .. } | XmlEvent::EndDocument => {}
            }
        }
        if depth != 0 || stats.elements == 0 {
            return Err(XmlReadError::Malformed {
                kind: "missing or unclosed root element",
            });
        }
        Ok(stats)
    }
}

fn check(limits: ParseLimits, limit: XmlLimit, actual: usize) -> Result<(), XmlReadError> {
    let maximum = match limit {
        XmlLimit::InputBytes => limits.max_input_bytes,
        XmlLimit::Depth => limits.max_depth,
        XmlLimit::Events => limits.max_events,
        XmlLimit::Elements => limits.max_elements,
        XmlLimit::Attributes => limits.max_attributes,
        XmlLimit::TextBytes => limits.max_text_bytes,
        XmlLimit::DecodedBytes => limits.max_decoded_bytes,
        XmlLimit::CollectionItems => limits.max_collection_items,
    };
    if actual > maximum {
        Err(XmlReadError::LimitExceeded { limit, maximum })
    } else {
        Ok(())
    }
}

/// Validate well-formedness and resource bounds without building a tree.
pub fn validate_xml(xml: &str, limits: ParseLimits) -> Result<XmlStats, XmlReadError> {
    BoundedXmlReader::new(limits).validate(xml)
}
