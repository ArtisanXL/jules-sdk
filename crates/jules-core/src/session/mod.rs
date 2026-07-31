//! Session module.

use std::error::Error;
use std::fmt;

/// An error that can occur when building a [`Session`].
#[derive(Debug)]
pub struct SessionBuildError(String);

impl fmt::Display for SessionBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Session build error: {}", self.0)
    }
}

impl Error for SessionBuildError {}

/// A session represents an active context for interactions.
#[derive(Debug, Clone)]
pub struct Session {
    name: Option<String>,
}

impl Session {
    /// Creates a new [`SessionBuilder`] to construct a [`Session`].
    #[must_use]
    pub fn builder() -> SessionBuilder {
        SessionBuilder::default()
    }

    /// Returns the name of the session, if configured.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

/// A builder for constructing a [`Session`].
#[derive(Debug, Default)]
pub struct SessionBuilder {
    name: Option<String>,
}

impl SessionBuilder {
    /// Sets the name for the session.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Builds the [`Session`] from the provided configuration.
    ///
    /// # Errors
    ///
    /// Returns a [`SessionBuildError`] if the session cannot be built from the provided configuration.
    pub fn build(self) -> Result<Session, SessionBuildError> {
        Ok(Session { name: self.name })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_builder_with_name() {
        let session = Session::builder().name("My Session").build().unwrap();

        assert_eq!(session.name(), Some("My Session"));
    }

    #[test]
    fn test_session_builder_without_name() {
        let session = Session::builder().build().unwrap();

        assert_eq!(session.name(), None);
    }
}
