//! Walks through the Jules `v1alpha` REST API session/activity/source
//! endpoints exposed by [`jules_sdk::jules_api::client::JulesClient`], using a
//! mock [`Transport`] so this example runs without any real network access or
//! credentials.

use jules_sdk::jules_api::auth::AuthType;
use jules_sdk::jules_api::client::JulesClient;
use jules_sdk::jules_api::http::{HttpRequest, HttpResponse, Method, Transport};
use jules_sdk::jules_api::session::{CreateSessionRequest, ListSessionsParams};
use jules_sdk::jules_api::{activity::ListActivitiesParams, source::ListSourcesParams};
use jules_sdk::jules_core::activity::ActivityKind;
use jules_sdk::jules_core::errors::SDKError;
use jules_sdk::jules_core::session::resource::{GithubRepoContext, SourceContext};

/// A canned session, as it would be returned by the real Jules API.
fn session_json() -> &'static str {
    r#"{
        "name": "sessions/12345",
        "id": "12345",
        "prompt": "Fix the flaky retry test",
        "sourceContext": {
            "source": "sources/github/acme/widgets",
            "githubRepoContext": { "startingBranch": "main" }
        },
        "title": "Fix the flaky retry test",
        "createTime": "2026-08-08T00:00:00Z",
        "updateTime": "2026-08-08T00:00:00Z",
        "state": "QUEUED",
        "url": "https://jules.google.com/session/12345"
    }"#
}

/// A mock [`Transport`] that returns canned JSON responses for the
/// session/activity/source endpoints, keyed on method + URL.
struct MockTransport;

impl Transport for MockTransport {
    async fn send(&self, request: HttpRequest) -> Result<HttpResponse, SDKError> {
        let body: String = match (request.method, request.url.as_str()) {
            (Method::Post, url) if url.ends_with("/sessions") => session_json().to_string(),
            (Method::Get, url) if url.contains("/sessions?") || url.ends_with("/sessions") => {
                format!(
                    r#"{{ "sessions": [{}], "nextPageToken": null }}"#,
                    session_json()
                )
            }
            (Method::Get, url) if url.ends_with("/sessions/12345") => session_json().to_string(),
            (Method::Post, url) if url.ends_with("/sessions/12345:sendMessage") => "{}".to_string(),
            (Method::Post, url) if url.ends_with("/sessions/12345:approvePlan") => "{}".to_string(),
            (Method::Get, url) if url.ends_with("/activities/1") => r#"{
                "name": "sessions/12345/activities/1",
                "id": "1",
                "createTime": "2026-08-08T00:01:00Z",
                "originator": "agent",
                "agentMessaged": { "message": "I've reproduced the flake and have a fix." }
            }"#
            .to_string(),
            (Method::Get, url) if url.contains("/sessions/12345/activities") => r#"{
                "activities": [
                    {
                        "name": "sessions/12345/activities/1",
                        "id": "1",
                        "createTime": "2026-08-08T00:01:00Z",
                        "originator": "agent",
                        "agentMessaged": { "message": "I've reproduced the flake and have a fix." }
                    }
                ],
                "nextPageToken": null
            }"#
            .to_string(),
            (Method::Get, url) if url.contains("/sources") => r#"{
                "sources": [
                    {
                        "name": "sources/github/acme/widgets",
                        "id": "github/acme/widgets",
                        "githubRepo": {
                            "owner": "acme",
                            "repo": "widgets",
                            "isPrivate": false,
                            "defaultBranch": { "displayName": "main" }
                        }
                    }
                ],
                "nextPageToken": null
            }"#
            .to_string(),
            (method, url) => panic!("unexpected request: {method:?} {url}"),
        };
        Ok(HttpResponse::new(200, vec![], body.into_bytes()))
    }
}

fn main() {
    let f = async {
        let client = JulesClient::new(MockTransport, AuthType::jules_api_key("dummy-api-key"));

        println!("--- Creating a session ---");
        let create_request = CreateSessionRequest {
            prompt: "Fix the flaky retry test".to_string(),
            source_context: Some(SourceContext {
                source: "sources/github/acme/widgets".to_string(),
                github_repo_context: Some(GithubRepoContext {
                    starting_branch: "main".to_string(),
                }),
                working_branch: None,
                environment_variables_enabled: None,
            }),
            title: None,
            require_plan_approval: None,
            automation_mode: None,
        };
        let session = client.create_session(&create_request).await.unwrap();
        println!(
            "Created session {} \"{}\" (state: {:?})",
            session.id, session.title, session.state
        );
        println!("  URL: {}", session.url);

        println!("\n--- Listing sessions ---");
        let sessions = client
            .list_sessions(&ListSessionsParams::default())
            .await
            .unwrap();
        for s in &sessions.sessions {
            println!("  {} \"{}\" (state: {:?})", s.id, s.title, s.state);
        }

        println!("\n--- Getting session {} ---", session.id);
        let fetched = client.get_session(&session.id).await.unwrap();
        println!(
            "  {} \"{}\" (state: {:?}, url: {})",
            fetched.id, fetched.title, fetched.state, fetched.url
        );

        println!("\n--- Sending a message to session {} ---", session.id);
        client
            .send_message(&session.id, "Also add a regression test.")
            .await
            .unwrap();
        println!("  Message sent.");

        println!("\n--- Approving the plan for session {} ---", session.id);
        client.approve_plan(&session.id).await.unwrap();
        println!("  Plan approved.");

        println!("\n--- Listing activities for session {} ---", session.id);
        let activities = client
            .list_activities(&session.id, &ListActivitiesParams::default())
            .await
            .unwrap();
        for a in &activities.activities {
            println!(
                "  {} (originator: {:?}): {}",
                a.id,
                a.originator,
                describe(&a.kind)
            );
        }

        println!("\n--- Getting activity 1 from session {} ---", session.id);
        let activity = client.get_activity(&session.id, "1").await.unwrap();
        println!(
            "  {} (originator: {:?}): {}",
            activity.id,
            activity.originator,
            describe(&activity.kind)
        );

        println!("\n--- Listing connected sources ---");
        let sources = client
            .list_sources(&ListSourcesParams::default())
            .await
            .unwrap();
        for s in &sources.sources {
            println!("  {} ({})", s.id, s.name);
            if let Some(repo) = &s.github_repo {
                println!(
                    "    GitHub: {}/{} (private: {})",
                    repo.owner, repo.repo, repo.is_private
                );
            }
        }
    };

    // Poor man's block_on
    let waker = std::task::Waker::noop();
    let mut cx = std::task::Context::from_waker(waker);
    let mut future = std::boxed::Box::pin(f);
    let mut iters = 0;
    while std::future::Future::poll(future.as_mut(), &mut cx).is_pending() {
        iters += 1;
        assert!(iters <= 100, "future didn't resolve");
    }
}

/// Renders a short human-readable description of an activity's payload.
fn describe(kind: &ActivityKind) -> String {
    match kind {
        ActivityKind::AgentMessaged { message } => format!("agent messaged: {message}"),
        ActivityKind::UserMessaged { message } => format!("user messaged: {message}"),
        ActivityKind::PlanGenerated { plan } => {
            format!("plan generated: {} step(s)", plan.steps.len())
        }
        ActivityKind::PlanApproved { plan_id } => format!("plan approved: {plan_id}"),
        ActivityKind::ProgressUpdated { title, .. } => format!("progress updated: {title}"),
        ActivityKind::SessionCompleted {} => "session completed".to_string(),
        ActivityKind::SessionFailed { reason } => format!("session failed: {reason}"),
    }
}
