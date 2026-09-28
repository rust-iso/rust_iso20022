# Contract: Core Library API

This is an API-shape contract, not final source code. Exact module paths may be
refined only if compatibility tests and these semantics remain satisfied.

## Compatibility Surface

- Existing generated paths, `MxMessage`, `MxId`, `BusinessArea`,
  `CatalogueEntry`, serialization behavior, detection functions, features, and
  legacy WASM exports remain source/wire compatible unless an approved SemVer-
  major WP documents otherwise.
- New required data is exposed through new types. No required variant, public
  field, or trait item is added to an existing externally exhaustive/implementable
  type during an additive release.

## Parse and Inspect

```rust
pub fn detect_with_limits(
    input: &[u8],
    limits: &ParseLimits,
) -> Result<MessageDescriptor, DetectError>;

pub fn parse_with_limits(
    input: &[u8],
    limits: &ParseLimits,
) -> Result<ParsedMessage, ParseError>;

impl ParsedMessage {
    pub fn descriptor(&self) -> &'static MessageDescriptor;
    pub fn message_id(&self) -> &MessageIdentifier;
    pub fn business_area(&self) -> &str;
    pub fn family(&self) -> &str;
    pub fn version(&self) -> &MessageVersion;
    pub fn namespace(&self) -> &str;
    pub fn root_element(&self) -> &str;
    pub fn description(&self) -> &str;
    pub fn as_message_ref(&self) -> MessageRef<'_>;
    pub fn to_xml(&self) -> Result<String, SerializeError>;
}
```

JSON methods exist only with the JSON feature. Concrete generated access is
provided through documented typed accessors or a generated `AnyMessage`; no
field-copying high-level DTO is returned.

## Catalogue

```rust
pub trait Catalogue {
    fn lookup(&self, id: &MessageIdentifier) -> Option<&MessageDescriptor>;
    fn list_family(&self, family: &MessageFamily) -> &[MessageDescriptor];
    fn versions(&self, family: &MessageFamily) -> &[MessageDescriptor];
    fn latest(&self, family: &MessageFamily) -> Option<&MessageDescriptor>;
    fn by_namespace(&self, namespace: &str) -> Option<&MessageDescriptor>;
    fn by_root(&self, namespace: &str, root: &str)
        -> Option<&MessageDescriptor>;
}
```

All results come from generated schema metadata. Ordering is stable and numeric
by message-version components.

## Builders

```rust
pub trait GeneratedBuilder {
    type Output; // a generated Document
    type Error: std::error::Error + Send + Sync + 'static;

    fn build(self) -> Result<Self::Output, Self::Error>;
}
```

Every phase-one builder's `Output` is an exact
`generated::<area>::<version>::Document`. Helper conversion uses `From` or
`TryFrom`; errors name the missing/invalid logical path and safe reason.

## Errors

- Public fallible APIs use domain error enums/structs.
- Errors expose stable classification without requiring display-string parsing.
- `Display` and `Debug` are redaction-safe by default.
- No untrusted-input path panics. `anyhow` is limited to binary outer layers.

