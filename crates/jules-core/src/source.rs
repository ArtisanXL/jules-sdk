//! Source module.

use serde::{Deserialize, Serialize};
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

/// A branch reference within a GitHub repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Branch {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
}

impl Branch {
    /// Creates a new `Branch` with the given display name.
    #[must_use]
    pub fn new(display_name: impl Into<String>) -> Self {
        Self {
            display_name: Some(display_name.into()),
        }
    }

    /// Returns the branch's display name, if configured.
    #[must_use]
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }
}

/// GitHub repository details for a [`Source`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GithubRepo {
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_branch: Option<Branch>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    branches: Vec<Branch>,
}

impl GithubRepo {
    /// Creates a new `GithubRepo` for the given owner/repo.
    #[must_use]
    pub fn new(owner: impl Into<String>, repo: impl Into<String>) -> Self {
        Self {
            owner: Some(owner.into()),
            repo: Some(repo.into()),
            ..Self::default()
        }
    }

    /// Sets whether the repository is private.
    #[must_use]
    pub fn with_is_private(mut self, is_private: bool) -> Self {
        self.is_private = Some(is_private);
        self
    }

    /// Sets the repository's default branch.
    #[must_use]
    pub fn with_default_branch(mut self, default_branch: Branch) -> Self {
        self.default_branch = Some(default_branch);
        self
    }

    /// Sets the repository's known branches.
    #[must_use]
    pub fn with_branches(mut self, branches: Vec<Branch>) -> Self {
        self.branches = branches;
        self
    }

    /// Returns the repository owner, if configured.
    #[must_use]
    pub fn owner(&self) -> Option<&str> {
        self.owner.as_deref()
    }

    /// Returns the repository name, if configured.
    #[must_use]
    pub fn repo(&self) -> Option<&str> {
        self.repo.as_deref()
    }

    /// Returns whether the repository is private, if known.
    #[must_use]
    pub fn is_private(&self) -> Option<bool> {
        self.is_private
    }

    /// Returns the repository's default branch, if known.
    #[must_use]
    pub fn default_branch(&self) -> Option<&Branch> {
        self.default_branch.as_ref()
    }

    /// Returns the repository's known branches.
    #[must_use]
    pub fn branches(&self) -> &[Branch] {
        &self.branches
    }
}

/// Represents a connected source (e.g. a GitHub repository) the Jules API can operate on.
///
/// Field shape matches the real `v1alpha` Jules API `Source` resource (verified against the
/// live API's `GET /v1alpha/sources` response on 2026-08-08).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github_repo: Option<GithubRepo>,
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

    /// Returns the name of the source, if configured (e.g. `sources/github/owner/repo`).
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the GitHub repository details, if this source is a GitHub repo.
    #[must_use]
    pub fn github_repo(&self) -> Option<&GithubRepo> {
        self.github_repo.as_ref()
    }
}

/// A builder for constructing a [`Source`].
#[derive(Debug, Default)]
pub struct SourceBuilder {
    id: Option<String>,
    name: Option<String>,
    github_repo: Option<GithubRepo>,
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

    /// Sets the GitHub repository details for the source.
    #[must_use]
    pub fn github_repo(mut self, github_repo: GithubRepo) -> Self {
        self.github_repo = Some(github_repo);
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
            github_repo: self.github_repo,
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

    #[test]
    fn test_source_builder_with_github_repo() {
        let source = Source::builder()
            .id("src-1")
            .name("sources/github/example-owner/example-repo")
            .github_repo(GithubRepo::new("example-owner", "example-repo").with_is_private(true))
            .build()
            .unwrap();

        assert_eq!(
            source.github_repo().and_then(GithubRepo::owner),
            Some("example-owner")
        );
    }

    /// Deserializes a payload shaped like the real `v1alpha` API response, proving the
    /// `camelCase` wire format round-trips correctly into this `snake_case` model.
    #[test]
    fn test_source_deserializes_real_api_shape() {
        let json = r#"{
            "name": "sources/github/example-owner/example-repo",
            "githubRepo": {
                "owner": "example-owner",
                "repo": "example-repo",
                "isPrivate": true,
                "defaultBranch": {
                    "displayName": "main"
                },
                "branches": [
                    {"displayName": "main"},
                    {"displayName": "feature/x"}
                ]
            },
            "id": "github/example-owner/example-repo"
        }"#;

        let source: Source = serde_json::from_str(json).unwrap();
        assert_eq!(
            source.name(),
            Some("sources/github/example-owner/example-repo")
        );
        let repo = source.github_repo().unwrap();
        assert_eq!(repo.owner(), Some("example-owner"));
        assert_eq!(repo.is_private(), Some(true));
        assert_eq!(
            repo.default_branch().and_then(Branch::display_name),
            Some("main")
        );
        assert_eq!(repo.branches().len(), 2);
    }
}
