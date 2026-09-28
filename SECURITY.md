# Security policy

## Supported versions

This project has not published a production-grade release baseline yet.
Security fixes are developed on the default branch and will be included in the
next `0.1.x` release after verification. Older pre-release snapshots are not
maintained as separate security branches.

| Version | Security support |
|---|---|
| default branch / next `0.1.x` | supported |
| older snapshots | not supported |

## Report a vulnerability privately

Do not open a public issue for a suspected vulnerability. Use GitHub's
[private vulnerability reporting form](https://github.com/rust-iso/rust_iso20022/security/advisories/new).

Include:

- affected version or commit;
- affected feature flags and target;
- a minimal reproduction using synthetic or fully redacted data;
- expected and observed impact;
- whether the issue is already public or under active exploitation.

Never submit real payment messages, account numbers, customer names, addresses,
transaction references, credentials, or access tokens. Replace them with
synthetic values while preserving only the structure needed to reproduce the
issue.

Maintainers will assess reports and coordinate fixes and disclosure according
to severity and available maintenance capacity. No response or remediation SLA
is currently promised. Please allow a reasonable private remediation window
before public disclosure.

## Security boundary

The default parser rejects DTD/entity declarations, XInclude, non-UTF-8 XML,
malformed namespaces/structure, and inputs exceeding finite resource limits.
CLI, MCP, and WASM adapters reuse the same core gates. MCP exposes no network,
filesystem, upload, or shell capability. Exact limits and residual backend
risks are documented in [docs/security.md](docs/security.md).

Core APIs and adapters do not log raw financial messages by default. Unsafe
full-value diagnostics require a non-default compile-time feature and explicit
unsafe runtime acknowledgement. Enabling them transfers responsibility for log
access, retention, export, backups, and privacy compliance to the application.

Validation means only that input passed the implemented rules selected by the
caller. It does not guarantee bank/network acceptance, regulatory
certification, onboarding approval, or legal compliance.

Security fixes follow the same Spec Kit verification and evidence requirements
described in [CONTRIBUTING.md](CONTRIBUTING.md), with sensitive details kept
private until coordinated disclosure. Sponsorship does not change triage
criteria or provide access to private vulnerability reports.
