//! `simple_type!` — yaserde (de)serialization for the generated leaf newtypes.
//!
//! ISO 20022 simple types are generated as single-field tuple newtypes such as
//! `pub struct Max35Text(pub String)` or `pub struct YesNoIndicator(pub bool)`.
//! Upstream `xsd-parser` derives `UtilsTupleIo` / `UtilsDefaultSerde` for these,
//! but `xsd-macro-utils` is not published on crates.io (and pulled in
//! `xsd-types`). This macro provides the equivalent impls locally — `FromStr`,
//! `Display`, and text-content `YaSerialize` / `YaDeserialize` — so the crate
//! depends only on published crates.
//!
//! Works for any newtype `(pub T)` whose `T` is `FromStr` (with a `Debug`
//! error) and `Display`, which covers the `String` and `bool` leaf types.

/// Implement `FromStr`, `Display` and yaserde text (de)serialization for a
/// single-field tuple newtype.
#[macro_export]
macro_rules! simple_type {
    ($name:ident) => {
        impl ::core::str::FromStr for $name {
            type Err = ::std::string::String;
            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                s.parse().map($name).map_err(|e| ::std::format!("{:?}", e))
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::write!(f, "{}", self.0)
            }
        }

        impl ::yaserde::YaSerialize for $name {
            fn serialize<W: ::std::io::Write>(
                &self,
                writer: &mut ::yaserde::ser::Serializer<W>,
            ) -> ::core::result::Result<(), ::std::string::String> {
                let name = writer
                    .get_start_event_name()
                    .unwrap_or_else(|| ::std::stringify!($name).to_string());
                if !writer.skip_start_end() {
                    writer
                        .write(::xml::writer::XmlEvent::start_element(name.as_str()))
                        .map_err(|_| "Start element write failed".to_string())?;
                }
                writer
                    .write(::xml::writer::XmlEvent::characters(&self.to_string()))
                    .map_err(|_| "Element value write failed".to_string())?;
                if !writer.skip_start_end() {
                    writer
                        .write(::xml::writer::XmlEvent::end_element())
                        .map_err(|_| "End element write failed".to_string())?;
                }
                Ok(())
            }

            fn serialize_attributes(
                &self,
                attributes: ::std::vec::Vec<::xml::attribute::OwnedAttribute>,
                namespace: ::xml::namespace::Namespace,
            ) -> ::core::result::Result<
                (
                    ::std::vec::Vec<::xml::attribute::OwnedAttribute>,
                    ::xml::namespace::Namespace,
                ),
                ::std::string::String,
            > {
                Ok((attributes, namespace))
            }
        }

        impl ::yaserde::YaDeserialize for $name {
            fn deserialize<R: ::std::io::Read>(
                reader: &mut ::yaserde::de::Deserializer<R>,
            ) -> ::core::result::Result<Self, ::std::string::String> {
                if let Ok(::xml::reader::XmlEvent::StartElement { .. }) = reader.peek() {
                    reader.next_event()?;
                } else {
                    return Err("Start element not found".to_string());
                }
                if let Ok(::xml::reader::XmlEvent::Characters(ref text)) = reader.peek() {
                    <$name as ::core::str::FromStr>::from_str(text)
                } else {
                    <$name as ::core::str::FromStr>::from_str("")
                }
            }
        }
    };
}
