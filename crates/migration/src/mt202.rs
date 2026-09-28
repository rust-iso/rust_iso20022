//! Explicit MT202 to generated `pacs.009.001.08` conversion.

use core::fmt;

use rust_iso20022::{
    builders::Pacs009Builder,
    generated::pacs::pacs_009_001_08 as pacs,
    helpers::{BicFi, IsoDate, Money},
};

use crate::{
    mt::{MtDocument, MtParseError},
    report::{
        MappingClassification as Class, MappingEntry, MappingReport, MappingReportError,
        MappingWarning, SourceFieldRef,
    },
};

#[derive(Debug)]
pub struct Mt202Conversion {
    pub message: pacs::Document,
    pub mapping_report: MappingReport,
}

pub fn convert(input: &str) -> Result<Mt202Conversion, Mt202Error> {
    let document = MtDocument::parse(input).map_err(Mt202Error::Parse)?;
    let reference = required(&document, "20")?;
    let date_amount = required(&document, "32A")?;
    let (date, currency, amount) = parse_32a(date_amount.value())?;
    let money =
        Money::parse(currency, &amount).map_err(|_| Mt202Error::InvalidField { tag: "32A" })?;
    let mut message = Pacs009Builder::new()
        .message_id(reference.value())
        .transaction_id(reference.value())
        .settlement(money)
        .build()
        .map_err(|_| Mt202Error::Build)?;
    let transaction = &mut message.fi_cdt_trf.cdt_trf_tx_inf[0];
    transaction.intr_bk_sttlm_dt = pacs::Isodate(date);

    let mut entries = Vec::new();
    let mut warnings = Vec::new();
    for field in document.fields() {
        let source = SourceFieldRef::from_document(&document, field.index()).ok_or(
            Mt202Error::Report(MappingReportError::UnknownSource {
                index: field.index(),
            }),
        )?;
        let (target, classification, note) = match field.tag() {
            "20" => (
                Some("Document/FICdtTrf/GrpHdr/MsgId"),
                Class::Derived,
                "also supplies end-to-end and transaction identifiers",
            ),
            "21" => {
                transaction.pmt_id.instr_id = pacs::Max35Text(field.value().to_owned());
                warnings.push(warning(
                    field.index(),
                    "MT202-21-AMBIGUOUS",
                    "related reference is provisionally mapped as instruction identification",
                ));
                (
                    Some("Document/FICdtTrf/CdtTrfTxInf/PmtId/InstrId"),
                    Class::Ambiguous,
                    "pacs.009 has no exact MT related-reference equivalent",
                )
            }
            "32A" => {
                warnings.push(warning(
                    field.index(),
                    "MT202-32A-CENTURY",
                    "two-digit MT year is interpreted using the documented 20YY policy",
                ));
                (
                    Some("Document/FICdtTrf/CdtTrfTxInf/IntrBkSttlmAmt"),
                    Class::Ambiguous,
                    "date/currency/amount are split; the two-digit year uses a 20YY policy",
                )
            }
            "52A" => {
                set_bic(&mut transaction.dbtr, field.value(), "52A")?;
                (
                    Some("Document/FICdtTrf/CdtTrfTxInf/Dbtr/FinInstnId/BICFI"),
                    Class::Exact,
                    "ordering institution BIC maps directly",
                )
            }
            "53A" => {
                set_bic(&mut transaction.instg_agt, field.value(), "53A")?;
                warnings.push(warning(
                    field.index(),
                    "MT202-53A-AMBIGUOUS",
                    "sender correspondent role requires scheme-specific confirmation",
                ));
                (
                    Some("Document/FICdtTrf/CdtTrfTxInf/InstgAgt/FinInstnId/BICFI"),
                    Class::Ambiguous,
                    "provisionally mapped to instructing agent",
                )
            }
            "56A" => {
                set_bic(&mut transaction.intrmy_agt_1, field.value(), "56A")?;
                (
                    Some("Document/FICdtTrf/CdtTrfTxInf/IntrmyAgt1/FinInstnId/BICFI"),
                    Class::Derived,
                    "correspondent chain position maps to first intermediary agent",
                )
            }
            "57A" => {
                set_bic(&mut transaction.cdtr_agt, field.value(), "57A")?;
                (
                    Some("Document/FICdtTrf/CdtTrfTxInf/CdtrAgt/FinInstnId/BICFI"),
                    Class::Exact,
                    "account-with institution BIC maps directly",
                )
            }
            "58A" => {
                set_bic(&mut transaction.cdtr, field.value(), "58A")?;
                (
                    Some("Document/FICdtTrf/CdtTrfTxInf/Cdtr/FinInstnId/BICFI"),
                    Class::Exact,
                    "beneficiary institution BIC maps directly",
                )
            }
            "72" => {
                warnings.push(warning(
                    field.index(),
                    "MT202-72-AMBIGUOUS",
                    "sender-to-receiver text requires manual interpretation",
                ));
                (None, Class::Ambiguous, "no automatic target selected")
            }
            _ => {
                warnings.push(warning(
                    field.index(),
                    "MT202-FIELD-UNSUPPORTED",
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
        MappingReport::build(&document, entries, warnings).map_err(Mt202Error::Report)?;
    Ok(Mt202Conversion {
        message,
        mapping_report,
    })
}

fn required<'a>(
    document: &'a MtDocument,
    tag: &'static str,
) -> Result<&'a crate::mt::MtField, Mt202Error> {
    document
        .fields()
        .iter()
        .find(|field| field.tag() == tag)
        .ok_or(Mt202Error::MissingField { tag })
}

fn parse_32a(value: &str) -> Result<(String, &str, String), Mt202Error> {
    let bytes = value.as_bytes();
    if bytes.len() < 10
        || !bytes[..6].iter().all(u8::is_ascii_digit)
        || !bytes[6..9].iter().all(u8::is_ascii_uppercase)
    {
        return Err(Mt202Error::InvalidField { tag: "32A" });
    }
    let date = format!("20{}-{}-{}", &value[0..2], &value[2..4], &value[4..6]);
    let date = IsoDate::parse(&date).map_err(|_| Mt202Error::InvalidField { tag: "32A" })?;
    Ok((date.into(), &value[6..9], value[9..].replace(',', ".")))
}

fn set_bic(
    target: &mut pacs::BranchAndFinancialInstitutionIdentification6,
    value: &str,
    tag: &'static str,
) -> Result<(), Mt202Error> {
    let bic = BicFi::parse(value).map_err(|_| Mt202Error::InvalidField { tag })?;
    target.fin_instn_id.bicfi = pacs::Bicfidec2014Identifier(bic.into());
    Ok(())
}

fn warning(index: usize, code: &str, message: &str) -> MappingWarning {
    MappingWarning {
        code: code.to_owned(),
        message: message.to_owned(),
        source_index: Some(index),
    }
}

#[derive(Debug)]
pub enum Mt202Error {
    Parse(MtParseError),
    MissingField { tag: &'static str },
    InvalidField { tag: &'static str },
    Build,
    Report(MappingReportError),
}

impl fmt::Display for Mt202Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("MT202 conversion failed; inspect the typed error variant")
    }
}

impl std::error::Error for Mt202Error {}
