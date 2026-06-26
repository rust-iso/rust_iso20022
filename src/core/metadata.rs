//! Business metadata extraction from a message `Document` — the rust parallel
//! to prowide's `MxSwiftMessage` / metadata strategy.
//!
//! Like [`app_hdr`](crate::app_hdr), this reads well-known ISO 20022 elements by
//! local name directly from the XML, so it works on any message version and
//! without the `model` feature. It is best-effort: fields not present in a given
//! message are `None`.
//!
//! ```
//! let xml = r#"
//!   <Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
//!     <FIToFICstmrCdtTrf>
//!       <GrpHdr><MsgId>ABC-123</MsgId><CreDtTm>2026-06-25T10:00:00Z</CreDtTm><NbOfTxs>1</NbOfTxs></GrpHdr>
//!       <CdtTrfTxInf><IntrBkSttlmAmt Ccy="EUR">1234.56</IntrBkSttlmAmt><IntrBkSttlmDt>2026-06-26</IntrBkSttlmDt></CdtTrfTxInf>
//!     </FIToFICstmrCdtTrf>
//!   </Document>"#;
//! let m = rust_iso20022::metadata::extract(xml);
//! assert_eq!(m.message_id.as_deref(), Some("ABC-123"));
//! assert_eq!(m.amount.as_deref(), Some("1234.56"));
//! assert_eq!(m.currency.as_deref(), Some("EUR"));
//! assert_eq!(m.value_date.as_deref(), Some("2026-06-26"));
//! ```

use crate::core::xml_scan::{element_attr, element_text, first_of};

/// Common business metadata extracted from a message.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageMetadata {
    /// Group-header message identifier (`GrpHdr/MsgId`).
    pub message_id: Option<String>,
    /// Group-header creation date/time (`GrpHdr/CreDtTm`).
    pub creation_date_time: Option<String>,
    /// Number of transactions (`GrpHdr/NbOfTxs`).
    pub number_of_transactions: Option<String>,
    /// The first settlement/instructed amount found, as text.
    pub amount: Option<String>,
    /// The currency (`Ccy`) of that amount.
    pub currency: Option<String>,
    /// The settlement / requested execution date.
    pub value_date: Option<String>,
}

/// Amount element names to look for, in priority order.
const AMOUNT_TAGS: &[&str] = &[
    "TtlIntrBkSttlmAmt",
    "IntrBkSttlmAmt",
    "InstdAmt",
    "EqvtAmt",
    "TtlInstdAmt",
    "Amt",
];

/// Date element names to look for, in priority order.
const DATE_TAGS: &[&str] = &["IntrBkSttlmDt", "ReqdExctnDt", "ReqdColltnDt", "IntrBkSttlmDtTm"];

/// Extract [`MessageMetadata`] from message XML.
pub fn extract(xml: &str) -> MessageMetadata {
    let (amount, currency) = AMOUNT_TAGS
        .iter()
        .find_map(|&tag| {
            element_text(xml, tag)
                .filter(|v| !v.is_empty())
                .map(|v| (Some(v), element_attr(xml, tag, "Ccy")))
        })
        .unwrap_or((None, None));

    MessageMetadata {
        message_id: element_text(xml, "MsgId").filter(|v| !v.is_empty()),
        creation_date_time: element_text(xml, "CreDtTm").filter(|v| !v.is_empty()),
        number_of_transactions: element_text(xml, "NbOfTxs").filter(|v| !v.is_empty()),
        amount,
        currency,
        value_date: first_of(xml, DATE_TAGS).map(|(_, v)| v).filter(|v| !v.is_empty()),
    }
}
