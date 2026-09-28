# Local MCP server

`rust-iso20022-mcp` is a thin, local Model Context Protocol adapter over the
`rust_iso20022` core SDK. It communicates only through standard input and
standard output. Generated structs remain the canonical ISO 20022 model; the
server has no independent parser, catalogue, comparison engine, or validation
rules.

Build and run it with a Rust 1.88 or newer toolchain:

```console
cargo +stable build -p rust_iso20022_mcp
cargo +stable run -p rust_iso20022_mcp
```

The core library continues to support its separately documented Rust 1.85
MSRV. The MCP crate uses Rust 1.88 because the pinned official `rmcp` 3.0.0 SDK
requires it. MCP is a separate, non-default workspace binary and therefore does
not raise the core library's MSRV or dependency surface.

## Protocol and schemas

The server targets the [MCP `2026-07-28`
specification](https://modelcontextprotocol.io/specification/2026-07-28) through
the [official Rust SDK](https://github.com/modelcontextprotocol/rust-sdk).
Compatibility with older protocol revisions is supplied by that SDK's protocol
negotiation, not by adapter-owned wire parsing. The exact seven tool definitions
are tracked in
[`crates/mcp/schemas/tools-v1.json`](../crates/mcp/schemas/tools-v1.json).
Regenerate the candidate document with:

```console
cargo +stable run -p rust_iso20022_mcp --example tool_schemas
```

The contract test requires generated definitions to be object-equal to the
tracked schema before a schema change can be accepted.

## Tools

| Tool | Input | Result source |
|---|---|---|
| `detect_message` | In-memory XML `message` | Bounded core detector |
| `inspect_message` | In-memory XML `message` | Generated message descriptor |
| `validate_message` | In-memory XML, explicit `layer`, optional exact `profile_release` | Core generated-message validation binding |
| `lookup_message` | Exact `message_id` | Generated catalogue |
| `lookup_field` | Exact `message_id` and normalized `logical_path` | Generated field metadata |
| `compare_versions` | Exact `from_message_id` and `to_message_id` | Schema-metadata comparison |
| `explain_validation_error` | Exact `rule_id` | Executable core rule registry |

The implemented validation slice is L2 ISO semantic validation for messages
with an installed generated binding. L1 XSD validation and L3 market-profile
execution return an explicit unavailable error; they never silently degrade to
another layer. A profile request must still use the complete immutable release
key described in [profiles.md](profiles.md). Short selectors such as
`cbpr-plus:2026` are rejected as incomplete.

## Security and privacy boundary

The server accepts message values in memory. Its tool schemas expose no file
path, URL, upload, network, shell, or command input, and its dependency features
enable only server macros and stdio transport. It does not include HTTP,
client, authentication, child-process, filesystem, or network capabilities.

Incoming JSON-RPC lines are bounded to the core XML input limit plus 1 MiB of
protocol overhead. XML content is then processed with the core parser limits.
Malformed, oversized, forbidden, and unsupported inputs produce generic
errors. The adapter does not log source messages, account data, names,
addresses, transaction references, or caller-supplied selectors. Standard
output is reserved for MCP protocol frames; the current adapter emits no
diagnostic logging to standard error.

These restrictions are intentional. Additions involving remote transport,
files, URLs, uploads, network access, commands, or payload logging require a
new reviewed capability boundary and schema version.

## Compliance boundary

A successful result means the message passed the rules implemented and selected
by this SDK. It does not guarantee acceptance by a bank or network, regulatory
certification, onboarding approval, or legal compliance.
