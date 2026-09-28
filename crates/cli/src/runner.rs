//! Shared process boundary compiled by the standalone CLI and legacy shim.

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use rust_iso20022::{DetectionError, ParseLimits, XmlLimit, detect_message};

use crate::commands::{self, CommandError};
use crate::output::{DetectData, Envelope, ExitCode};

const MAX_INPUT_BYTES: usize = ParseLimits::DEFAULT.max_input_bytes;

pub fn run(args: impl Iterator<Item = String>) -> ExitCode {
    run_inner(args).unwrap_or_else(|code| code)
}

fn run_inner(args: impl Iterator<Item = String>) -> Result<ExitCode, ExitCode> {
    let args: Vec<_> = args.collect();
    let Some(command) = args.first().map(String::as_str) else {
        return usage_error();
    };
    if command == "validate" {
        return validate_arguments(&args[1..]);
    }
    let (json, positional) = parse_arguments(&args[1..])?;
    match command {
        "detect" if positional.len() == 1 => detect_command(json, positional[0]),
        "inspect" if positional.len() == 1 => inspect_command(json, positional[0]),
        "to-json" if positional.len() == 1 => to_json_command(json, positional[0]),
        "to-xml" if positional.len() == 2 => to_xml_command(json, positional[0], positional[1]),
        "catalog" if positional.len() == 1 => catalog_command(json, positional[0]),
        "versions" if positional.len() == 1 => versions_command(json, positional[0]),
        "explain" if positional.len() == 1 => explain_command(json, positional[0]),
        "compare" if positional.len() == 2 => compare_command(json, positional[0], positional[1]),
        _ => usage_error(),
    }
}

fn validate_arguments(arguments: &[String]) -> Result<ExitCode, ExitCode> {
    let mut json = false;
    let mut layer = "l2";
    let mut profile = None;
    let mut input = None;
    let mut index = 0;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--json" if !json => json = true,
            "--layer" if index + 1 < arguments.len() => {
                index += 1;
                layer = arguments[index].as_str();
            }
            "--profile" if index + 1 < arguments.len() => {
                index += 1;
                profile = Some(arguments[index].as_str());
            }
            value if !value.starts_with('-') || value == "-" => {
                if input.replace(value).is_some() {
                    return usage_error();
                }
            }
            _ => return usage_error(),
        }
        index += 1;
    }
    let Some(input) = input else {
        return usage_error();
    };
    if let Some(profile) = profile {
        #[cfg(feature = "profiles")]
        if rust_iso20022::profiles::ProfileReleaseKey::parse(profile).is_err() {
            return usage_error();
        }
        #[cfg(not(feature = "profiles"))]
        let _ = profile;
        return emit_error(
            "validate",
            json,
            "profile_unavailable",
            "the exact requested profile release is not installed",
            ExitCode::Unavailable,
        );
    }
    match layer {
        "l2" | "iso" | "iso-semantic" => validate_command(json, input),
        "l1" | "syntax" | "schema" => emit_error(
            "validate",
            json,
            "layer_unavailable",
            "full offline XSD validation is unavailable",
            ExitCode::Unavailable,
        ),
        "l3" | "profile" => emit_error(
            "validate",
            json,
            "profile_unavailable",
            "an exact profile release key is required and must be installed",
            ExitCode::Unavailable,
        ),
        _ => usage_error(),
    }
}

fn parse_arguments(arguments: &[String]) -> Result<(bool, Vec<&str>), ExitCode> {
    let mut json = false;
    let mut positional = Vec::new();
    for argument in arguments {
        if argument == "--json" {
            if json {
                return usage_error();
            }
            json = true;
        } else {
            positional.push(argument.as_str());
        }
    }
    Ok((json, positional))
}

fn detect_command(json: bool, input: &str) -> Result<ExitCode, ExitCode> {
    let xml = read_input("detect", json, input)?;
    match detect_message(&xml, ParseLimits::DEFAULT) {
        Ok(detected) => {
            if json {
                let identifier = detected.message_id();
                emit_success(
                    "detect",
                    DetectData {
                        message_id: identifier.as_str(),
                        business_area: identifier.business_area(),
                        family: identifier.family(),
                        version: identifier.version().as_str(),
                        namespace: detected.namespace(),
                        root_element: detected.root_element(),
                    },
                )?;
            } else {
                println!("{}", detected.message_id());
            }
            Ok(ExitCode::Success)
        }
        Err(error) => {
            let (code, message) = classify_detection_error(&error);
            emit_error("detect", json, code, message, ExitCode::ParseFailure)
        }
    }
}

