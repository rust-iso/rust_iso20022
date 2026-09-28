//! Thin command-line adapter over `rust_iso20022` core APIs.

/// Core-result to command-result mappings; no domain logic is implemented here.
pub mod commands;
/// Versioned JSON envelopes and stable process exit codes.
pub mod output;
/// Bounded argument, input, output, and process-exit boundary.
pub mod runner;

pub use output::ExitCode;
