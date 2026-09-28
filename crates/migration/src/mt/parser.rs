use core::fmt;

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MtBlock {
    id: String,
    content: String,
    span: SourceSpan,
}

impl MtBlock {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub const fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MtField {
    index: usize,
    tag: String,
    occurrence: usize,
    value: String,
    span: SourceSpan,
}

impl MtField {
    pub const fn index(&self) -> usize {
        self.index
    }
    pub fn tag(&self) -> &str {
        &self.tag
    }
    pub const fn occurrence(&self) -> usize {
        self.occurrence
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub const fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MtDocument {
    blocks: Vec<MtBlock>,
    fields: Vec<MtField>,
}

impl MtDocument {
    pub fn parse(input: &str) -> Result<Self, MtParseError> {
        Self::parse_with_limits(input, MtParseLimits::DEFAULT)
    }

    pub fn parse_with_limits(input: &str, limits: MtParseLimits) -> Result<Self, MtParseError> {
        parse(input, limits)
    }

    pub fn blocks(&self) -> &[MtBlock] {
        &self.blocks
    }
    pub fn fields(&self) -> &[MtField] {
        &self.fields
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MtParseLimits {
    pub max_input_bytes: usize,
    pub max_block_depth: usize,
    pub max_blocks: usize,
    pub max_fields: usize,
    pub max_field_bytes: usize,
    pub max_continuation_lines: usize,
}

impl MtParseLimits {
    pub const DEFAULT: Self = Self {
        max_input_bytes: 1024 * 1024,
        max_block_depth: 8,
        max_blocks: 16,
        max_fields: 512,
        max_field_bytes: 64 * 1024,
        max_continuation_lines: 256,
    };
}

impl Default for MtParseLimits {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MtLimit {
    InputBytes,
    BlockDepth,
    BlockCount,
    FieldCount,
    FieldBytes,
    ContinuationLines,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MtParseErrorKind {
    Empty,
    InvalidStructure,
    InvalidBlock,
    InvalidBlockSequence,
    DuplicateBlock,
    MissingTextBlock,
    InvalidField,
    InvalidUtf8Boundary,
    LimitExceeded(MtLimit),
}

#[derive(Clone, PartialEq, Eq, Serialize)]
pub struct MtParseError {
    kind: MtParseErrorKind,
    offset: Option<usize>,
}

impl MtParseError {
    pub const fn kind(&self) -> MtParseErrorKind {
        self.kind
    }
    pub const fn offset(&self) -> Option<usize> {
        self.offset
    }
}

impl fmt::Debug for MtParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MtParseError")
            .field("kind", &self.kind)
            .field("offset", &self.offset)
            .finish()
    }
}

impl fmt::Display for MtParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "SWIFT MT input is invalid ({:?})", self.kind)
    }
}

impl std::error::Error for MtParseError {}

fn error(kind: MtParseErrorKind, offset: Option<usize>) -> MtParseError {
    MtParseError { kind, offset }
}

fn parse(input: &str, limits: MtParseLimits) -> Result<MtDocument, MtParseError> {
    if input.is_empty() {
        return Err(error(MtParseErrorKind::Empty, None));
    }
    if input.len() > limits.max_input_bytes {
        return Err(error(
            MtParseErrorKind::LimitExceeded(MtLimit::InputBytes),
            None,
        ));
    }
    let bytes = input.as_bytes();
    let mut blocks = Vec::new();
    let mut cursor = 0;
    let mut previous_rank = 0;
    let mut seen = std::collections::BTreeSet::new();
    while cursor < bytes.len() {
        if bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
            continue;
        }
        if bytes[cursor] != b'{' {
            return Err(error(MtParseErrorKind::InvalidStructure, Some(cursor)));
        }
        let start = cursor;
        let colon = input[cursor + 1..]
            .find(':')
            .map(|at| cursor + 1 + at)
            .ok_or_else(|| error(MtParseErrorKind::InvalidBlock, Some(cursor)))?;
        let id = &input[cursor + 1..colon];
        if !valid_block_id(id) {
            return Err(error(MtParseErrorKind::InvalidBlock, Some(start)));
        }
        if !seen.insert(id.to_owned()) {
            return Err(error(MtParseErrorKind::DuplicateBlock, Some(start)));
        }
        let rank = block_rank(id);
        if rank < previous_rank {
            return Err(error(MtParseErrorKind::InvalidBlockSequence, Some(start)));
        }
        previous_rank = rank;
        let mut depth = 1usize;
        let mut end = None;
        for (relative, byte) in bytes[colon + 1..].iter().enumerate() {
            match byte {
                b'{' => {
                    depth += 1;
                    if depth > limits.max_block_depth {
                        return Err(error(
                            MtParseErrorKind::LimitExceeded(MtLimit::BlockDepth),
                            Some(colon + 1 + relative),
                        ));
                    }
                }
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(colon + 1 + relative);
                        break;
                    }
                }
                _ => {}
            }
        }
        let end = end.ok_or_else(|| error(MtParseErrorKind::InvalidStructure, Some(start)))?;
        blocks.push(MtBlock {
            id: id.to_owned(),
            content: input[colon + 1..end].to_owned(),
            span: SourceSpan {
                start,
                end: end + 1,
            },
        });
        if blocks.len() > limits.max_blocks {
            return Err(error(
                MtParseErrorKind::LimitExceeded(MtLimit::BlockCount),
                Some(start),
            ));
        }
        cursor = end + 1;
    }
    let text = blocks
        .iter()
        .find(|block| block.id == "4")
        .ok_or_else(|| error(MtParseErrorKind::MissingTextBlock, None))?;
    let fields = parse_fields(input, text, limits)?;
    Ok(MtDocument { blocks, fields })
}