fn inspect_command(json: bool, input: &str) -> Result<ExitCode, ExitCode> {
    let xml = read_input("inspect", json, input)?;
    match commands::inspect(&xml) {
        Ok(data) => {
            if json {
                emit_success("inspect", data)?;
            } else {
                println!("message_id: {}", data.message_id);
                println!("business_area: {}", data.business_area);
                println!("family: {}", data.family);
                println!("version: {}", data.version);
                println!("namespace: {}", data.namespace);
                println!("root_element: {}", data.root_element);
                println!("description: {}", data.description);
                println!("generated_module: {}", data.generated_module);
                println!("required_feature: {}", data.required_feature);
                println!("field_count: {}", data.field_count);
            }
            Ok(ExitCode::Success)
        }
        Err(error) => emit_command_error("inspect", json, error),
    }
}

fn to_json_command(json_envelope: bool, input: &str) -> Result<ExitCode, ExitCode> {
    let xml = read_input("to-json", json_envelope, input)?;
    match commands::to_json(&xml) {
        Ok(data) => {
            if json_envelope {
                emit_success("to-json", data)?;
            } else {
                println!(
                    "{}",
                    serde_json::to_string(&data.document).map_err(|_| ExitCode::InternalFailure)?
                );
            }
            Ok(ExitCode::Success)
        }
        Err(error) => emit_command_error("to-json", json_envelope, error),
    }
}

fn to_xml_command(
    json_envelope: bool,
    message_id: &str,
    input: &str,
) -> Result<ExitCode, ExitCode> {
    let json = read_input("to-xml", json_envelope, input)?;
    match commands::to_xml(message_id, &json) {
        Ok(data) => {
            if json_envelope {
                emit_success("to-xml", data)?;
            } else {
                println!("{}", data.document);
            }
            Ok(ExitCode::Success)
        }
        Err(error) => emit_command_error("to-xml", json_envelope, error),
    }
}

fn validate_command(json: bool, input: &str) -> Result<ExitCode, ExitCode> {
    let xml = read_input("validate", json, input)?;
    match commands::validate(&xml) {
        Ok(data) => {
            let valid = data.report.valid();
            if json {
                emit_success("validate", data)?;
            } else if valid {
                println!("valid according to the implemented ISO semantic rules");
            } else {
                for issue in data.report.issues() {
                    println!("{} {}", issue.rule_id, issue.path.as_str());
                }
            }
            if valid {
                Ok(ExitCode::Success)
            } else {
                Err(ExitCode::ValidationFailure)
            }
        }
        Err(error) => emit_command_error("validate", json, error),
    }
}

fn catalog_command(json: bool, family: &str) -> Result<ExitCode, ExitCode> {
    let data = commands::catalog(family);
    if json {
        emit_success("catalog", data)?;
    } else {
        for message in data.messages {
            println!("{}", message.message_id);
        }
    }
    Ok(ExitCode::Success)
}

fn versions_command(json: bool, family: &str) -> Result<ExitCode, ExitCode> {
    let data = commands::versions(family);
    if json {
        emit_success("versions", data)?;
    } else {
        for version in data.versions {
            println!("{version}");
        }
    }
    Ok(ExitCode::Success)
}

fn explain_command(json: bool, rule_id: &str) -> Result<ExitCode, ExitCode> {
    match commands::explain(rule_id) {
        Ok(explanation) => {
            if json {
                emit_success("explain", &explanation)?;
            } else {
                let descriptor = explanation.descriptor;
                println!("rule_id: {}", descriptor.id);
                println!("layer: {:?}", descriptor.layer);
                println!("title: {}", descriptor.title);
                println!("reason: {}", descriptor.reason);
                println!("source: {}", descriptor.source.unwrap_or("none"));
                println!("profile: {}", descriptor.profile.unwrap_or("none"));
                println!(
                    "profile_version: {}",
                    descriptor.profile_version.unwrap_or("none")
                );
                println!(
                    "affected_messages: {}",
                    descriptor.affected_messages.join(",")
                );
                println!("field_paths: {}", explanation.field_paths.join(","));
            }
            Ok(ExitCode::Success)
        }
        Err(error) => emit_command_error("explain", json, error),
    }
}

