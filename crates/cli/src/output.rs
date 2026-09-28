//! Stable CLI output DTOs, JSON envelope, and process exit codes.

use serde::Serialize;

/// Stable process outcomes shared by all CLI commands.
///
/// Some variants are reserved before their commands land so numeric meanings
/// cannot drift between work packages.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ExitCode {
    Success = 0,
    Usage = 2,
    ParseFailure = 3,
    ValidationFailure = 4,
    Unavailable = 5,
    InternalFailure = 10,
}

#[derive(Debug, Serialize)]
pub struct Envelope<T> {
    pub schema_version: u8,
    pub command: &'static str,
    pub status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorBody>,
}

impl<T> Envelope<T> {
    pub fn success(command: &'static str, data: T) -> Self {
        Self {
            schema_version: 1,
            command,
            status: "success",
            data: Some(data),
            error: None,
        }
    }

    pub fn error(command: &'static str, code: &'static str, message: &'static str) -> Self {
        Self {
            schema_version: 1,
            command,
            status: "error",
            data: None,
            error: Some(ErrorBody { code, message }),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub code: &'static str,
    pub message: &'static str,
}

#[derive(Debug, Serialize)]
pub struct DetectData<'a> {
    pub message_id: &'a str,
    pub business_area: &'a str,
    pub family: &'a str,
    pub version: &'a str,
    pub namespace: &'a str,
    pub root_element: &'a str,
}
