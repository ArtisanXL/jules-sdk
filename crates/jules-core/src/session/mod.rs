//! Session module.

use serde::{Deserialize, Serialize};
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

/// GitHub-specific context for a [`SourceContext`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubRepoContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_branch: Option<String>,
}

impl GithubRepoContext {
    /// Creates a new `GithubRepoContext` with the given starting branch.
    #[must_use]
    pub fn new(starting_branch: impl Into<String>) -> Self {
        Self {
            starting_branch: Some(starting_branch.into()),
        }
    }

    /// Returns the starting branch, if configured.
    #[must_use]
    pub fn starting_branch(&self) -> Option<&str> {
        self.starting_branch.as_deref()
    }
}

/// The source a [`Session`] operates on, as returned/accepted by the Jules API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SourceContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github_repo_context: Option<GithubRepoContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_variables_enabled: Option<bool>,
}

impl SourceContext {
    /// Creates a new `SourceContext` pointing at the given source (e.g. `sources/github/owner/repo`).
    #[must_use]
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: Some(source.into()),
            ..Self::default()
        }
    }

    /// Sets the GitHub repo context (e.g. starting branch).
    #[must_use]
    pub fn with_github_repo_context(mut self, context: GithubRepoContext) -> Self {
        self.github_repo_context = Some(context);
        self
    }

    /// Sets whether environment variables are enabled for this source context.
    #[must_use]
    pub fn with_environment_variables_enabled(mut self, enabled: bool) -> Self {
        self.environment_variables_enabled = Some(enabled);
        self
    }

    /// Returns the source identifier, if configured.
    #[must_use]
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    /// Returns the GitHub repo context, if configured.
    #[must_use]
    pub fn github_repo_context(&self) -> Option<&GithubRepoContext> {
        self.github_repo_context.as_ref()
    }

    /// Returns whether environment variables are enabled, if configured.
    #[must_use]
    pub fn environment_variables_enabled(&self) -> Option<bool> {
        self.environment_variables_enabled
    }
}

/// A session represents an active context for interactions with the Jules API.
///
/// Field shape matches the real `v1alpha` Jules API `Session` resource (verified against the
/// live API's `GET /v1alpha/sessions` and `GET /v1alpha/{name}` responses on 2026-08-08).
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_context: Option<SourceContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

impl std::fmt::Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Session")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("title", &self.title)
            .field("create_time", &self.create_time)
            .field("update_time", &self.update_time)
            .field("state", &self.state)
            .field("source_context", &self.source_context)
            .field("prompt", &self.prompt.as_ref().map(|_| "***REDACTED***"))
            .field("url", &self.url)
            .finish()
    }
}

impl Session {
    /// Creates a new [`SessionBuilder`] to construct a [`Session`].
    #[must_use]
    pub fn builder() -> SessionBuilder {
        SessionBuilder::default()
    }

    /// Returns the name of the session, if configured (e.g. `sessions/1234567890`).
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the id of the session, if configured.
    #[must_use]
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Returns the human-readable title of the session, if configured.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the session's creation timestamp (RFC 3339), if configured.
    #[must_use]
    pub fn create_time(&self) -> Option<&str> {
        self.create_time.as_deref()
    }

    /// Returns the session's last-update timestamp (RFC 3339), if configured.
    #[must_use]
    pub fn update_time(&self) -> Option<&str> {
        self.update_time.as_deref()
    }

    /// Returns the session's state (e.g. `AWAITING_USER_FEEDBACK`), if configured.
    #[must_use]
    pub fn state(&self) -> Option<&str> {
        self.state.as_deref()
    }

    /// Returns the session's source context, if configured.
    #[must_use]
    pub fn source_context(&self) -> Option<&SourceContext> {
        self.source_context.as_ref()
    }

    /// Returns the prompt the session was started with, if configured.
    #[must_use]
    pub fn prompt(&self) -> Option<&str> {
        self.prompt.as_deref()
    }

    /// Returns the session's URL, if configured.
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
}

/// A builder for constructing a [`Session`].
#[derive(Default)]
pub struct SessionBuilder {
    id: Option<String>,
    name: Option<String>,
    title: Option<String>,
    create_time: Option<String>,
    update_time: Option<String>,
    state: Option<String>,
    source_context: Option<SourceContext>,
    prompt: Option<String>,
    url: Option<String>,
}

