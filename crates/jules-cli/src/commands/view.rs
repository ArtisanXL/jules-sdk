//! Shared rendering views for session/source resources returned by the Jules API.

use jules_sdk::jules_core::session::resource::SessionResource;
use jules_sdk::jules_core::source::SourceResource;
use serde::Serialize;

use crate::utils::Render;

/// A rendered view of a [`SessionResource`]'s key fields.
#[derive(Debug, Serialize)]
pub struct SessionView {
    /// The unique id of the session.
    pub id: String,
    /// The session title.
    pub title: String,
    /// The current lifecycle state of the session.
    pub state: String,
    /// A URL to view the session in the Jules web UI.
    pub url: String,
}

impl From<&SessionResource> for SessionView {
    fn from(session: &SessionResource) -> Self {
        Self {
            id: session.id.clone(),
            title: session.title.clone(),
            state: format!("{:?}", session.state),
            url: session.url.clone(),
        }
    }
}

impl SessionView {
    /// Renders this session as a single tab-separated summary line, for use
    /// in list output.
    #[must_use]
    pub fn render_plain_line(&self) -> String {
        format!("{}\t{}\t{}\t{}", self.id, self.state, self.title, self.url)
    }
}

impl Render for SessionView {
    fn render_plain(&self) -> String {
        format!(
            "id: {}\ntitle: {}\nstate: {}\nurl: {}",
            self.id, self.title, self.state, self.url
        )
    }
}

/// A rendered view of a [`SourceResource`]'s key fields.
#[derive(Debug, Serialize)]
pub struct SourceView {
    /// The resource name, e.g. `"sources/github/owner/repo"`.
    pub name: String,
    /// The unique id of the source, e.g. `"github/owner/repo"`.
    pub id: String,
    /// The GitHub owner, if this is a GitHub-backed source.
    pub owner: Option<String>,
    /// The GitHub repository name, if this is a GitHub-backed source.
    pub repo: Option<String>,
}

impl From<&SourceResource> for SourceView {
    fn from(source: &SourceResource) -> Self {
        Self {
            name: source.name.clone(),
            id: source.id.clone(),
            owner: source.github_repo.as_ref().map(|repo| repo.owner.clone()),
            repo: source.github_repo.as_ref().map(|repo| repo.repo.clone()),
        }
    }
}

impl SourceView {
    /// Renders this source as a single tab-separated summary line, for use
    /// in list output.
    #[must_use]
    pub fn render_plain_line(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}",
            self.id,
            self.name,
            self.owner.as_deref().unwrap_or("-"),
            self.repo.as_deref().unwrap_or("-")
        )
    }
}
