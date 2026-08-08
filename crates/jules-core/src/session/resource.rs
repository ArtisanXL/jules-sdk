//! REST v1alpha `Session` resource models.
//!
//! These types mirror the wire format of the Jules `v1alpha` REST API
//! (`https://jules.googleapis.com/v1alpha`) and are distinct from the
//! [`crate::session::Session`] builder type used elsewhere in the SDK.

use crate::activity::ActivityEvent;
use crate::source::SourceResource;
use serde::{Deserialize, Serialize};

/// The lifecycle state of a [`SessionResource`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    /// The state was not specified by the server.
    #[serde(rename = "STATE_UNSPECIFIED")]
    Unspecified,
    /// The session is queued for processing.
    #[serde(rename = "QUEUED")]
    Queued,
    /// The agent is planning its approach.
    #[serde(rename = "PLANNING")]
    Planning,
    /// The session is waiting for the user to approve a generated plan.
    #[serde(rename = "AWAITING_PLAN_APPROVAL")]
    AwaitingPlanApproval,
    /// The session is waiting for feedback from the user.
    #[serde(rename = "AWAITING_USER_FEEDBACK")]
    AwaitingUserFeedback,
    /// The agent is actively working on the session.
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    /// The session has been paused.
    #[serde(rename = "PAUSED")]
    Paused,
    /// The session failed.
    #[serde(rename = "FAILED")]
    Failed,
    /// The session completed successfully.
    #[serde(rename = "COMPLETED")]
    Completed,
}

/// Controls what the agent automatically does with the changes it produces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutomationMode {
    /// The automation mode was not specified by the server.
    #[serde(rename = "AUTOMATION_MODE_UNSPECIFIED")]
    Unspecified,
    /// The agent automatically creates a pull request when it is done.
    #[serde(rename = "AUTO_CREATE_PR")]
    AutoCreatePr,
}

/// A `{ displayName: string }` reference to a starting branch inside a GitHub repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubRepoContext {
    /// The name of the branch the session should start from.
    pub starting_branch: String,
}

/// Identifies the source repository and branch a [`SessionResource`] operates on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceContext {
    /// The resource name of the source, e.g. `"sources/github/owner/repo"`.
    pub source: String,
    /// GitHub-specific starting branch information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub github_repo_context: Option<GithubRepoContext>,
    /// The branch the agent is committing its work to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_branch: Option<String>,
    /// Whether environment variables are enabled for the session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_variables_enabled: Option<bool>,
}

/// A unidiff-style patch produced by the agent, along with commit metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitPatch {
    /// The patch contents in unidiff format.
    pub unidiff_patch: String,
    /// The commit id the patch should be applied on top of.
    pub base_commit_id: String,
    /// A suggested commit message describing the patch.
    pub suggested_commit_message: String,
}

/// A set of changes produced by the agent for a given source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSet {
    /// The resource name of the source the changes apply to.
    pub source: String,
    /// The patch describing the changes.
    pub git_patch: GitPatch,
}

/// A pull request opened by the agent as a result of a session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    /// The URL of the pull request.
    pub url: String,
    /// The pull request title.
    pub title: String,
    /// The pull request description.
    pub description: String,
    /// The base branch the pull request targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_ref: Option<String>,
    /// The head branch the pull request was created from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_ref: Option<String>,
}

/// A single output produced by a session: either a pull request or a raw change set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SessionOutput {
    /// The agent opened a pull request.
    PullRequest(PullRequest),
    /// The agent produced a change set without opening a pull request.
    ChangeSet(ChangeSet),
}

/// A single step within a generated [`Plan`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanStep {
    /// The unique id of the step.
    pub id: String,
    /// A short title for the step.
    pub title: String,
    /// An optional longer description of the step.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The zero-based position of the step within the plan.
    pub index: u32,
}

/// A plan generated by the agent describing the steps it intends to take.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    /// The unique id of the plan.
    pub id: String,
    /// The ordered steps that make up the plan.
    pub steps: Vec<PlanStep>,
    /// The RFC 3339 timestamp the plan was created at.
    pub create_time: String,
}

/// The kind of change made to a [`GeneratedFile`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeType {
    /// The file was newly created.
    Created,
    /// The file was modified.
    Modified,
    /// The file was deleted.
    Deleted,
}

/// An SDK-only convenience view of a single file changed by a session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedFile {
    /// The path of the file, relative to the repository root.
    pub path: String,
    /// The kind of change made to the file.
    pub change_type: ChangeType,
    /// The full contents of the file after the change.
    pub content: String,
    /// The number of lines added.
    pub additions: u32,
    /// The number of lines removed.
    pub deletions: u32,
}