impl std::fmt::Debug for SessionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionBuilder")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("title", &self.title)
            .field("create_time", &self.create_time)
            .field("update_time", &self.update_time)
            .field("state", &self.state)
            .field("source_context", &self.source_context)
            .field("prompt", &self.prompt.as_ref().map(|_| "***REDACTED***"))
            .field("url", &self.url)
            .finish()
    }
}

impl SessionBuilder {
    /// Sets the id for the session.
    #[must_use]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the name for the session.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the title for the session.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the creation timestamp for the session.
    #[must_use]
    pub fn create_time(mut self, create_time: impl Into<String>) -> Self {
        self.create_time = Some(create_time.into());
        self
    }

    /// Sets the last-update timestamp for the session.
    #[must_use]
    pub fn update_time(mut self, update_time: impl Into<String>) -> Self {
        self.update_time = Some(update_time.into());
        self
    }

    /// Sets the state for the session.
    #[must_use]
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Sets the source context for the session.
    #[must_use]
    pub fn source_context(mut self, source_context: SourceContext) -> Self {
        self.source_context = Some(source_context);
        self
    }

    /// Sets the prompt for the session.
    #[must_use]
    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    /// Sets the URL for the session.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Builds the [`Session`] from the provided configuration.
    ///
    /// # Errors
    ///
    /// Returns a [`SessionBuildError`] if the session cannot be built from the provided configuration.
    pub fn build(self) -> Result<Session, SessionBuildError> {
        Ok(Session {
            id: self.id,
            name: self.name,
            title: self.title,
            create_time: self.create_time,
            update_time: self.update_time,
            state: self.state,
            source_context: self.source_context,
            prompt: self.prompt,
            url: self.url,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_builder_with_name_and_id() {
        let session = Session::builder()
            .id("session-123")
            .name("My Session")
            .build()
            .unwrap();

        assert_eq!(session.id(), Some("session-123"));
        assert_eq!(session.name(), Some("My Session"));
    }

    #[test]
    fn test_session_builder_without_fields() {
        let session = Session::builder().build().unwrap();

        assert_eq!(session.id(), None);
        assert_eq!(session.name(), None);
    }

    #[test]
    fn test_session_builder_with_full_fields() {
        let session = Session::builder()
            .id("11413719004378428992")
            .name("sessions/11413719004378428992")
            .title("Example session")
            .create_time("2026-08-08T12:42:12.441608052Z")
            .update_time("2026-08-08T12:59:20.277897Z")
            .state("AWAITING_USER_FEEDBACK")
            .source_context(
                SourceContext::new("sources/github/example-owner/example-repo")
                    .with_github_repo_context(GithubRepoContext::new("main"))
                    .with_environment_variables_enabled(true),
            )
            .prompt("Do the thing")
            .url("https://jules.google.com/session/123")
            .build()
            .unwrap();

        assert_eq!(session.title(), Some("Example session"));
        assert_eq!(session.state(), Some("AWAITING_USER_FEEDBACK"));
        assert_eq!(
            session.source_context().and_then(SourceContext::source),
            Some("sources/github/example-owner/example-repo")
        );
        assert_eq!(
            session
                .source_context()
                .and_then(SourceContext::github_repo_context)
                .and_then(GithubRepoContext::starting_branch),
            Some("main")
        );
    }

    /// Deserializes a payload shaped like the real `v1alpha` API response, proving the
    /// `camelCase` wire format round-trips correctly into this `snake_case` model.
    #[test]
    fn test_session_deserializes_real_api_shape() {
        let json = r#"{
            "name": "sessions/11413719004378428992",
            "title": "Example session",
            "createTime": "2026-08-08T12:42:12.441608052Z",
            "updateTime": "2026-08-08T12:59:20.277897Z",
            "state": "AWAITING_USER_FEEDBACK",
            "sourceContext": {
                "source": "sources/github/example-owner/example-repo",
                "githubRepoContext": {
                    "startingBranch": "main"
                },
                "environmentVariablesEnabled": true
            },
            "prompt": "Do the thing",
            "url": "https://jules.google.com/session/123",
            "id": "11413719004378428992"
        }"#;

        let session: Session = serde_json::from_str(json).unwrap();
        assert_eq!(session.name(), Some("sessions/11413719004378428992"));
        assert_eq!(session.state(), Some("AWAITING_USER_FEEDBACK"));
        assert_eq!(
            session
                .source_context()
                .and_then(SourceContext::github_repo_context)
                .and_then(GithubRepoContext::starting_branch),
            Some("main")
        );
    }
}
