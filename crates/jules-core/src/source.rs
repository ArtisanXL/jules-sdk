//! Source module.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

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

/// A `{ displayName: string }` reference to a branch of a GitHub repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubBranch {
    /// The human-readable name of the branch.
    pub display_name: String,
}

/// A GitHub repository backing a [`SourceResource`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubRepo {
    /// The GitHub organization or user that owns the repository.
    pub owner: String,
    /// The repository name.
    pub repo: String,
    /// Whether the repository is private.
    ///
    /// Omitted by the REST API when `false` (proto3 JSON encoding omits
    /// default scalar values), so this defaults to `false` when absent.
    #[serde(default)]
    pub is_private: bool,
    /// The repository's default branch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<GitHubBranch>,
    /// The branches available on the repository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<GitHubBranch>>,
}

/// A Jules `v1alpha` source resource, as returned by the REST API.
///
/// This is distinct from [`Source`], which is a lightweight builder type
/// used elsewhere in the SDK. Currently the only supported source variant
/// is a GitHub repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceResource {
    /// The resource name, e.g. `"sources/github/owner/repo"`.
    pub name: String,
    /// The unique id of the source, e.g. `"github/owner/repo"`.
    pub id: String,
    /// The GitHub repository backing this source, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub github_repo: Option<GitHubRepo>,
}

#[cfg(test)]
mod resource_tests {
    use super::*;

    #[test]
    fn source_resource_with_github_repo_round_trips() {
        let json = r#"{
            "name": "sources/github/owner/repo",
            "id": "github/owner/repo",
            "githubRepo": {
                "owner": "owner",
                "repo": "repo",
                "isPrivate": true,
                "defaultBranch": { "displayName": "main" },
                "branches": [
                    { "displayName": "main" },
                    { "displayName": "dev" }
                ]
            }
        }"#;

        let source: SourceResource = serde_json::from_str(json).unwrap();
        assert_eq!(source.name, "sources/github/owner/repo");
        let repo = source.github_repo.as_ref().unwrap();
        assert_eq!(repo.owner, "owner");
        assert!(repo.is_private);
        assert_eq!(repo.branches.as_ref().unwrap().len(), 2);

        let round_tripped: SourceResource =
            serde_json::from_str(&serde_json::to_string(&source).unwrap()).unwrap();
        assert_eq!(round_tripped, source);
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
