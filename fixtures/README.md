# Acceptance Fixtures

Fixtures are synthetic or irreversibly anonymized and organized by capability:

```text
fixtures/
├── iso/{valid,invalid}/
├── cbpr_plus/{valid,invalid}/
├── sepa/{sct,sct_inst}/{valid,invalid}/
└── migration/{mt103,mt202,mt940}/
```

Each fixture has a sidecar named `<fixture>.expected.json` using this shape:

```json
{
  "format_version": 1,
  "fixture_id": "stable-id",
  "message_id": "pacs.008.001.10",
  "serialization": "xml",
  "expected_valid": false,
  "validation": {
    "layers": ["iso_semantic"],
    "profile": null,
    "issues": [
      {
        "rule_id": "ISO20022-L2-EXAMPLE",
        "path": "/Document/example",
        "severity": "error"
      }
    ]
  },
  "migration": null,
  "source_digests": []
}
```

Profile sidecars must use an exact composite profile release and source digests.
Migration sidecars must account for every populated source field and include
`Exact`, `Derived`, `Lossy`, `Ambiguous`, or `Unsupported` mapping
classification. Invalid fixtures must always declare expected rule IDs and paths.
No sidecar or error expectation may contain raw sensitive values.

