//! Explicit MT103 to generated `pacs.008.001.08` conversion.

use core::fmt;

use rust_iso20022::{
    builders::Pacs008Builder,
    generated::pacs::pacs_008_001_08 as pacs,
    helpers::{BicFi, Iban, IsoDate, Money},
};

use crate::{
    mt::{MtDocument, MtParseError},
    report::{
        MappingClassification as Class, MappingEntry, MappingReport, MappingReportError,
        MappingWarning, SourceFieldRef,
    },
};

#[derive(Debug)]
pub struct Mt103Conversion {
    pub message: pacs::Document,
    pub mapping_report: MappingReport,
}

pub fn convert(input: &str) -> Result<Mt103Conversion, Mt103Error> {
    let document = MtDocument::parse(input).map_err(Mt103Error::Parse)?;
    let reference = required(&document, "20")?;
    let date_amount = required(&document, "32A")?;
    let (date, currency, amount) = parse_32a(date_amount.value())?;
    let money =
        Money::parse(currency, &amount).map_err(|_| Mt103Error::InvalidField { tag: "32A" })?;
    let mut message = Pacs008Builder::new()
        .message_id(reference.value())
        .transaction_id(reference.value())
        .settlement(money)
        .build()
        .map_err(|_| Mt103Error::Build)?;
    let transaction = &mut message.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf[0];
    transaction.intr_bk_sttlm_dt = pacs::Isodate(date);

    let mut entries = Vec::new();
    let mut warnings = Vec::new();
    for field in document.fields() {
        let source = SourceFieldRef::from_document(&document, field.index()).ok_or(
            Mt103Error::Report(MappingReportError::UnknownSource {
                index: field.index(),
            }),
        )?;
        let (target, classification, note) = match field.tag() {
            "20" => (
                Some("Document/FIToFICstmrCdtTrf/GrpHdr/MsgId"),
                Class::Derived,
                "also supplies transaction identifiers",
            ),
            "32A" => {
                warnings.push(warning(
                    field.index(),
                    "MT103-32A-CENTURY",
                    "two-digit MT year is interpreted using the documented 20YY policy",
                ));
                (
                    Some("Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt"),
                    Class::Ambiguous,
                    "date/currency/amount are split; the two-digit year uses an explicit 20YY policy",
                )
            }
            "50K" => {
                let (account, name) = party(field.value());
                if let Some(account) = account {
                    let account = Iban::parse(&account)
                        .map_err(|_| Mt103Error::InvalidField { tag: "50K" })?;
                    transaction.dbtr_acct.id.iban = Some(pacs::Iban2007Identifier(account.into()));
                }
                transaction.dbtr.nm = pacs::Max140Text(name);
                warnings.push(warning(
                    field.index(),
                    "MT103-50K-LOSSY",
                    "ordering-customer address lines are not mapped",
                ));
                (
                    Some("Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Dbtr"),
                    Class::Lossy,
                    "account and name mapped; address lines retained only in source evidence",
                )
            }
            "52A" => {
                let bic = BicFi::parse(field.value())
                    .map_err(|_| Mt103Error::InvalidField { tag: "52A" })?;
                transaction.dbtr_agt.fin_instn_id.bicfi = pacs::Bicfidec2014Identifier(bic.into());
                (
                    Some("Document/FIToFICstmrCdtTrf/CdtTrfTxInf/DbtrAgt/FinInstnId/BICFI"),
                    Class::Exact,
                    "BIC maps directly",
                )
            }
            "57A" => {
                let bic = BicFi::parse(field.value())
                    .map_err(|_| Mt103Error::InvalidField { tag: "57A" })?;
                transaction.cdtr_agt.fin_instn_id.bicfi = pacs::Bicfidec2014Identifier(bic.into());
                (
                    Some("Document/FIToFICstmrCdtTrf/CdtTrfTxInf/CdtrAgt/FinInstnId/BICFI"),
                    Class::Exact,
                    "BIC maps directly",
                )
            }
            "59" => {
                let (account, name) = party(field.value());
                if let Some(account) = account {
                    let account = Iban::parse(&account)
                        .map_err(|_| Mt103Error::InvalidField { tag: "59" })?;
                    transaction.cdtr_acct.id.iban = Some(pacs::Iban2007Identifier(account.into()));
                }
                transaction.cdtr.nm = pacs::Max140Text(name);
                warnings.push(warning(
                    field.index(),
                    "MT103-59-LOSSY",
                    "beneficiary address lines are not mapped",
                ));
                (
                    Some("Document/FIToFICstmrCdtTrf/CdtTrfTxInf/Cdtr"),
                    Class::Lossy,
                    "account and name mapped; address lines retained only in source evidence",
                )
            }
            "70" => {
                transaction
                    .rmt_inf
                    .ustrd
                    .push(pacs::Max140Text(field.value().to_owned()));
                (
                    Some("Document/FIToFICstmrCdtTrf/CdtTrfTxInf/RmtInf/Ustrd"),
                    Class::Exact,
                    "unstructured remittance maps directly",
                )
            }
            "71A" => {
                transaction.chrg_br = match field.value() {
                    "OUR" => pacs::ChargeBearerType1Code::Debt,
                    "BEN" => pacs::ChargeBearerType1Code::Cred,
                    "SHA" => pacs::ChargeBearerType1Code::Shar,
                    _ => return Err(Mt103Error::InvalidField { tag: "71A" }),
                };
                (
                    Some("Document/FIToFICstmrCdtTrf/CdtTrfTxInf/ChrgBr"),
                    Class::Derived,
                    "MT charge code maps to ISO charge-bearer code",
                )
            }
            "72" => {
                warnings.push(warning(
                    field.index(),
                    "MT103-72-AMBIGUOUS",
                    "sender-to-receiver text requires manual interpretation",
                ));
                (None, Class::Ambiguous, "no automatic target selected")
            }
            _ => {
                warnings.push(warning(
                    field.index(),
                    "MT103-FIELD-UNSUPPORTED",
                    "field is not supported by this conversion release",
                ));
                (
                    None,
                    Class::Unsupported,
                    "field is intentionally not mapped",
                )
            }
        };
        entries.push(MappingEntry::new(
            source,
            target.map(str::to_owned),
            classification,
            note,
        ));
    }
    let mapping_report =
        MappingReport::build(&document, entries, warnings).map_err(Mt103Error::Report)?;
    Ok(Mt103Conversion {
        message,
        mapping_report,
    })
}

