//! Business Application Header (BAH / `head.001`) support.
//!
//! A complete ISO 20022 *business message* is a Business Application Header
//! (`<AppHdr>`) followed by the message `<Document>`. This module provides a
//! lightweight, version-independent reader for the header's key fields
//! (sender, receiver, references, creation date) without needing the typed
//! `model`. The fully-typed header is available under
//! `generated::head::head_001_001_xx` with the `model` feature.
//!
//! ```
//! let xml = r#"
//!   <AppHdr xmlns="urn:iso:std:iso:20022:tech:xsd:head.001.001.02">
//!     <Fr><FIId><FinInstnId><BICFI>BANKBEBB</BICFI></FinInstnId></FIId></Fr>
//!     <To><FIId><FinInstnId><BICFI>BANKDEFF</BICFI></FinInstnId></FIId></To>
//!     <BizMsgIdr>MSG-001</BizMsgIdr>
//!     <MsgDefIdr>pacs.008.001.08</MsgDefIdr>
//!     <CreDt>2026-06-25T10:30:00Z</CreDt>
//!   </AppHdr>"#;
//! let h = rust_iso20022::app_hdr::parse_business_header(xml).unwrap();
//! assert_eq!(h.from.as_deref(), Some("BANKBEBB"));
//! assert_eq!(h.to.as_deref(), Some("BANKDEFF"));
//! assert_eq!(h.biz_msg_idr.as_deref(), Some("MSG-001"));
//! assert_eq!(h.msg_def_idr.as_deref(), Some("pacs.008.001.08"));
//! ```

use crate::core::xml_scan::{element_inner, element_text};

/// The key fields of a Business Application Header, read version-independently.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BusinessHeader {
    /// Sending party identifier (a BIC when present in `From`).
    pub from: Option<String>,
    /// Receiving party identifier (a BIC when present in `To`).
    pub to: Option<String>,
    /// Business message identifier (`BizMsgIdr`).
    pub biz_msg_idr: Option<String>,
    /// Message definition identifier (`MsgDefIdr`), e.g. `"pacs.008.001.08"`.
    pub msg_def_idr: Option<String>,
    /// Creation date/time (`CreDt`), as the raw ISO 8601 string.
    pub cre_dt: Option<String>,
}

impl BusinessHeader {
    /// Render this header as a `head.001` `<AppHdr>` element (default
    /// `head.001.001.02` namespace). `From`/`To` are emitted as financial
    /// institution BICs. Fields that are `None` are omitted.
    ///
    /// ```
    /// use rust_iso20022::app_hdr::BusinessHeader;
    /// let h = BusinessHeader {
    ///     from: Some("BANKBEBB".into()),
    ///     to: Some("BANKDEFF".into()),
    ///     biz_msg_idr: Some("MSG-1".into()),
    ///     msg_def_idr: Some("pacs.008.001.08".into()),
    ///     cre_dt: Some("2026-06-25T10:00:00Z".into()),
    /// };
    /// let xml = h.to_app_hdr_xml();
    /// assert!(xml.contains("<BICFI>BANKBEBB</BICFI>"));
    /// // and it round-trips back through the reader
    /// let back = rust_iso20022::app_hdr::parse_business_header(&xml).unwrap();
    /// assert_eq!(back, h);
    /// ```
    pub fn to_app_hdr_xml(&self) -> String {
        self.to_app_hdr_xml_ns("urn:iso:std:iso:20022:tech:xsd:head.001.001.02")
    }

    /// As [`to_app_hdr_xml`](Self::to_app_hdr_xml) but with an explicit BAH
    /// namespace.
    pub fn to_app_hdr_xml_ns(&self, namespace: &str) -> String {
        fn party(tag: &str, bic: &Option<String>) -> String {
            match bic {
                Some(b) => format!(
                    "<{tag}><FIId><FinInstnId><BICFI>{}</BICFI></FinInstnId></FIId></{tag}>",
                    xml_escape(b)
                ),
                None => String::new(),
            }
        }
        fn elem(tag: &str, v: &Option<String>) -> String {
            match v {
                Some(x) => format!("<{tag}>{}</{tag}>", xml_escape(x)),
                None => String::new(),
            }
        }
        format!(
            "<AppHdr xmlns=\"{ns}\">{fr}{to}{bmi}{mdi}{cd}</AppHdr>",
            ns = namespace,
            fr = party("Fr", &self.from),
            to = party("To", &self.to),
            bmi = elem("BizMsgIdr", &self.biz_msg_idr),
            mdi = elem("MsgDefIdr", &self.msg_def_idr),
            cd = elem("CreDt", &self.cre_dt),
        )
    }
}

/// Minimal XML text escaping for element content.
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Read a [`BusinessHeader`] from XML containing an `<AppHdr>` element. Returns
/// `None` if no header fields are found.
pub fn parse_business_header(xml: &str) -> Option<BusinessHeader> {
    // Narrow to the AppHdr element if present, so a trailing Document does not
    // leak its own fields (e.g. a Document-level CreDt) into the header.
    let scope = element_inner(xml, "AppHdr").unwrap_or_else(|| xml.to_string());

    let from = element_inner(&scope, "Fr").and_then(|s| first_bic(&s));
    let to = element_inner(&scope, "To").and_then(|s| first_bic(&s));
    let biz_msg_idr = element_text(&scope, "BizMsgIdr");
    let msg_def_idr = element_text(&scope, "MsgDefIdr");
    let cre_dt = element_text(&scope, "CreDt");

    if from.is_none()
        && to.is_none()
        && biz_msg_idr.is_none()
        && msg_def_idr.is_none()
        && cre_dt.is_none()
    {
        return None;
    }
    Some(BusinessHeader {
        from,
        to,
        biz_msg_idr,
        msg_def_idr,
        cre_dt,
    })
}

/// Find the first BIC-like value inside a `Fr`/`To` fragment.
fn first_bic(fragment: &str) -> Option<String> {
    for tag in ["BICFI", "AnyBIC", "BIC", "Id"] {
        if let Some(v) = element_text(fragment, tag) {
            if !v.is_empty() {
                return Some(v);
            }
        }
    }
    None
}
