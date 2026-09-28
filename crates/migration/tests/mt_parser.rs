use rust_iso20022_migration::mt::{MtDocument, MtLimit, MtParseErrorKind, MtParseLimits};

const VALID: &str = "{1:F01BANKBEBBAXXX0000000000}{2:I103BANKDEFFXXXXN}{3:{108:REF-1}}{4:\n:20:PAYMENT-1\n:32A:260928EUR1234,56\n:50K:/DE89370400440532013000\nJANE DOE 東京\n:71A:SHA\n:71A:OUR\n-}{5:{CHK:ABCDEF123456}}";

#[test]
fn parses_blocks_fields_continuations_duplicates_unicode_and_spans() {
    let document = MtDocument::parse(VALID).unwrap();
    assert_eq!(
        document.blocks().iter().map(|b| b.id()).collect::<Vec<_>>(),
        ["1", "2", "3", "4", "5"]
    );
    assert_eq!(document.fields().len(), 5);
    assert_eq!(document.fields()[2].tag(), "50K");
    assert_eq!(
        document.fields()[2].value(),
        "/DE89370400440532013000\nJANE DOE 東京"
    );
    assert_eq!(document.fields()[3].occurrence(), 1);
    assert_eq!(document.fields()[4].occurrence(), 2);
    for field in document.fields() {
        assert_eq!(&VALID[field.span().start..field.span().start + 1], ":");
    }
}

#[test]
fn rejects_malformed_sequence_blocks_and_fields_with_typed_safe_errors() {
    let cases = [
        ("", MtParseErrorKind::Empty),
        ("{4:\n:20:X\n-", MtParseErrorKind::InvalidStructure),
        ("{4:\n20:X\n-}", MtParseErrorKind::InvalidField),
        ("{4:\n:2A:X\n-}", MtParseErrorKind::InvalidField),
        (
            "{4:\n:20:X\n-}{2:I103X}",
            MtParseErrorKind::InvalidBlockSequence,
        ),
        (
            "{4:\n:20:X\n-}{4:\n:21:Y\n-}",
            MtParseErrorKind::DuplicateBlock,
        ),
        ("{1:F01X}", MtParseErrorKind::MissingTextBlock),
    ];
    for (input, expected) in cases {
        let error = MtDocument::parse(input).unwrap_err();
        assert_eq!(error.kind(), expected);
        if !input.is_empty() {
            assert!(!error.to_string().contains(input));
        }
        assert!(!format!("{error:?}").contains("PAYMENT"));
    }
}

#[test]
fn every_resource_limit_has_an_enforced_boundary() {
    let base = MtParseLimits::DEFAULT;
    let checks = [
        (
            MtParseLimits {
                max_input_bytes: VALID.len() - 1,
                ..base
            },
            MtLimit::InputBytes,
        ),
        (
            MtParseLimits {
                max_blocks: 4,
                ..base
            },
            MtLimit::BlockCount,
        ),
        (
            MtParseLimits {
                max_block_depth: 1,
                ..base
            },
            MtLimit::BlockDepth,
        ),
        (
            MtParseLimits {
                max_fields: 4,
                ..base
            },
            MtLimit::FieldCount,
        ),
        (
            MtParseLimits {
                max_field_bytes: 4,
                ..base
            },
            MtLimit::FieldBytes,
        ),
        (
            MtParseLimits {
                max_continuation_lines: 1,
                ..base
            },
            MtLimit::ContinuationLines,
        ),
    ];
    for (limits, expected) in checks {
        let error = MtDocument::parse_with_limits(VALID, limits).unwrap_err();
        assert_eq!(error.kind(), MtParseErrorKind::LimitExceeded(expected));
    }
}

#[test]
fn arbitrary_unicode_never_panics_and_is_either_parsed_or_typed() {
    for value in ["💶", "東京", "\0", "\u{2028}", "é", "{", "}", ":"] {
        let input = format!("{{4:\n:20:{value}\n-}}");
        let result = std::panic::catch_unwind(|| MtDocument::parse(&input));
        assert!(result.is_ok());
    }
}