fn required<'a>(
    document: &'a MtDocument,
    tag: &'static str,
) -> Result<&'a crate::mt::MtField, Mt103Error> {
    document
        .fields()
        .iter()
        .find(|field| field.tag() == tag)
        .ok_or(Mt103Error::MissingField { tag })
}

fn parse_32a(value: &str) -> Result<(String, &str, String), Mt103Error> {
    let bytes = value.as_bytes();
    if bytes.len() < 10
        || !bytes[..6].iter().all(u8::is_ascii_digit)
        || !bytes[6..9].iter().all(u8::is_ascii_uppercase)
    {
        return Err(Mt103Error::InvalidField { tag: "32A" });
    }
    let date = format!("20{}-{}-{}", &value[0..2], &value[2..4], &value[4..6]);
    let date = IsoDate::parse(&date).map_err(|_| Mt103Error::InvalidField { tag: "32A" })?;
    let currency = &value[6..9];
    let amount = value[9..].replace(',', ".");
    Ok((date.into(), currency, amount))
}

fn party(value: &str) -> (Option<String>, String) {
    let mut lines = value.lines();
    let first = lines.next().unwrap_or_default();
    if let Some(account) = first.strip_prefix('/') {
        (
            Some(account.to_owned()),
            lines.next().unwrap_or_default().to_owned(),
        )
    } else {
        (None, first.to_owned())
    }
}

fn warning(index: usize, code: &str, message: &str) -> MappingWarning {
    MappingWarning {
        code: code.to_owned(),
        message: message.to_owned(),
        source_index: Some(index),
    }
}

#[derive(Debug)]
pub enum Mt103Error {
    Parse(MtParseError),
    MissingField { tag: &'static str },
    InvalidField { tag: &'static str },
    Build,
    Report(MappingReportError),
}
impl fmt::Display for Mt103Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("MT103 conversion failed; inspect the typed error variant")
    }
}
impl std::error::Error for Mt103Error {}
