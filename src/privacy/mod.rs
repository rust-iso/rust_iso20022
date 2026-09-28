//! Default-safe redaction for financial-message diagnostics.
//!
//! Core APIs never log message bodies. Adapters should pass values through a
//! [`crate::privacy::RedactionPolicy`] before emitting diagnostics. Full-value diagnostics are
//! unavailable unless the crate feature is enabled *and* the caller performs a
//! separate unsafe acknowledgement at runtime.

use std::sync::Once;

/// Warning applications must surface before enabling unredacted diagnostics.
pub const UNSAFE_FULL_LOGGING_WARNING: &str =
    "unsafe full logging may expose raw financial values and must not be enabled by default";

/// Install a panic hook that never renders the panic payload.
///
/// Panic payloads can contain raw XML or field values supplied by an untrusted
/// caller. WASM adapters use this hook and return their normal typed errors to
/// JavaScript; the hook intentionally emits no payload, location, or backtrace.
pub fn install_redacted_panic_hook() {
    static INSTALL: Once = Once::new();
    INSTALL.call_once(|| std::panic::set_hook(Box::new(|_| {})));
}

/// Classification used to choose a masking strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SensitiveKind {
    Iban,
    Account,
    Bic,
    Name,
    Address,
    TransactionReference,
    Remittance,
    MessageXml,
}

/// Diagnostics redaction policy. The default cannot expose full values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RedactionPolicy {
    allow_full_values: bool,
}

impl RedactionPolicy {
    pub const fn allows_full_values(self) -> bool {
        self.allow_full_values
    }

    pub fn redact(self, kind: SensitiveKind, value: &str) -> String {
        if self.allow_full_values {
            return value.to_owned();
        }
        match kind {
            SensitiveKind::Iban | SensitiveKind::Account | SensitiveKind::Bic => {
                mask_identifier(value)
            }
            SensitiveKind::Name
            | SensitiveKind::Address
            | SensitiveKind::TransactionReference
            | SensitiveKind::Remittance
            | SensitiveKind::MessageXml => "[REDACTED]".to_owned(),
        }
    }

    /// Runtime half of the unsafe logging opt-in. This method only exists when
    /// the `unsafe-full-logging` compile-time feature is explicitly enabled.
    #[cfg(feature = "unsafe-full-logging")]
    pub const fn with_unsafe_full_logging(mut self, _: UnsafeLoggingPermit) -> Self {
        self.allow_full_values = true;
        self
    }
}

fn mask_identifier(value: &str) -> String {
    let characters: Vec<char> = value.chars().collect();
    if characters.len() <= 10 {
        return "[REDACTED]".to_owned();
    }
    let prefix: String = characters[..4].iter().collect();
    let suffix: String = characters[characters.len() - 6..].iter().collect();
    format!("{prefix}{}{suffix}", "*".repeat(characters.len() - 10))
}

/// Explicit runtime acknowledgement required in addition to the feature flag.
#[cfg(feature = "unsafe-full-logging")]
#[derive(Debug, Clone, Copy)]
pub struct UnsafeLoggingPermit(());

#[cfg(feature = "unsafe-full-logging")]
impl UnsafeLoggingPermit {
    /// Prominent warning associated with this unsafe opt-in.
    pub const fn warning(self) -> &'static str {
        UNSAFE_FULL_LOGGING_WARNING
    }

    /// Create an acknowledgement that full financial values may reach logs.
    ///
    /// # Safety
    ///
    /// The caller must ensure the logging destination, retention, access
    /// controls, and applicable privacy obligations permit unredacted data.
    pub const unsafe fn acknowledge_risk() -> Self {
        Self(())
    }
}
