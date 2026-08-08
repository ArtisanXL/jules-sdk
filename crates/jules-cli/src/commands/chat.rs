//! `chat` subcommand: create a new Jules session, or send a message to an existing one.
//!
//! With no `--session` flag, `chat` creates a new session via
//! [`JulesClient::create_session`] using the message as the prompt (a bare invocation with no
//! `--source` creates a repoless session). With `--session <id>`, it sends the message to that
//! existing session via [`JulesClient::send_message`].

use clap::Args;
use jules_sdk::jules_api::client::CreateSessionParams;
use jules_sdk::jules_core::session::{GithubRepoContext, SourceContext};
use jules_sdk::JulesClient;
use serde::Serialize;

use crate::api::session_resource_name;
use crate::commands::view::SessionView;
use crate::error::CliError;
use crate::utils::{OutputFormat, Render};

/// Arguments for the `chat` subcommand.
#[derive(Debug, Args)]
pub struct ChatArgs {
    /// The message to send, either as a new session's prompt or to an existing session.
    pub message: String,

    /// An existing session id (or full resource name) to send the message to, instead of
    /// creating a new session.
    #[arg(long)]
    pub session: Option<String>,

    /// The source repository to create the session against, as `owner/repo`. Ignored when
    /// `--session` is given.
    #[arg(long)]
    pub source: Option<String>,

    /// The branch to start the new session from. Requires `--source`.
    #[arg(long)]
    pub branch: Option<String>,

    /// An optional title for the new session.
    #[arg(long)]
    pub title: Option<String>,
}

/// The rendered result of a `chat` subcommand invocation.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ChatResult {
    /// A newly created session.
    Created(SessionView),
    /// Confirmation that a message was sent to an existing session.
    MessageSent {
        /// The full resource name of the session the message was sent to.
        session_name: String,
    },
}

impl Render for ChatResult {
    fn render_plain(&self) -> String {
        match self {
            Self::Created(session) => format!("Created session:\n{}", session.render_plain()),
            Self::MessageSent { session_name } => {
                format!("Message sent to session {session_name}")
            }
        }
    }
}

fn build_source_context(source: &str, branch: Option<&str>) -> Result<SourceContext, CliError> {
    let (owner, repo) = source.split_once('/').ok_or_else(|| {
        CliError::InvalidArgument(format!(
            "--source must be in `owner/repo` form, got `{source}`"
        ))
    })?;
    let mut context = SourceContext::new(format!("sources/github/{owner}/{repo}"));
    if let Some(branch) = branch {
        context = context.with_github_repo_context(GithubRepoContext::new(branch));
    }
    Ok(context)
}

/// Handles the `chat` subcommand.
///
/// # Errors
///
/// Returns an error if `--branch` is given without `--source`, `--source` is not in
/// `owner/repo` form, the Jules API request fails, or rendering the result fails.
pub async fn handle(
    client: &JulesClient,
    args: &ChatArgs,
    format: OutputFormat,
) -> Result<String, CliError> {
    let result = if let Some(session) = &args.session {
        let session_name = session_resource_name(session);
        client.send_message(&session_name, &args.message).await?;
        ChatResult::MessageSent { session_name }
    } else if args.branch.is_some() && args.source.is_none() {
        return Err(CliError::InvalidArgument(
            "--branch requires --source to also be given".to_string(),
        ));
    } else {
        let source_context = args
            .source
            .as_deref()
            .map(|source| build_source_context(source, args.branch.as_deref()))
            .transpose()?;

        let params = CreateSessionParams {
            title: args.title.clone(),
            prompt: Some(args.message.clone()),
            source_context,
        };
        let session = client.create_session(params).await?;
        ChatResult::Created(SessionView::from(&session))
    };

    result.render(format)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::MockServer;
    use jules_sdk::jules_api::auth::AuthType;
    use jules_sdk::JulesClientBuilder;
    use std::time::Duration;

    fn client_for(server: &MockServer) -> JulesClient {
        JulesClientBuilder::new()
            .base_url(server.base_url())
            .timeout(Duration::from_secs(5))
            .auth(AuthType::Bearer("test-token".to_string()))
            .build()
            .unwrap()
    }

    #[tokio::test]
    async fn no_session_creates_session_with_repoless_prompt() {
        let server = MockServer::respond_once(
            200,
            r#"{
                "id": "12345",
                "name": "sessions/12345",
                "title": "Fix the bug",
                "state": "QUEUED",
                "url": "https://jules.google.com/session/12345"
            }"#,
        )
        .await;
        let client = client_for(&server);
        let args = ChatArgs {
            message: "Fix the bug".to_string(),
            session: None,
            source: None,
            branch: None,
            title: None,
        };

        let output = handle(&client, &args, OutputFormat::Plain).await.unwrap();
        assert!(output.contains("Created session:"));
        assert!(output.contains("id: 12345"));
        assert!(output.contains("state: QUEUED"));

        let request = server.received().await;
        assert!(request.path.ends_with("/v1alpha/sessions"));
        let body: serde_json::Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(body["prompt"], "Fix the bug");
        assert!(body.get("sourceContext").is_none());
    }

    #[tokio::test]
    async fn source_and_branch_populate_source_context() {
        let server = MockServer::respond_once(
            200,
            r#"{"id": "1", "name": "sessions/1", "state": "QUEUED"}"#,
        )
        .await;
        let client = client_for(&server);
        let args = ChatArgs {
            message: "Fix the bug".to_string(),
            session: None,
            source: Some("owner/repo".to_string()),
            branch: Some("main".to_string()),
            title: Some("My title".to_string()),
        };

        handle(&client, &args, OutputFormat::Plain).await.unwrap();

        let request = server.received().await;
        let body: serde_json::Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(body["sourceContext"]["source"], "sources/github/owner/repo");
        assert_eq!(
            body["sourceContext"]["githubRepoContext"]["startingBranch"],
            "main"
        );
        assert_eq!(body["title"], "My title");
    }

    #[tokio::test]
    async fn invalid_source_format_is_rejected_without_a_network_call() {
        let server = MockServer::respond_once(200, "{}").await;
        let client = client_for(&server);
        let args = ChatArgs {
            message: "hi".to_string(),
            session: None,
            source: Some("not-a-valid-source".to_string()),
            branch: None,
            title: None,
        };

        let err = handle(&client, &args, OutputFormat::Plain)
            .await
            .unwrap_err();
        assert!(matches!(err, CliError::InvalidArgument(_)));
    }

    #[tokio::test]
    async fn branch_without_source_is_rejected_without_a_network_call() {
        let server = MockServer::respond_once(200, "{}").await;
        let client = client_for(&server);
        let args = ChatArgs {
            message: "hi".to_string(),
            session: None,
            source: None,
            branch: Some("main".to_string()),
            title: None,
        };

        let err = handle(&client, &args, OutputFormat::Plain)
            .await
            .unwrap_err();
        assert!(matches!(err, CliError::InvalidArgument(_)));
    }

    #[tokio::test]
    async fn with_session_sends_message_to_existing_session() {
        let server = MockServer::respond_once(200, "{}").await;
        let client = client_for(&server);
        let args = ChatArgs {
            message: "keep going".to_string(),
            session: Some("12345".to_string()),
            source: None,
            branch: None,
            title: None,
        };

        let output = handle(&client, &args, OutputFormat::Plain).await.unwrap();
        assert_eq!(output, "Message sent to session sessions/12345");

        let request = server.received().await;
        assert!(request
            .path
            .ends_with("/v1alpha/sessions/12345:sendMessage"));
        let body: serde_json::Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(body["prompt"], "keep going");
    }
}
