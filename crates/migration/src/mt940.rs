//! Explicit MT940 to generated `camt.053.001.09` conversion.

use core::fmt;

use rust_iso20022::{
    generated::camt::camt_053_001_09 as camt,
    helpers::{AccountIdentifier, Iban, IsoDate, Money},
};

use crate::{
    mt::{MtDocument, MtParseError},
    report::{
        MappingClassification as Class, MappingEntry, MappingReport, MappingReportError,
        MappingWarning, SourceFieldRef,
    },
};

#[derive(Debug)]
pub struct Mt940Conversion {
    pub message: camt::Document,
    pub mapping_report: MappingReport,
}

pub fn convert(input: &str) -> Result<Mt940Conversion, Mt940Error> {
    let document = MtDocument::parse(input).map_err(Mt940Error::Parse)?;
    let reference = required(&document, "20")?;
    let account = required(&document, "25")?;
    let statement_number = required(&document, "28C")?;
    bounded_identifier(reference.value(), "20")?;
    bounded_identifier(statement_number.value(), "28C")?;
    let mut message = camt::Document::default();
    message.bk_to_cstmr_stmt.grp_hdr.msg_id = camt::Max35Text(reference.value().to_owned());
    message
        .bk_to_cstmr_stmt
        .stmt
        .push(camt::AccountStatement10 {
            id: camt::Max35Text(statement_number.value().to_owned()),
            ..Default::default()
        });
    let statement = &mut message.bk_to_cstmr_stmt.stmt[0];
    set_account(&mut statement.acct, account.value())?;

    let statement_currency = document
        .fields()
        .iter()
        .find(|field| matches!(field.tag(), "60F" | "60M" | "62F" | "62M"))
        .map(|field| parse_balance(field.tag(), field.value()))
        .transpose()?
        .map(|balance| balance.currency)
        .ok_or(Mt940Error::MissingField { tag: "60F/62F" })?;
    statement.acct.ccy = camt::ActiveOrHistoricCurrencyCode(statement_currency.clone());

    let mut entries = Vec::new();
    let mut warnings = Vec::new();
    let mut last_entry = None;
    for field in document.fields() {
        let source = SourceFieldRef::from_document(&document, field.index()).ok_or(
            Mt940Error::Report(MappingReportError::UnknownSource {
                index: field.index(),
            }),
        )?;
        let (target, classification, note) = match field.tag() {
            "20" => (
                Some("Document/BkToCstmrStmt/GrpHdr/MsgId"),
                Class::Exact,
                "statement reference maps to message identification",
            ),
            "25" => (
                Some("Document/BkToCstmrStmt/Stmt/Acct/Id"),
                if Iban::parse(field.value()).is_ok() {
                    Class::Exact
                } else {
                    Class::Derived
                },
                "IBAN maps directly; other bounded identifiers use the ISO 20022 other choice",
            ),
            "28C" => {
                set_sequence(statement, field.value())?;
                (
                    Some("Document/BkToCstmrStmt/Stmt/Id"),
                    Class::Derived,
                    "statement/sequence text supplies statement ID and numeric sequence when present",
                )
            }
            "60F" | "60M" | "62F" | "62M" | "64" | "65" => {
                let balance = parse_balance(field.tag(), field.value())?;
                statement.bal.push(balance.into_generated());
                warnings.push(warning(
                    field.index(),
                    "MT940-BALANCE-CENTURY",
                    "two-digit balance date is interpreted using the documented 20YY policy",
                ));
                (
                    Some("Document/BkToCstmrStmt/Stmt/Bal"),
                    Class::Ambiguous,
                    "balance type is explicit, but the two-digit date requires the 20YY policy",
                )
            }
            "61" => {
                let parsed = parse_entry(field.value(), &statement_currency)?;
                statement.ntry.push(parsed.into_generated(field.index()));
                last_entry = Some(statement.ntry.len() - 1);
                warnings.push(warning(
                    field.index(),
                    "MT940-61-LOSSY",
                    "entry subfields without a direct camt.053 projection remain described by the mapping report",
                ));
                warnings.push(warning(
                    field.index(),
                    "MT940-ENTRY-CENTURY",
                    "two-digit entry date is interpreted using the documented 20YY policy",
                ));
                (
                    Some("Document/BkToCstmrStmt/Stmt/Ntry"),
                    Class::Lossy,
                    "amount, direction, value date, reference, and transaction code are retained; MT-specific substructure is not fully represented",
                )
            }
            "86" => {
                let index = last_entry.ok_or(Mt940Error::OrphanInformation)?;
                statement.ntry[index].addtl_ntry_inf = camt::Max500Text(field.value().to_owned());
                warnings.push(warning(
                    field.index(),
                    "MT940-86-AMBIGUOUS",
                    "free text is associated with the immediately preceding entry and requires review",
                ));
                (
                    Some("Document/BkToCstmrStmt/Stmt/Ntry/AddtlNtryInf"),
                    Class::Ambiguous,
                    "association by source order is preserved but structured semantics are not inferred",
                )
            }
            _ => {
                warnings.push(warning(
                    field.index(),
                    "MT940-FIELD-UNSUPPORTED",
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
        MappingReport::build(&document, entries, warnings).map_err(Mt940Error::Report)?;
    Ok(Mt940Conversion {
        message,
        mapping_report,
    })
}

fn required<'a>(
    document: &'a MtDocument,
    tag: &'static str,
) -> Result<&'a crate::mt::MtField, Mt940Error> {
    document
        .fields()
        .iter()
        .find(|field| field.tag() == tag)
        .ok_or(Mt940Error::MissingField { tag })
}

fn bounded_identifier(value: &str, tag: &'static str) -> Result<(), Mt940Error> {
    if value.is_empty() || value.len() > 35 || value.chars().any(char::is_control) {
        return Err(Mt940Error::InvalidField { tag });
    }
    Ok(())
}

fn set_account(target: &mut camt::CashAccount41, value: &str) -> Result<(), Mt940Error> {
    if let Ok(iban) = Iban::parse(value) {
        target.id.iban = Some(camt::Iban2007Identifier(iban.as_str().to_owned()));
        return Ok(());
    }
    let account =
        AccountIdentifier::parse(value).map_err(|_| Mt940Error::InvalidField { tag: "25" })?;
    target.id.othr = Some(camt::GenericAccountIdentification1 {
        id: camt::Max34Text(account.as_str().to_owned()),
        ..Default::default()
    });
    Ok(())
}

fn set_sequence(statement: &mut camt::AccountStatement10, value: &str) -> Result<(), Mt940Error> {
    let sequence = value.split('/').next().unwrap_or(value);
    if sequence.is_empty() || !sequence.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(Mt940Error::InvalidField { tag: "28C" });
    }
    statement.elctrnc_seq_nb = camt::Number(sequence.to_owned());
    Ok(())
}

struct ParsedBalance {
    indicator: camt::CreditDebitCode,
    date: String,
    currency: String,
    amount: String,
    code: &'static str,
}

impl ParsedBalance {
    fn into_generated(self) -> camt::CashBalance8 {
        camt::CashBalance8 {
            tp: camt::BalanceType13 {
                cd_or_prtry: camt::BalanceType10Choice {
                    cd: Some(camt::ExternalBalanceType1Code(self.code.to_owned())),
                    ..Default::default()
                },
                ..Default::default()
            },
            amt: camt::ActiveOrHistoricCurrencyAndAmount {
                value: self.amount,
                ccy: camt::ActiveOrHistoricCurrencyCode(self.currency),
            },
            cdt_dbt_ind: self.indicator,
            dt: camt::DateAndDateTime2Choice {
                dt: Some(camt::Isodate(self.date)),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

fn parse_balance(tag: &str, value: &str) -> Result<ParsedBalance, Mt940Error> {
    let bytes = value.as_bytes();
    // MT's fixed-width date and currency are ASCII. Check them before slicing
    // the UTF-8 string so malformed multi-byte input returns a typed error.
    if bytes.len() < 11
        || !bytes[1..7].iter().all(u8::is_ascii_digit)
        || !bytes[7..10].iter().all(u8::is_ascii_uppercase)
    {
        return Err(Mt940Error::InvalidField {
            tag: balance_tag(tag),
        });
    }
    let indicator = parse_indicator(bytes[0], balance_tag(tag))?;
    let date = parse_mt_date(&value[1..7], balance_tag(tag))?;
    let currency = &value[7..10];
    let amount = value[10..].replace(',', ".");
    Money::parse(currency, &amount).map_err(|_| Mt940Error::InvalidField {
        tag: balance_tag(tag),
    })?;
    let code = match tag {
        "60F" | "60M" => "OPBD",
        "62F" | "62M" => "CLBD",
        "64" => "CLAV",
        "65" => "FWAV",
        _ => return Err(Mt940Error::InvalidField { tag: "balance" }),
    };
    Ok(ParsedBalance {
        indicator,
        date,
        currency: currency.to_owned(),
        amount,
        code,
    })
}

struct ParsedEntry {
    indicator: camt::CreditDebitCode,
    date: String,
    amount: String,
    currency: String,
    transaction_code: String,
    reference: String,
}

impl ParsedEntry {
    fn into_generated(self, source_index: usize) -> camt::ReportEntry11 {
        camt::ReportEntry11 {
            ntry_ref: camt::Max35Text(format!("MT940-{}", source_index + 1)),
            amt: camt::ActiveOrHistoricCurrencyAndAmount {
                value: self.amount,
                ccy: camt::ActiveOrHistoricCurrencyCode(self.currency),
            },
            cdt_dbt_ind: self.indicator,
            bookg_dt: camt::DateAndDateTime2Choice {
                dt: Some(camt::Isodate(self.date)),
                ..Default::default()
            },
            acct_svcr_ref: camt::Max35Text(self.reference),
            bk_tx_cd: camt::BankTransactionCodeStructure4 {
                prtry: camt::ProprietaryBankTransactionCodeStructure1 {
                    cd: camt::Max35Text(self.transaction_code),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

fn parse_entry(value: &str, currency: &str) -> Result<ParsedEntry, Mt940Error> {
    let bytes = value.as_bytes();
    if bytes.len() < 13 || !bytes[..6].iter().all(u8::is_ascii_digit) {
        return Err(Mt940Error::InvalidField { tag: "61" });
    }
    let date = parse_mt_date(&value[..6], "61")?;
    let mut cursor = 6;
    if bytes
        .get(cursor..cursor + 4)
        .is_some_and(|part| part.iter().all(u8::is_ascii_digit))
    {
        cursor += 4;
    }
    let indicator_byte = *bytes
        .get(cursor)
        .ok_or(Mt940Error::InvalidField { tag: "61" })?;
    let indicator = parse_indicator(indicator_byte, "61")?;
    cursor += 1;
    if bytes.get(cursor) == Some(&b'R') {
        cursor += 1;
    }
    if bytes.get(cursor).is_some_and(u8::is_ascii_uppercase)
        && bytes.get(cursor + 1).is_some_and(u8::is_ascii_digit)
    {
        cursor += 1;
    }
    let amount_start = cursor;
    while bytes
        .get(cursor)
        .is_some_and(|byte| byte.is_ascii_digit() || *byte == b',')
    {
        cursor += 1;
    }
    if cursor == amount_start || bytes.get(cursor).is_none() {
        return Err(Mt940Error::InvalidField { tag: "61" });
    }
    let amount = value[amount_start..cursor].replace(',', ".");
    Money::parse(currency, &amount).map_err(|_| Mt940Error::InvalidField { tag: "61" })?;
    let code_start = cursor;
    cursor = cursor.saturating_add(4);
    let transaction_code = value
        .get(code_start..cursor)
        .filter(|code| code.bytes().all(|byte| byte.is_ascii_alphanumeric()))
        .ok_or(Mt940Error::InvalidField { tag: "61" })?;
    let remainder = value
        .get(cursor..)
        .ok_or(Mt940Error::InvalidField { tag: "61" })?;
    let reference = remainder
        .split_once("//")
        .map(|(_, bank_reference)| bank_reference)
        .unwrap_or(remainder)
        .lines()
        .next()
        .unwrap_or("");
    if reference.len() > 35 {
        return Err(Mt940Error::InvalidField { tag: "61" });
    }
    Ok(ParsedEntry {
        indicator,
        date,
        amount,
        currency: currency.to_owned(),
        transaction_code: transaction_code.to_owned(),
        reference: reference.to_owned(),
    })
}

fn parse_indicator(byte: u8, tag: &'static str) -> Result<camt::CreditDebitCode, Mt940Error> {
    match byte {
        b'C' => Ok(camt::CreditDebitCode::Crdt),
        b'D' => Ok(camt::CreditDebitCode::Dbit),
        _ => Err(Mt940Error::InvalidField { tag }),
    }
}

fn parse_mt_date(value: &str, tag: &'static str) -> Result<String, Mt940Error> {
    if value.len() != 6 || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(Mt940Error::InvalidField { tag });
    }
    let date = format!("20{}-{}-{}", &value[0..2], &value[2..4], &value[4..6]);
    IsoDate::parse(&date).map_err(|_| Mt940Error::InvalidField { tag })?;
    Ok(date)
}

fn balance_tag(tag: &str) -> &'static str {
    match tag {
        "60F" => "60F",
        "60M" => "60M",
        "62F" => "62F",
        "62M" => "62M",
        "64" => "64",
        "65" => "65",
        _ => "balance",
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
pub enum Mt940Error {
    Parse(MtParseError),
    MissingField { tag: &'static str },
    InvalidField { tag: &'static str },
    OrphanInformation,
    Report(MappingReportError),
}

impl fmt::Display for Mt940Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("MT940 conversion failed; inspect the typed error variant")
    }
}

impl std::error::Error for Mt940Error {}
