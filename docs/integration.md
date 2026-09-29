# Payment integration flow

The recommended application flow keeps routing metadata, generated values, and
validation results separate while using the same schema-derived catalogue.
Both `lookup_message` and `required_feature` accept either a canonical message
name or its full XSD namespace:

```text
XML input
  ↓
detect / lookup_message
  ↓
required_feature preflight
  ↓
parse_auto or typed from_xml
  ↓
generated struct
  ↓
validation binding / profile registry
  ↓
to_xml or to_json
```

For a complete executable example:

```bash
cargo run --example payment_pipeline --features model-pacs,serde
```

The example validates `pacs.008.001.08` using the generated document directly.
No high-level duplicate message DTO is introduced. Applications that need
several generated families can use the `payments` feature alias, while
resource-constrained applications should select individual `model-*` features.