fn compare_command(json: bool, from: &str, to: &str) -> Result<ExitCode, ExitCode> {
    match commands::compare(from, to) {
        Ok(diff) => {
            if json {
                emit_success("compare", diff)?;
            } else {
                println!("{} -> {}", diff.from.identity, diff.to.identity);
                println!("from_schema_sha256: {}", diff.from.schema_sha256);
                println!("to_schema_sha256: {}", diff.to.schema_sha256);
                for change in diff.changes {
                    println!(
                        "{:?} {} {} -> {}",
                        change.kind,
                        change.path,
                        change.before.as_deref().unwrap_or("none"),
                        change.after.as_deref().unwrap_or("none")
                    );
                }
            }
            Ok(ExitCode::Success)
        }
        Err(error) => emit_command_error("compare", json, error),
    }
}

fn emit_success<T: serde::Serialize>(command: &'static str, data: T) -> Result<(), ExitCode> {
    println!(
        "{}",
        serde_json::to_string(&Envelope::success(command, data))
            .map_err(|_| ExitCode::InternalFailure)?
    );
    Ok(())
}

fn emit_command_error(
    command: &'static str,
    json: bool,
    error: CommandError,
) -> Result<ExitCode, ExitCode> {
    emit_error(command, json, error.code, error.message, error.exit)
}

fn read_input(command: &'static str, json: bool, input: &str) -> Result<String, ExitCode> {
    match read_bounded(input) {
        Ok(value) => Ok(value),
        Err(InputError::Limit) => emit_error(
            command,
            json,
            "input_limit_exceeded",
            "input exceeds the configured limit",
            ExitCode::ParseFailure,
        ),
        Err(InputError::Read) => emit_error(
            command,
            json,
            "input_read_failed",
            "unable to read UTF-8 input",
            ExitCode::ParseFailure,
        ),
    }
}

fn usage_error<T>() -> Result<T, ExitCode> {
    eprintln!(
        "usage: iso20022 <detect|inspect|to-json|validate> [options] <FILE|-> | \
         iso20022 to-xml <MESSAGE_ID> [--json] <FILE|-> | \
         iso20022 <catalog|versions> [--json] <FAMILY> | \
         iso20022 explain [--json] <RULE_ID> | \
         iso20022 compare [--json] <FROM_MESSAGE_ID> <TO_MESSAGE_ID>"
    );
    eprintln!(
        "validation covers only implemented rules and does not guarantee bank/network acceptance, certification, onboarding, or legal compliance"
    );
    Err(ExitCode::Usage)
}

fn emit_error<T>(
    command: &'static str,
    json: bool,
    code: &'static str,
    message: &'static str,
    exit: ExitCode,
) -> Result<T, ExitCode> {
    if json {
        let envelope: Envelope<()> = Envelope::error(command, code, message);
        println!(
            "{}",
            serde_json::to_string(&envelope).map_err(|_| ExitCode::InternalFailure)?
        );
    }
    eprintln!("{message}");
    Err(exit)
}

fn classify_detection_error(error: &DetectionError) -> (&'static str, &'static str) {
    match error {
        DetectionError::Xml(rust_iso20022::XmlReadError::LimitExceeded {
            limit: XmlLimit::InputBytes,
            ..
        }) => (
            "input_limit_exceeded",
            "XML input exceeds the configured limit",
        ),
        DetectionError::Xml(_) => ("malformed_xml", "XML input is malformed or forbidden"),
        DetectionError::UnknownMessage => ("unknown_message", "unknown ISO 20022 message"),
        DetectionError::RootMismatch { .. } => (
            "root_mismatch",
            "message namespace is attached to an unexpected root element",
        ),
    }
}

enum InputError {
    Limit,
    Read,
}

fn read_bounded(path: &str) -> Result<String, InputError> {
    if path == "-" {
        return read_from(io::stdin().lock());
    }
    let path = Path::new(path);
    let metadata = path.metadata().map_err(|_| InputError::Read)?;
    if metadata.len() > MAX_INPUT_BYTES as u64 {
        return Err(InputError::Limit);
    }
    read_from(File::open(path).map_err(|_| InputError::Read)?)
}

fn read_from(reader: impl Read) -> Result<String, InputError> {
    let mut bytes = Vec::new();
    reader
        .take(MAX_INPUT_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| InputError::Read)?;
    if bytes.len() > MAX_INPUT_BYTES {
        return Err(InputError::Limit);
    }
    String::from_utf8(bytes).map_err(|_| InputError::Read)
}