/// A Jules `v1alpha` session resource, as returned by the REST API.
///
/// This is distinct from [`crate::session::Session`], which is a lightweight
/// builder type used elsewhere in the SDK.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionResource {
    /// The resource name, e.g. `"sessions/12345"`.
    pub name: String,
    /// The unique id of the session.
    pub id: String,
    /// The prompt the session was created with.
    pub prompt: String,
    /// The source and branch the session operates on.
    pub source_context: SourceContext,
    /// The resolved source, populated on some responses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceResource>,
    /// The session title.
    pub title: String,
    /// The RFC 3339 timestamp the session was created at.
    pub create_time: String,
    /// The RFC 3339 timestamp the session was last updated at.
    pub update_time: String,
    /// The current lifecycle state of the session.
    pub state: SessionState,
    /// Whether a generated plan requires explicit user approval.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_plan_approval: Option<bool>,
    /// The automation mode configured for the session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automation_mode: Option<AutomationMode>,
    /// A URL to view the session in the Jules web UI.
    pub url: String,
    /// The outputs produced by the session, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<SessionOutput>>,
    /// Activities belonging to the session, rarely populated inline.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activities: Option<Vec<ActivityEvent>>,
    /// SDK-only convenience view of the files changed by the session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_files: Option<Vec<GeneratedFile>>,
    /// Whether the session has been archived.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_resource_with_github_source_and_pull_request_round_trips() {
        let json = r#"{
            "name": "sessions/12345",
            "id": "12345",
            "prompt": "Fix the failing test",
            "sourceContext": {
                "source": "sources/github/owner/repo",
                "githubRepoContext": { "startingBranch": "main" },
                "workingBranch": "jules/fix-test",
                "environmentVariablesEnabled": true
            },
            "source": {
                "name": "sources/github/owner/repo",
                "id": "github/owner/repo",
                "githubRepo": {
                    "owner": "owner",
                    "repo": "repo",
                    "isPrivate": false,
                    "defaultBranch": { "displayName": "main" },
                    "branches": [{ "displayName": "main" }]
                }
            },
            "title": "Fix the failing test",
            "createTime": "2026-08-08T00:00:00Z",
            "updateTime": "2026-08-08T00:05:00Z",
            "state": "COMPLETED",
            "requirePlanApproval": true,
            "automationMode": "AUTO_CREATE_PR",
            "url": "https://jules.google.com/session/12345",
            "outputs": [
                {
                    "pullRequest": {
                        "url": "https://github.com/owner/repo/pull/1",
                        "title": "Fix the failing test",
                        "description": "Fixes it.",
                        "baseRef": "main",
                        "headRef": "jules/fix-test"
                    }
                }
            ],
            "archived": false
        }"#;

        let session: SessionResource = serde_json::from_str(json).unwrap();
        assert_eq!(session.name, "sessions/12345");
        assert_eq!(session.state, SessionState::Completed);
        assert_eq!(session.automation_mode, Some(AutomationMode::AutoCreatePr));
        assert_eq!(
            session.source_context.github_repo_context,
            Some(GithubRepoContext {
                starting_branch: "main".to_string()
            })
        );
        let source = session.source.as_ref().unwrap();
        assert_eq!(source.id, "github/owner/repo");
        assert!(!source.github_repo.as_ref().unwrap().is_private);

        let outputs = session.outputs.as_ref().unwrap();
        assert_eq!(outputs.len(), 1);
        match &outputs[0] {
            SessionOutput::PullRequest(pr) => {
                assert_eq!(pr.title, "Fix the failing test");
                assert_eq!(pr.base_ref.as_deref(), Some("main"));
            }
            SessionOutput::ChangeSet(_) => panic!("expected a pull request output"),
        }

        let round_tripped: SessionResource =
            serde_json::from_str(&serde_json::to_string(&session).unwrap()).unwrap();
        assert_eq!(round_tripped, session);
    }

    #[test]
    fn repoless_session_round_trips() {
        let json = r#"{
            "name": "sessions/no-source",
            "id": "no-source",
            "prompt": "Just chat with me",
            "sourceContext": {
                "source": "sources/github/owner/repo"
            },
            "title": "Chat session",
            "createTime": "2026-08-08T00:00:00Z",
            "updateTime": "2026-08-08T00:00:00Z",
            "state": "QUEUED",
            "url": "https://jules.google.com/session/no-source"
        }"#;

        let session: SessionResource = serde_json::from_str(json).unwrap();
        assert!(session.source.is_none());
        assert!(session.outputs.is_none());
        assert_eq!(session.state, SessionState::Queued);

        let round_tripped: SessionResource =
            serde_json::from_str(&serde_json::to_string(&session).unwrap()).unwrap();
        assert_eq!(round_tripped, session);
    }

    #[test]
    fn change_set_output_round_trips() {
        let json = r#"{
            "changeSet": {
                "source": "sources/github/owner/repo",
                "gitPatch": {
                    "unidiffPatch": "diff --git a/a b/a",
                    "baseCommitId": "abc123",
                    "suggestedCommitMessage": "Update a"
                }
            }
        }"#;

        let output: SessionOutput = serde_json::from_str(json).unwrap();
        match &output {
            SessionOutput::ChangeSet(cs) => assert_eq!(cs.source, "sources/github/owner/repo"),
            SessionOutput::PullRequest(_) => panic!("expected a change set output"),
        }

        let round_tripped: SessionOutput =
            serde_json::from_str(&serde_json::to_string(&output).unwrap()).unwrap();
        assert_eq!(round_tripped, output);
    }

    #[test]
    fn generated_file_round_trips() {
        let json = r#"{
            "path": "src/lib.rs",
            "changeType": "modified",
            "content": "fn main() {}",
            "additions": 3,
            "deletions": 1
        }"#;

        let file: GeneratedFile = serde_json::from_str(json).unwrap();
        assert_eq!(file.change_type, ChangeType::Modified);

        let round_tripped: GeneratedFile =
            serde_json::from_str(&serde_json::to_string(&file).unwrap()).unwrap();
        assert_eq!(round_tripped, file);
    }
}
