//! Local-only, stdio-only MCP adapter over `rust_iso20022` core APIs.
//!
//! Validation results cover only implemented rules; they do not guarantee bank
//! or network acceptance, certification, onboarding, or legal compliance.

/// Protocol DTOs with bounded in-memory message inputs and typed schemas.
pub mod dto;
/// Seven thin tools that delegate all domain behavior to the core SDK.
pub mod tools;

pub use tools::Iso20022Mcp;
