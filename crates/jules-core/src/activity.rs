//! Activity module.

use std::error::Error;
use std::fmt;

/// An error that can occur when building an [`Activity`].
#[derive(Debug)]
pub struct ActivityBuildError(String);

impl fmt::Display for ActivityBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Activity build error: {}", self.0)
    }
}

impl Error for ActivityBuildError {}

/// Represents an activity within a session.
#[derive(Debug, Clone)]
pub struct Activity {
    id: Option<String>,
    name: Option<String>,
}

impl Activity {
    /// Creates a new [`ActivityBuilder`] to construct an [`Activity`].
    #[must_use]
    pub fn builder() -> ActivityBuilder {
        ActivityBuilder::default()
    }

    /// Returns the id of the activity, if configured.
    #[must_use]
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Returns the name of the activity, if configured.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

/// A builder for constructing an [`Activity`].
#[derive(Debug, Default)]
pub struct ActivityBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ActivityBuilder {
    /// Sets the id for the activity.
    #[must_use]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the name for the activity.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Builds the [`Activity`] from the provided configuration.
    ///
    /// # Errors
    ///
    /// Returns an [`ActivityBuildError`] if the activity cannot be built from the provided configuration.
    pub fn build(self) -> Result<Activity, ActivityBuildError> {
        Ok(Activity {
            id: self.id,
            name: self.name,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activity_builder_with_fields() {
        let activity = Activity::builder()
            .id("act-1")
            .name("My Activity")
            .build()
            .unwrap();
        assert_eq!(activity.id(), Some("act-1"));
        assert_eq!(activity.name(), Some("My Activity"));
    }

    #[test]
    fn test_activity_builder_without_fields() {
        let activity = Activity::builder().build().unwrap();
        assert_eq!(activity.id(), None);
        assert_eq!(activity.name(), None);
    }
}
