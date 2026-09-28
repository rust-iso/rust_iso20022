use core::fmt;

/// Extensible profile scheme identifier such as `cbpr-plus` or `sepa-sct`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ProfileScheme(String);

impl ProfileScheme {
    pub fn parse(value: &str) -> Result<Self, ProfileSchemeParseError> {
        if value.len() < 2
            || value.len() > 32
            || value.starts_with('-')
            || value.ends_with('-')
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        {
            return Err(ProfileSchemeParseError);
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProfileScheme {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfileSchemeParseError;

impl fmt::Display for ProfileSchemeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("profile scheme must be a lowercase kebab-case identifier")
    }
}

impl std::error::Error for ProfileSchemeParseError {}

/// Calendar date used to resolve an immutable profile release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ProfileDate {
    year: u16,
    month: u8,
    day: u8,
}

impl ProfileDate {
    pub fn parse(value: &str) -> Result<Self, ProfileDateParseError> {
        if value.len() != 10 || value.as_bytes()[4] != b'-' || value.as_bytes()[7] != b'-' {
            return Err(ProfileDateParseError);
        }
        let year = parse_digits(&value.as_bytes()[0..4]).ok_or(ProfileDateParseError)?;
        let month = parse_digits(&value.as_bytes()[5..7]).ok_or(ProfileDateParseError)? as u8;
        let day = parse_digits(&value.as_bytes()[8..10]).ok_or(ProfileDateParseError)? as u8;
        let max_day = days_in_month(year, month).ok_or(ProfileDateParseError)?;
        if day == 0 || day > max_day {
            return Err(ProfileDateParseError);
        }
        Ok(Self { year, month, day })
    }
}

impl fmt::Display for ProfileDate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:04}-{:02}-{:02}",
            self.year, self.month, self.day
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfileDateParseError;

impl fmt::Display for ProfileDateParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("profile effective date must be a valid YYYY-MM-DD date")
    }
}

impl std::error::Error for ProfileDateParseError {}

/// Complete immutable identity required for L3 profile validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ProfileReleaseKey {
    scheme: ProfileScheme,
    scheme_release: String,
    guideline_edition: String,
    effective_from: ProfileDate,
    effective_until: ProfileDate,
    source_digests: Vec<String>,
}

impl ProfileReleaseKey {
    /// Parse the stable six-component wire form:
    /// `scheme|release|guideline|from|until|digest[,digest]`.
    pub fn parse(value: &str) -> Result<Self, ReleaseKeyParseError> {
        let parts: Vec<_> = value.split('|').collect();
        if parts.len() != 6 {
            return Err(ReleaseKeyParseError::Incomplete);
        }
        let scheme =
            ProfileScheme::parse(parts[0]).map_err(|_| ReleaseKeyParseError::InvalidScheme)?;
        validate_label(parts[1]).map_err(|_| ReleaseKeyParseError::InvalidRelease)?;
        validate_label(parts[2]).map_err(|_| ReleaseKeyParseError::InvalidGuideline)?;
        let effective_from =
            ProfileDate::parse(parts[3]).map_err(|_| ReleaseKeyParseError::InvalidDate)?;
        let effective_until =
            ProfileDate::parse(parts[4]).map_err(|_| ReleaseKeyParseError::InvalidDate)?;
        if effective_until < effective_from {
            return Err(ReleaseKeyParseError::InvalidRange);
        }
        let source_digests: Vec<_> = parts[5].split(',').map(str::to_owned).collect();
        if source_digests.is_empty() || source_digests.iter().any(|digest| !valid_sha256(digest)) {
            return Err(ReleaseKeyParseError::InvalidDigest);
        }
        Ok(Self {
            scheme,
            scheme_release: parts[1].to_owned(),
            guideline_edition: parts[2].to_owned(),
            effective_from,
            effective_until,
            source_digests,
        })
    }

    pub fn scheme(&self) -> &ProfileScheme {
        &self.scheme
    }

    pub fn scheme_release(&self) -> &str {
        &self.scheme_release
    }

    pub fn guideline_edition(&self) -> &str {
        &self.guideline_edition
    }

    pub const fn effective_from(&self) -> ProfileDate {
        self.effective_from
    }

    pub const fn effective_until(&self) -> ProfileDate {
        self.effective_until
    }

    pub fn source_digests(&self) -> &[String] {
        &self.source_digests
    }

    pub fn rule_set_version(&self) -> String {
        format!("{}/{}", self.scheme_release, self.guideline_edition)
    }

    pub fn effective_on(&self, date: ProfileDate) -> bool {
        self.effective_from <= date && date <= self.effective_until
    }
}

impl fmt::Display for ProfileReleaseKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}|{}|{}|{}|{}|{}",
            self.scheme,
            self.scheme_release,
            self.guideline_edition,
            self.effective_from,
            self.effective_until,
            self.source_digests.join(",")
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseKeyParseError {
    Incomplete,
    InvalidScheme,
    InvalidRelease,
    InvalidGuideline,
    InvalidDate,
    InvalidRange,
    InvalidDigest,
}

impl fmt::Display for ReleaseKeyParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Incomplete => "profile release key must contain all six components",
            Self::InvalidScheme => "invalid profile scheme",
            Self::InvalidRelease => "invalid profile scheme release",
            Self::InvalidGuideline => "invalid profile guideline edition",
            Self::InvalidDate => "invalid profile effective date",
            Self::InvalidRange => "profile effective range ends before it starts",
            Self::InvalidDigest => "profile source digest must be lowercase SHA-256",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for ReleaseKeyParseError {}

fn validate_label(value: &str) -> Result<(), ()> {
    if value.is_empty()
        || value.len() > 64
        || value
            .bytes()
            .any(|byte| byte.is_ascii_control() || matches!(byte, b'|' | b',' | b'/'))
    {
        return Err(());
    }
    Ok(())
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn parse_digits(bytes: &[u8]) -> Option<u16> {
    bytes.iter().try_fold(0_u16, |number, byte| {
        byte.is_ascii_digit()
            .then(|| number * 10 + u16::from(byte - b'0'))
    })
}

fn days_in_month(year: u16, month: u8) -> Option<u8> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) => Some(29),
        2 => Some(28),
        _ => None,
    }
}
