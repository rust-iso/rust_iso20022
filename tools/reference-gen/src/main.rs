//! Deterministically materialize reviewed financial reference snapshots.

use serde_json::json;
use sha2::{Digest, Sha256};
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

type DynError = Box<dyn std::error::Error>;
type IbanRow<'a> = (&'a str, u8, &'a str);
type CurrencyRow<'a> = (&'a str, Option<u8>);

fn main() -> Result<(), DynError> {
    let check = env::args().skip(1).any(|argument| argument == "--check");
    let root = workspace_root()?;
    let reference_dir = root.join("reference-data");
    let country_path = reference_dir.join("iso3166-alpha2.csv");
    let iban_path = reference_dir.join("iban-lengths.csv");
    let currency_path = reference_dir.join("iso4217-active.csv");

    let mut countries = rust_iso3166::ALL_ALPHA2.to_vec();
    countries.sort_unstable();
    countries.dedup();
    let mut country_csv = String::from("alpha2\n");
    for country in &countries {
        writeln!(country_csv, "{country}")?;
    }
    let datasets = vec![
        dataset(
            "iban-lengths",
            "SWIFT IBAN Registry release 102 (June 2026)",
            "https://www.swift.com/standards/data-standards/iban-international-bank-account-number",
            "2026-09-28",
            "Factual registry fields reviewed against the free SWIFT registry; transcription cross-checked against sepa 0.8.0 (MIT OR Apache-2.0).",
            "reference-data/iban-lengths.csv",
            &fs::read(&iban_path)?,
        )?,
        dataset(
            "iso-3166-alpha2",
            "rust_iso3166 0.2.0 dataset",
            "https://github.com/rust-iso/rust_iso3166",
            "2026-09-28",
            "Snapshot generated from rust_iso3166 0.2.0, licensed Apache-2.0.",
            "reference-data/iso3166-alpha2.csv",
            country_csv.as_bytes(),
        )?,
        dataset(
            "iso-4217-active",
            "SIX ISO 4217 List One, published 2026-09-17",
            "https://www.six-group.com/dam/download/financial-information/data-center/iso-currrency/lists/list-one.xml",
            "2026-09-28",
            "ISO permits free use of ISO 4217 codes; factual code/minor-unit fields reviewed from the official SIX List One.",
            "reference-data/iso4217-active.csv",
            &fs::read(&currency_path)?,
        )?,
    ];
    let manifest = serde_json::to_string_pretty(&json!({
        "format_version": 1,
        "generated_by": "iso20022-reference-gen 0.0.0",
        "datasets": datasets,
    }))? + "\n";
    let manifest_path = reference_dir.join("manifest.json");
    let iban_source = fs::read_to_string(&iban_path)?;
    let currency_source = fs::read_to_string(&currency_path)?;
    let iban = parse_iban(&iban_source)?;
    let currencies = parse_currencies(&currency_source)?;
    let generated = render_rust(
        &hex(Sha256::digest(manifest.as_bytes())),
        &iban,
        &currencies,
        &countries,
    );
    let generated_path = root.join("src/helpers/reference_data.rs");
    if check {
        check_equal(&country_path, country_csv.as_bytes())?;
        check_equal(&manifest_path, manifest.as_bytes())?;
        check_equal(&generated_path, generated.as_bytes())?;
    } else {
        fs::write(&country_path, country_csv)?;
        fs::write(&manifest_path, manifest)?;
        fs::write(generated_path, generated)?;
    }
    Ok(())
}

fn workspace_root() -> Result<PathBuf, DynError> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    Ok(manifest
        .parent()
        .and_then(Path::parent)
        .ok_or("invalid tool path")?
        .to_owned())
}

fn dataset(
    id: &str,
    release: &str,
    source: &str,
    retrieved_at: &str,
    license_decision: &str,
    path: &str,
    bytes: &[u8],
) -> Result<serde_json::Value, DynError> {
    Ok(json!({
        "id": id,
        "reviewed": true,
        "release": release,
        "source": source,
        "retrieved_at": retrieved_at,
        "license_decision": license_decision,
        "path": path,
        "sha256": hex(Sha256::digest(bytes)),
    }))
}

