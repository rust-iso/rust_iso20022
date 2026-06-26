//! Local replacement for the `Validate` trait that `xsd-parser`-generated code
//! expects. The upstream `xsd-parser` crate is only a build-time dependency
//! (behind the `codegen` feature), so the generated runtime code cannot import
//! its `Validate` trait. We provide an equivalent here.
//!
//! Generated facet checks call `self.validate()` and propagate the `Err(String)`.
//! Types without restrictions get a blanket `impl Validate for T {}` emitted by
//! the generator, which uses the default no-op `validate`.

/// Mirrors `xsd_parser::generator::validator::Validate`.
pub trait Validate {
    /// Validate the value against its XSD facets. The default is a no-op so that
    /// unrestricted types satisfy the trait with `impl Validate for T {}`.
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
