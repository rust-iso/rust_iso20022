//! A generic ISO 20022 message tree ([`MxNode`]) — read any message's fields
//! without the typed model (and without a `model-<area>` feature).
//!
//! This is the lightweight counterpart to a typed `Document`: parse any MX XML
//! into a tree of named nodes and navigate it by local element name.
//!
//! ```
//! use rust_iso20022::MxNode;
//!
//! let xml = r#"
//!   <Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
//!     <FIToFICstmrCdtTrf>
//!       <GrpHdr><MsgId>ABC-1</MsgId></GrpHdr>
//!       <CdtTrfTxInf><IntrBkSttlmAmt Ccy="EUR">10.00</IntrBkSttlmAmt></CdtTrfTxInf>
//!       <CdtTrfTxInf><IntrBkSttlmAmt Ccy="USD">20.00</IntrBkSttlmAmt></CdtTrfTxInf>
//!     </FIToFICstmrCdtTrf>
//!   </Document>"#;
//!
//! let doc = MxNode::parse(xml).unwrap();
//! // navigate by path
//! assert_eq!(doc.at(&["FIToFICstmrCdtTrf", "GrpHdr", "MsgId"]).and_then(|n| n.text()), Some("ABC-1"));
//! // first descendant by name
//! assert_eq!(doc.find("MsgId").and_then(|n| n.text()), Some("ABC-1"));
//! // all repeated elements + an attribute
//! let amts = doc.find_all("IntrBkSttlmAmt");
//! assert_eq!(amts.len(), 2);
//! assert_eq!(amts[0].attr("Ccy"), Some("EUR"));
//! assert_eq!(amts[1].text(), Some("20.00"));
//! ```

use xml::reader::{EventReader, XmlEvent};

/// A node in a parsed ISO 20022 message tree: an element with its local name,
/// attributes, text value (for a leaf) and child elements.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MxNode {
    /// Local element name (namespace prefix stripped), e.g. `"MsgId"`.
    pub name: String,
    /// Text content, for a leaf element.
    pub value: Option<String>,
    /// Attributes as `(local-name, value)` pairs, e.g. `("Ccy", "EUR")`.
    pub attributes: Vec<(String, String)>,
    /// Child elements.
    pub children: Vec<MxNode>,
}

impl MxNode {
    /// Parse XML into a tree, returning the root element (e.g. `Document` or
    /// `AppHdr`). Returns `None` if the XML has no element or is malformed.
    pub fn parse(xml: &str) -> Option<MxNode> {
        let parser = EventReader::from_str(xml);
        let mut stack: Vec<MxNode> = Vec::new();
        let mut root: Option<MxNode> = None;
        for event in parser {
            match event.ok()? {
                XmlEvent::StartElement {
                    name, attributes, ..
                } => {
                    let node = MxNode {
                        name: name.local_name,
                        value: None,
                        attributes: attributes
                            .into_iter()
                            .map(|a| (a.name.local_name, a.value))
                            .collect(),
                        children: Vec::new(),
                    };
                    stack.push(node);
                }
                XmlEvent::Characters(text) => {
                    if let Some(top) = stack.last_mut() {
                        let t = text.trim();
                        if !t.is_empty() {
                            top.value = Some(match top.value.take() {
                                Some(mut v) => {
                                    v.push_str(t);
                                    v
                                }
                                None => t.to_string(),
                            });
                        }
                    }
                }
                XmlEvent::EndElement { .. } => {
                    if let Some(node) = stack.pop() {
                        match stack.last_mut() {
                            Some(parent) => parent.children.push(node),
                            None => root = Some(node),
                        }
                    }
                }
                _ => {}
            }
        }
        root
    }

    /// The text value of this node, if it is a leaf with content.
    pub fn text(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// The value of an attribute by local name, e.g. `"Ccy"`.
    pub fn attr(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }

    /// The first direct child with the given local name.
    pub fn get(&self, name: &str) -> Option<&MxNode> {
        self.children.iter().find(|c| c.name == name)
    }

    /// Navigate a path of local element names from this node.
    pub fn at(&self, path: &[&str]) -> Option<&MxNode> {
        let mut cur = self;
        for seg in path {
            cur = cur.get(seg)?;
        }
        Some(cur)
    }

    /// The first descendant (depth-first) with the given local name.
    pub fn find(&self, name: &str) -> Option<&MxNode> {
        for c in &self.children {
            if c.name == name {
                return Some(c);
            }
            if let Some(found) = c.find(name) {
                return Some(found);
            }
        }
        None
    }

    /// All descendants with the given local name.
    pub fn find_all<'a>(&'a self, name: &str) -> Vec<&'a MxNode> {
        let mut out = Vec::new();
        self.collect(name, &mut out);
        out
    }

    fn collect<'a>(&'a self, name: &str, out: &mut Vec<&'a MxNode>) {
        for c in &self.children {
            if c.name == name {
                out.push(c);
            }
            c.collect(name, out);
        }
    }
}
