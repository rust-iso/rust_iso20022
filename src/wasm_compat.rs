//! Serialization-only presentation shims for the original JavaScript contract.

use serde::Serialize;

pub fn mx_id(namespace_or_name: &str) -> Option<String> {
    let id = crate::MxId::parse(namespace_or_name).ok()?;
    Some(json(&LegacyMxId {
        message_name: id.message_name(),
        namespace: id.namespace(),
        business_area: id.business_area.code(),
        functionality: &id.functionality,
        variant: &id.variant,
        version: &id.version,
    }))
}

pub fn catalogue_entry(message_name: &str) -> Option<String> {
    let entry = crate::catalogue::from_message_name(message_name)?;
    Some(json(&LegacyCatalogueEntry {
        message_name: entry.message_name,
        namespace: entry.namespace,
        business_area: entry.business_area,
        has_model: entry.has_model,
    }))
}

pub fn header(header: &crate::app_hdr::BusinessHeader) -> String {
    json(&LegacyHeader::from(header))
}

pub fn metadata(metadata: &crate::metadata::MessageMetadata) -> String {
    json(&LegacyMetadata::from(metadata))
}

pub fn business_message(message: &crate::BusinessMessage) -> String {
    json(&LegacyBusinessMessage {
        message_name: message.id.as_ref().map(crate::MxId::message_name),
        header: message.header.as_ref().map(LegacyHeader::from),
        metadata: LegacyMetadata::from(&message.metadata),
    })
}

pub fn node(node: &crate::MxNode) -> String {
    json(&LegacyNode::from(node))
}

fn json(value: &impl Serialize) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "null".to_owned())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LegacyMxId<'a> {
    message_name: String,
    namespace: String,
    business_area: &'a str,
    functionality: &'a str,
    variant: &'a str,
    version: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LegacyCatalogueEntry<'a> {
    message_name: &'a str,
    namespace: &'a str,
    business_area: &'a str,
    has_model: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LegacyHeader<'a> {
    from: &'a Option<String>,
    to: &'a Option<String>,
    biz_msg_idr: &'a Option<String>,
    msg_def_idr: &'a Option<String>,
    cre_dt: &'a Option<String>,
}

impl<'a> From<&'a crate::app_hdr::BusinessHeader> for LegacyHeader<'a> {
    fn from(header: &'a crate::app_hdr::BusinessHeader) -> Self {
        Self {
            from: &header.from,
            to: &header.to,
            biz_msg_idr: &header.biz_msg_idr,
            msg_def_idr: &header.msg_def_idr,
            cre_dt: &header.cre_dt,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LegacyMetadata<'a> {
    message_id: &'a Option<String>,
    creation_date_time: &'a Option<String>,
    number_of_transactions: &'a Option<String>,
    amount: &'a Option<String>,
    currency: &'a Option<String>,
    value_date: &'a Option<String>,
}

impl<'a> From<&'a crate::metadata::MessageMetadata> for LegacyMetadata<'a> {
    fn from(metadata: &'a crate::metadata::MessageMetadata) -> Self {
        Self {
            message_id: &metadata.message_id,
            creation_date_time: &metadata.creation_date_time,
            number_of_transactions: &metadata.number_of_transactions,
            amount: &metadata.amount,
            currency: &metadata.currency,
            value_date: &metadata.value_date,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LegacyBusinessMessage<'a> {
    message_name: Option<String>,
    header: Option<LegacyHeader<'a>>,
    metadata: LegacyMetadata<'a>,
}

#[derive(Serialize)]
struct LegacyNode<'a> {
    name: &'a str,
    value: &'a Option<String>,
    attributes: std::collections::BTreeMap<&'a str, &'a str>,
    children: Vec<LegacyNode<'a>>,
}

impl<'a> From<&'a crate::MxNode> for LegacyNode<'a> {
    fn from(node: &'a crate::MxNode) -> Self {
        Self {
            name: &node.name,
            value: &node.value,
            attributes: node
                .attributes
                .iter()
                .map(|(name, value)| (name.as_str(), value.as_str()))
                .collect(),
            children: node.children.iter().map(Self::from).collect(),
        }
    }
}