fn parse_fields(
    input: &str,
    block: &MtBlock,
    limits: MtParseLimits,
) -> Result<Vec<MtField>, MtParseError> {
    let content_start = block.span.start + 3;
    let mut fields = Vec::new();
    let mut current: Option<(String, usize, usize, Vec<&str>)> = None;
    let mut occurrences = std::collections::BTreeMap::<String, usize>::new();
    let mut offset = 0usize;
    for segment in block.content.split_inclusive('\n') {
        let raw = segment
            .strip_suffix('\n')
            .unwrap_or(segment)
            .strip_suffix('\r')
            .unwrap_or(segment.strip_suffix('\n').unwrap_or(segment));
        let line_start = content_start + offset;
        offset += segment.len();
        if raw.is_empty() {
            continue;
        }
        if raw == "-" {
            break;
        }
        if let Some(rest) = raw.strip_prefix(':') {
            if let Some((tag, occurrence, start, lines)) = current.take() {
                push_field(
                    &mut fields,
                    tag,
                    occurrence,
                    start,
                    line_start,
                    lines,
                    limits,
                )?;
            }
            let separator = rest
                .find(':')
                .ok_or_else(|| error(MtParseErrorKind::InvalidField, Some(line_start)))?;
            let tag = &rest[..separator];
            if !valid_tag(tag) {
                return Err(error(MtParseErrorKind::InvalidField, Some(line_start)));
            }
            let occurrence = occurrences.entry(tag.to_owned()).or_default();
            *occurrence += 1;
            current = Some((
                tag.to_owned(),
                *occurrence,
                line_start,
                vec![&rest[separator + 1..]],
            ));
        } else if let Some((_, _, _, lines)) = current.as_mut() {
            lines.push(raw);
            if lines.len() > limits.max_continuation_lines {
                return Err(error(
                    MtParseErrorKind::LimitExceeded(MtLimit::ContinuationLines),
                    Some(line_start),
                ));
            }
        } else {
            return Err(error(MtParseErrorKind::InvalidField, Some(line_start)));
        }
    }
    if let Some((tag, occurrence, start, lines)) = current.take() {
        push_field(
            &mut fields,
            tag,
            occurrence,
            start,
            block.span.end - 1,
            lines,
            limits,
        )?;
    }
    if fields.is_empty() {
        return Err(error(MtParseErrorKind::InvalidField, Some(content_start)));
    }
    if !input.is_char_boundary(block.span.start) {
        return Err(error(
            MtParseErrorKind::InvalidUtf8Boundary,
            Some(block.span.start),
        ));
    }
    Ok(fields)
}

fn push_field(
    fields: &mut Vec<MtField>,
    tag: String,
    occurrence: usize,
    start: usize,
    end: usize,
    lines: Vec<&str>,
    limits: MtParseLimits,
) -> Result<(), MtParseError> {
    let value = lines.join("\n");
    if value.len() > limits.max_field_bytes {
        return Err(error(
            MtParseErrorKind::LimitExceeded(MtLimit::FieldBytes),
            Some(start),
        ));
    }
    if fields.len() >= limits.max_fields {
        return Err(error(
            MtParseErrorKind::LimitExceeded(MtLimit::FieldCount),
            Some(start),
        ));
    }
    fields.push(MtField {
        index: fields.len(),
        tag,
        occurrence,
        value,
        span: SourceSpan { start, end },
    });
    Ok(())
}

fn valid_block_id(id: &str) -> bool {
    matches!(id, "1" | "2" | "3" | "4" | "5" | "S")
}
fn block_rank(id: &str) -> usize {
    match id {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "S" => 6,
        _ => usize::MAX,
    }
}
fn valid_tag(tag: &str) -> bool {
    let bytes = tag.as_bytes();
    matches!(bytes.len(), 2 | 3)
        && bytes[..2].iter().all(u8::is_ascii_digit)
        && (bytes.len() == 2 || bytes[2].is_ascii_uppercase())
}
