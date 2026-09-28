use super::{ValueConstraint, ValueError};

/// Generic account identifier with ISO 20022 text bounds.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountIdentifier(String);

impl AccountIdentifier {
    pub fn parse(value: &str) -> Result<Self, ValueError> {
        validate_component(value, 34, "ISO20022-L2-ACCOUNT-ID")?;
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Clearing scheme plus member identifier. This validates bounded syntax, not
/// membership in a network directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClearingIdentifier {
    scheme: String,
    member_id: String,
}

impl ClearingIdentifier {
    pub fn parse(scheme: &str, member_id: &str) -> Result<Self, ValueError> {
        validate_component(scheme, 35, "ISO20022-L2-CLEARING-SCHEME")?;
        validate_component(member_id, 35, "ISO20022-L2-CLEARING-MEMBER")?;
        Ok(Self {
            scheme: scheme.to_owned(),
            member_id: member_id.to_owned(),
        })
    }

    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    pub fn member_id(&self) -> &str {
        &self.member_id
    }
}

fn validate_component(value: &str, maximum: usize, rule: &'static str) -> Result<(), ValueError> {
    if value.is_empty() || value.len() > maximum {
        return Err(ValueError::new(rule, ValueConstraint::Length));
    }
    if !value.bytes().all(|byte| {
        byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'/' | b'-' | b'?' | b':' | b'(' | b')' | b'.' | b',' | b'\'' | b'+' | b' '
            )
    }) {
        return Err(ValueError::new(rule, ValueConstraint::CharacterSet));
    }
    Ok(())
}
