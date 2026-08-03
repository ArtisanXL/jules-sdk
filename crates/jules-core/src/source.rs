//! Source module.

use std::error::Error;
use std::fmt;

/// An error that can occur when building a [`Source`].
#[derive(Debug)]
pub struct SourceBuildError(String);

impl fmt::Display for SourceBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Source build error: {}", self.0)
    }
}

impl Error for SourceBuildError {}

/// Represents a source of information or data.
#[derive(Debug, Clone)]
pub struct Source {
    id: Option<String>,
    name: Option<String>,
}

impl Source {
    /// Creates a new [`SourceBuilder`] to construct a [`Source`].
    #[must_use]
    pub fn builder() -> SourceBuilder {
        SourceBuilder::default()
    }

    /// Returns the id of the source, if configured.
    #[must_use]
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Returns the name of the source, if configured.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

/// A builder for constructing a [`Source`].
#[derive(Debug, Default)]
pub struct SourceBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl SourceBuilder {
    /// Sets the id for the source.
    #[must_use]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the name for the source.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Builds the [`Source`] from the provided configuration.
    ///
    /// # Errors
    ///
    /// Returns a [`SourceBuildError`] if the source cannot be built from the provided configuration.
    pub fn build(self) -> Result<Source, SourceBuildError> {
        Ok(Source {
            id: self.id,
            name: self.name,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_builder_with_fields() {
        let source = Source::builder()
            .id("src-1")
            .name("My Source")
            .build()
            .unwrap();
        assert_eq!(source.id(), Some("src-1"));
        assert_eq!(source.name(), Some("My Source"));
    }

    #[test]
    fn test_source_builder_without_fields() {
        let source = Source::builder().build().unwrap();
        assert_eq!(source.id(), None);
        assert_eq!(source.name(), None);
    }
}
