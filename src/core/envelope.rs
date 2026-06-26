//! Reading a complete ISO 20022 *business message* (Business Application Header
//! + message Document) in one call, without the `model` feature.

use crate::core::app_hdr::{parse_business_header, BusinessHeader};
use crate::core::metadata::{extract, MessageMetadata};
use crate::core::mx_message::detect;
use crate::core::MxId;

/// A parsed business message: its optional header, the detected message type,
/// and extracted business metadata.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BusinessMessage {
    /// The Business Application Header, if an `<AppHdr>` is present.
    pub header: Option<BusinessHeader>,
    /// The detected message identification (from the `Document` namespace).
    pub id: Option<MxId>,
    /// Business metadata read from the message.
    pub metadata: MessageMetadata,
}

/// Read a business message (header + document) from XML.
///
/// ```
/// let xml = r#"
///   <Envelope>
///     <AppHdr xmlns="urn:iso:std:iso:20022:tech:xsd:head.001.001.02">
///       <BizMsgIdr>B-1</BizMsgIdr><MsgDefIdr>pacs.008.001.08</MsgDefIdr>
///     </AppHdr>
///     <Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
///       <FIToFICstmrCdtTrf><GrpHdr><MsgId>M-1</MsgId></GrpHdr></FIToFICstmrCdtTrf>
///     </Document>
///   </Envelope>"#;
/// let bm = rust_iso20022::read_business_message(xml);
/// assert_eq!(bm.id.unwrap().message_name(), "pacs.008.001.08");
/// assert_eq!(bm.header.unwrap().biz_msg_idr.as_deref(), Some("B-1"));
/// assert_eq!(bm.metadata.message_id.as_deref(), Some("M-1"));
/// ```
pub fn read_business_message(xml: &str) -> BusinessMessage {
    BusinessMessage {
        header: parse_business_header(xml),
        id: detect(xml),
        metadata: extract(xml),
    }
}
