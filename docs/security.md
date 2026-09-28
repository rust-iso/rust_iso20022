# Security and privacy model

`rust_iso20022` treats XML, SWIFT MT, JSON, and profile selectors as untrusted
input. The limits below are availability controls, not a claim of formal
verification or constant-memory processing.

## XML defaults

Every public XML detection/deserialization path runs the shared streaming
preflight with `ParseLimits::DEFAULT` before a generated model is allocated.
Limits are inclusive: a value equal to the maximum is accepted; the first value
above it returns a typed `XmlReadError::LimitExceeded`.

| Resource | Default maximum |
|---|---:|
| UTF-8 input | 8 MiB |
| element depth | 64 |
| parser events | 1,000,000 |
| elements | 100,000 |
| attributes, total | 500,000 |
| one text/CDATA/comment event | 1 MiB |
| decoded text, total | 8 MiB |
| direct child elements of one parent | 100,000 |

The preflight rejects DTD and entity declarations, XInclude, non-UTF-8
declarations, NUL/invalid scalar input, unbound or malformed namespaces, and
malformed structure. `xsi:schemaLocation` is accepted only as inert data. The
core XML path has no resolver and performs no filesystem or network fetch.

## Entry-point audit

| Surface | Boundary |
|---|---|
| `detect`, `detect_message` | shared XML preflight before namespace/root scanning |
| `from_xml`, `MxMessage::parse`, `parse_as` | shared preflight before `yaserde` |
| high-level `parse`, generated `parse_auto` | bounded detection, then generated deserialization through `from_xml` |
| `MxNode::parse`, metadata, AppHdr, envelope | shared preflight before tree/string extraction |
| CLI | bounded reader (8 MiB) and shared core preflight |
| MCP | bounded JSON-RPC frame plus shared core preflight; no file/network/shell tools |
| WASM | shared core preflight and redacted structured errors |
| MT103/MT202/MT940 | `MtDocument::parse` with `MtParseLimits::DEFAULT` |

The library-level JSON helpers accept an in-memory `&str` and do not impose an
independent size limit. The CLI and WASM adapters cap JSON before generated
deserialization. Other applications accepting untrusted JSON must bound their
transport/input before calling `from_json` or `parse_json`.

## SWIFT MT defaults

| Resource | Default maximum |
|---|---:|
| input | 1 MiB |
| block nesting | 8 |
| blocks | 16 |
| fields | 512 |
| one field | 64 KiB |
| continuation lines per field | 256 |

MT conversion always parses with these defaults before mapping. Parse failures
contain a category and optional byte offset, not the field value.

## Allocation behavior and residual risk

XML is preflighted without constructing a document tree, then supported
generated messages are deserialized in a second pass. The preflight bounds the
source, nesting, event counts, text events, decoded text, attributes, elements,
and per-parent collection size. The underlying `xml-rs`/`yaserde` backends may
allocate event strings and generated collections proportional to those finite
limits; this is not a zero-allocation or constant-memory design.

The attack corpus covers oversized input, deep nesting, DTD/entity expansion,
XInclude, malformed/unbound namespaces, non-UTF-8 declarations, invalid
Unicode, excessive collections, and malformed structure. Exact minus/at/plus
tests freeze each limit. Coverage-guided fuzzing is tracked separately in
WP-029, so the current evidence must not be described as proof that no parser
defect exists.

## Privacy and logging

Core APIs do not log message bodies. Default redaction masks identifiers and
fully suppresses names, addresses, references, remittance text, and complete
XML. Errors, `Debug` output, validation reports, mapping reports, CLI, MCP, and
WASM are covered by canary tests.

WASM installs a panic hook that suppresses the panic payload because it may
contain caller-controlled financial data. Unredacted diagnostics require both
the non-default `unsafe-full-logging` Cargo feature and an explicit unsafe
runtime acknowledgement. No adapter enables that feature. Applications that
opt in must display `UNSAFE_FULL_LOGGING_WARNING` and independently control log
access, retention, export, backups, and incident handling.

For vulnerability reporting, see [SECURITY.md](../SECURITY.md).