fn check_equal(path: &Path, expected: &[u8]) -> Result<(), DynError> {
    if fs::read(path).ok().as_deref() != Some(expected) {
        return Err(format!("stale generated reference data: {}", path.display()).into());
    }
    Ok(())
}

fn parse_iban(input: &str) -> Result<Vec<IbanRow<'_>>, DynError> {
    let rows: Vec<_> = input
        .lines()
        .skip(1)
        .filter(|line| !line.is_empty())
        .map(|line| -> Result<_, DynError> {
            let mut fields = line.split(',');
            let country = fields.next().ok_or("missing IBAN country")?;
            let length = fields.next().ok_or("missing IBAN length")?.parse()?;
            let pattern = fields.next().ok_or("missing BBAN pattern")?;
            if fields.next().is_some() || pattern.len() + 4 != usize::from(length) {
                return Err("invalid IBAN snapshot row".into());
            }
            Ok((country, length, pattern))
        })
        .collect::<Result<_, _>>()?;
    ensure_sorted_unique(rows.iter().map(|(country, _, _)| *country))?;
    Ok(rows)
}

fn parse_currencies(input: &str) -> Result<Vec<CurrencyRow<'_>>, DynError> {
    let rows: Vec<_> = input
        .lines()
        .skip(1)
        .filter(|line| !line.is_empty())
        .map(|line| -> Result<_, DynError> {
            let mut fields = line.split(',');
            let code = fields.next().ok_or("missing currency code")?;
            let units = match fields.next().ok_or("missing minor units")? {
                "N.A." => None,
                value => Some(value.parse()?),
            };
            if fields.next().is_some() {
                return Err("invalid currency snapshot row".into());
            }
            Ok((code, units))
        })
        .collect::<Result<_, _>>()?;
    ensure_sorted_unique(rows.iter().map(|(code, _)| *code))?;
    Ok(rows)
}

fn ensure_sorted_unique<'a>(values: impl IntoIterator<Item = &'a str>) -> Result<(), DynError> {
    let mut previous = None;
    for value in values {
        if previous.is_some_and(|candidate| candidate >= value) {
            return Err("reference rows must be sorted and unique".into());
        }
        previous = Some(value);
    }
    Ok(())
}

fn render_rust(
    manifest_sha: &str,
    iban: &[(&str, u8, &str)],
    currencies: &[(&str, Option<u8>)],
    countries: &[&str],
) -> String {
    let mut output = String::from("// @generated by iso20022-reference-gen; do not edit.\n\n");
    writeln!(
        output,
        "pub const REFERENCE_DATA_MANIFEST_SHA256: &str =\n    {manifest_sha:?};"
    )
    .unwrap();
    output.push_str("\n#[derive(Debug, Clone, Copy)]\npub(crate) struct IbanFormat {\n    pub country: &'static str,\n    pub total_length: u8,\n    pub bban_pattern: &'static str,\n}\n\n");
    output.push_str("pub(crate) const IBAN_FORMATS: &[IbanFormat] = &[\n");
    for (country, length, pattern) in iban {
        writeln!(output, "    IbanFormat {{\n        country: {country:?},\n        total_length: {length},\n        bban_pattern: {pattern:?},\n    }},").unwrap();
    }
    output.push_str("];\n\n#[derive(Debug, Clone, Copy)]\npub(crate) struct CurrencyRecord {\n    pub code: &'static str,\n    pub minor_units: Option<u8>,\n}\n\npub(crate) const CURRENCIES: &[CurrencyRecord] = &[\n");
    for (code, units) in currencies {
        writeln!(output, "    CurrencyRecord {{\n        code: {code:?},\n        minor_units: {units:?},\n    }},").unwrap();
    }
    output.push_str("];\n\n#[rustfmt::skip]\npub(crate) const COUNTRY_CODES: &[&str] = &[\n");
    for code in countries {
        writeln!(output, "    {code:?},").unwrap();
    }
    output.push_str("];\n");
    output
}

fn hex(bytes: impl AsRef<[u8]>) -> String {
    bytes
        .as_ref()
        .iter()
        .fold(String::new(), |mut output, byte| {
            write!(output, "{byte:02x}").expect("writing to String cannot fail");
            output
        })
}
