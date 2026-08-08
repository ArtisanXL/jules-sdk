//! Activity module.

use std::error::Error;
use std::fmt;

use crate::session::resource::{ChangeSet, Plan};
use serde::{Deserialize, Serialize};

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

/// Who or what triggered an [`ActivityEvent`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Originator {
    /// The activity was triggered by the end user.
    User,
    /// The activity was triggered by the agent.
    Agent,
    /// The activity was triggered by the system.
    System,
}

/// Base64-encoded media data attached to an [`Artifact`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    /// The base64-encoded media contents.
    pub data: String,
    /// The MIME type of the media.
    pub mime_type: String,
}

/// A piece of content attached to an [`ActivityEvent`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Artifact {
    /// A set of changes attached to the activity.
    ChangeSet {
        /// The attached change set.
        change_set: ChangeSet,
    },
    /// A media attachment on the activity.
    Media {
        /// The attached media.
        media: Media,
    },
}

/// The variant-specific payload of an [`ActivityEvent`], discriminated by which
/// field is present on the wire (e.g. `agentMessaged`, `planGenerated`, ...).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ActivityKind {
    /// The agent posted a message.
    AgentMessaged {
        /// The message contents.
        message: String,
    },
    /// The user posted a message.
    UserMessaged {
        /// The message contents.
        message: String,
    },
    /// The agent generated a plan.
    PlanGenerated {
        /// The generated plan.
        plan: Plan,
    },
    /// The user approved a plan.
    PlanApproved {
        /// The id of the approved plan.
        plan_id: String,
    },
    /// The agent reported progress on its current step.
    ProgressUpdated {
        /// A short title describing the progress update.
        title: String,
        /// A longer description of the progress update.
        description: String,
    },
    /// The session completed successfully.
    SessionCompleted {},
    /// The session failed.
    SessionFailed {
        /// The reason the session failed.
        reason: String,
    },
}

/// A single Jules `v1alpha` activity resource, as returned by the REST API.
///
/// This is distinct from [`Activity`], which is a lightweight builder type
/// used elsewhere in the SDK.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityEvent {
    /// The resource name, e.g. `"sessions/12345/activities/1"`.
    pub name: String,
    /// The unique id of the activity.
    pub id: String,
    /// An optional human-readable description of the activity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The RFC 3339 timestamp the activity was created at.
    pub create_time: String,
    /// Who or what triggered the activity.
    pub originator: Originator,
    /// Artifacts attached to the activity.
    #[serde(default)]
    pub artifacts: Vec<Artifact>,
    /// The variant-specific payload of the activity.
    #[serde(flatten)]
    pub kind: ActivityKind,
}

#[cfg(test)]
mod resource_tests {
    use super::*;

    fn activity_json(kind_json: &str) -> String {
        format!(
            r#"{{
                "name": "sessions/12345/activities/1",
                "id": "1",
                "description": "An activity",
                "createTime": "2026-08-08T00:00:00Z",
                "originator": "agent",
                "artifacts": [],
                {kind_json}
            }}"#
        )
    }

    fn round_trip(json: &str) -> ActivityEvent {
        let activity: ActivityEvent = serde_json::from_str(json).unwrap();
        let round_tripped: ActivityEvent =
            serde_json::from_str(&serde_json::to_string(&activity).unwrap()).unwrap();
        assert_eq!(round_tripped, activity);
        activity
    }

    #[test]
    fn agent_messaged_round_trips() {
        let json = activity_json(r#""agentMessaged": { "message": "hello" }"#);
        let activity = round_trip(&json);
        assert_eq!(
            activity.kind,
            ActivityKind::AgentMessaged {
                message: "hello".to_string()
            }
        );
    }

    #[test]
    fn user_messaged_round_trips() {
        let json = activity_json(r#""userMessaged": { "message": "hi there" }"#);
        let activity = round_trip(&json);
        assert_eq!(
            activity.kind,
            ActivityKind::UserMessaged {
                message: "hi there".to_string()
            }
        );
    }

    #[test]
    fn plan_generated_round_trips() {
        let json = activity_json(
            r#""planGenerated": {
                "plan": {
                    "id": "plan-1",
                    "steps": [
                        { "id": "step-1", "title": "Do the thing", "index": 0 }
                    ],
                    "createTime": "2026-08-08T00:00:00Z"
                }
            }"#,
        );
        let activity = round_trip(&json);
        match activity.kind {
            ActivityKind::PlanGenerated { plan } => assert_eq!(plan.steps.len(), 1),
            other => panic!("unexpected kind: {other:?}"),
        }
    }

    #[test]
    fn plan_approved_round_trips() {
        let json = activity_json(r#""planApproved": { "planId": "plan-1" }"#);
        let activity = round_trip(&json);
        assert_eq!(
            activity.kind,
            ActivityKind::PlanApproved {
                plan_id: "plan-1".to_string()
            }
        );
    }

    #[test]
    fn progress_updated_round_trips() {
        let json = activity_json(
            r#""progressUpdated": { "title": "Working", "description": "Running tests" }"#,
        );
        let activity = round_trip(&json);
        assert_eq!(
            activity.kind,
            ActivityKind::ProgressUpdated {
                title: "Working".to_string(),
                description: "Running tests".to_string()
            }
        );
    }

    #[test]
    fn session_completed_round_trips() {
        let json = activity_json(r#""sessionCompleted": {}"#);
        let activity = round_trip(&json);
        assert_eq!(activity.kind, ActivityKind::SessionCompleted {});
    }

    #[test]
    fn session_failed_round_trips() {
        let json = activity_json(r#""sessionFailed": { "reason": "timed out" }"#);
        let activity = round_trip(&json);
        assert_eq!(
            activity.kind,
            ActivityKind::SessionFailed {
                reason: "timed out".to_string()
            }
        );
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
