//! Renderable views over `jules-api` resource types, shared by the `chat`/`sessions`/`sources`
//! subcommands.

use jules_sdk::jules_core::activity::Activity;
use jules_sdk::jules_core::session::Session;
use jules_sdk::jules_core::source::{GithubRepo, Source};
use serde::Serialize;

use crate::utils::Render;

/// A renderable view of a [`Session`].
#[derive(Debug, Serialize)]
pub struct SessionView {
    /// The session's bare id, if known.
    pub id: Option<String>,
    /// The session's full resource name (e.g. `sessions/123`).
    pub name: Option<String>,
    /// The session's title.
    pub title: Option<String>,
    /// The session's current state (e.g. `AWAITING_USER_FEEDBACK`).
    pub state: Option<String>,
    /// A URL to view the session in the Jules web app.
    pub url: Option<String>,
}

impl From<&Session> for SessionView {
    fn from(session: &Session) -> Self {
        Self {
            id: session.id().map(str::to_string),
            name: session.name().map(str::to_string),
            title: session.title().map(str::to_string),
            state: session.state().map(str::to_string),
            url: session.url().map(str::to_string),
        }
    }
}

impl Render for SessionView {
    fn render_plain(&self) -> String {
        format!(
            "id: {}\nname: {}\ntitle: {}\nstate: {}\nurl: {}",
            self.id.as_deref().unwrap_or("-"),
            self.name.as_deref().unwrap_or("-"),
            self.title.as_deref().unwrap_or("-"),
            self.state.as_deref().unwrap_or("-"),
            self.url.as_deref().unwrap_or("-"),
        )
    }
}

/// A renderable view of a page of [`Session`]s.
#[derive(Debug, Serialize)]
pub struct SessionListView {
    /// The sessions in this page.
    pub sessions: Vec<SessionView>,
    /// A token to fetch the next page, if any further pages remain.
    pub next_page_token: Option<String>,
}

impl Render for SessionListView {
    fn render_plain(&self) -> String {
        if self.sessions.is_empty() {
            return "No sessions found.".to_string();
        }
        let mut lines: Vec<String> = self
            .sessions
            .iter()
            .map(|s| {
                format!(
                    "{}  {}  {}",
                    s.id.as_deref().unwrap_or("-"),
                    s.state.as_deref().unwrap_or("-"),
                    s.title.as_deref().unwrap_or("-"),
                )
            })
            .collect();
        if let Some(token) = &self.next_page_token {
            lines.push(format!(
                "(more results available, next page token: {token})"
            ));
        }
        lines.join("\n")
    }
}

/// A renderable view of a [`Source`].
#[derive(Debug, Serialize)]
pub struct SourceView {
    /// The source's bare id, if known.
    pub id: Option<String>,
    /// The source's full resource name (e.g. `sources/github/owner/repo`).
    pub name: Option<String>,
    /// The GitHub repository owner, if this is a GitHub source.
    pub owner: Option<String>,
    /// The GitHub repository name, if this is a GitHub source.
    pub repo: Option<String>,
    /// Whether the repository is private, if known.
    pub is_private: Option<bool>,
}

impl From<&Source> for SourceView {
    fn from(source: &Source) -> Self {
        let repo = source.github_repo();
        Self {
            id: source.id().map(str::to_string),
            name: source.name().map(str::to_string),
            owner: repo.and_then(GithubRepo::owner).map(str::to_string),
            repo: repo.and_then(GithubRepo::repo).map(str::to_string),
            is_private: repo.and_then(GithubRepo::is_private),
        }
    }
}

impl Render for SourceView {
    fn render_plain(&self) -> String {
        format!(
            "{}  {}/{}  (private: {})",
            self.name.as_deref().unwrap_or("-"),
            self.owner.as_deref().unwrap_or("-"),
            self.repo.as_deref().unwrap_or("-"),
            self.is_private
                .map_or("unknown".to_string(), |p| p.to_string()),
        )
    }
}

/// A renderable view of a page of [`Source`]s.
#[derive(Debug, Serialize)]
pub struct SourceListView {
    /// The sources in this page.
    pub sources: Vec<SourceView>,
    /// A token to fetch the next page, if any further pages remain.
    pub next_page_token: Option<String>,
}

impl Render for SourceListView {
    fn render_plain(&self) -> String {
        if self.sources.is_empty() {
            return "No sources found.".to_string();
        }
        let mut lines: Vec<String> = self.sources.iter().map(SourceView::render_plain).collect();
        if let Some(token) = &self.next_page_token {
            lines.push(format!(
                "(more results available, next page token: {token})"
            ));
        }
        lines.join("\n")
    }
}

/// A short, human-readable summary of what kind of event an [`Activity`] represents.
fn activity_summary(activity: &Activity) -> String {
    if let Some(plan) = activity.plan_generated().and_then(|p| p.plan()) {
        format!("plan generated ({} step(s))", plan.steps().len())
    } else if activity.extra().is_empty() {
        "activity".to_string()
    } else {
        activity
            .extra()
            .keys()
            .next()
            .cloned()
            .unwrap_or_else(|| "activity".to_string())
    }
}

/// A renderable view of an [`Activity`].
#[derive(Debug, Serialize)]
pub struct ActivityView {
    /// The activity's bare id, if known.
    pub id: Option<String>,
    /// The activity's full resource name.
    pub name: Option<String>,
    /// Who/what originated the activity (e.g. `AGENT`, `USER`).
    pub originator: Option<String>,
    /// A short, human-readable summary of the activity kind.
    pub summary: String,
}

impl From<&Activity> for ActivityView {
    fn from(activity: &Activity) -> Self {
        Self {
            id: activity.id().map(str::to_string),
            name: activity.name().map(str::to_string),
            originator: activity.originator().map(str::to_string),
            summary: activity_summary(activity),
        }
    }
}

impl Render for ActivityView {
    fn render_plain(&self) -> String {
        format!(
            "{}  {}  {}",
            self.id.as_deref().unwrap_or("-"),
            self.originator.as_deref().unwrap_or("-"),
            self.summary,
        )
    }
}

/// A renderable view of a page of [`Activity`]s.
#[derive(Debug, Serialize)]
pub struct ActivityListView {
    /// The activities in this page.
    pub activities: Vec<ActivityView>,
    /// A token to fetch the next page, if any further pages remain.
    pub next_page_token: Option<String>,
}

impl Render for ActivityListView {
    fn render_plain(&self) -> String {
        if self.activities.is_empty() {
            return "No activities found.".to_string();
        }
        let mut lines: Vec<String> = self
            .activities
            .iter()
            .map(ActivityView::render_plain)
            .collect();
        if let Some(token) = &self.next_page_token {
            lines.push(format!(
                "(more results available, next page token: {token})"
            ));
        }
        lines.join("\n")
    }
}
