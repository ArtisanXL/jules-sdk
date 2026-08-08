//! `sessions` subcommand: list or inspect Jules sessions (read-only).

use clap::{Args, Subcommand};
use jules_sdk::JulesClient;

use crate::api::session_resource_name;
use crate::commands::view::{SessionListView, SessionView};
use crate::error::CliError;
use crate::utils::{OutputFormat, Render};

/// Arguments for the `sessions` subcommand.
#[derive(Debug, Args)]
pub struct SessionsArgs {
    /// The `sessions` subcommand to run.
    #[command(subcommand)]
    pub command: SessionsCommand,
}

/// `sessions` subcommands.
#[derive(Debug, Subcommand)]
pub enum SessionsCommand {
    /// Lists sessions.
    List {
        /// The maximum number of sessions to return.
        #[arg(long)]
        page_size: Option<i32>,
        /// A page token from a previous `sessions list` response.
        #[arg(long)]
        page_token: Option<String>,
    },
    /// Fetches a single session by id (or full resource name).
    Get {
        /// The session id (e.g. `12345`) or full resource name (e.g. `sessions/12345`).
        id: String,
    },
}

/// Handles the `sessions` subcommand.
///
/// # Errors
///
/// Returns a [`CliError`] if the Jules API request fails or rendering the result fails.
pub async fn handle(
    client: &JulesClient,
    args: &SessionsArgs,
    format: OutputFormat,
) -> Result<String, CliError> {
    match &args.command {
        SessionsCommand::List {
            page_size,
            page_token,
        } => {
            let page = client
                .list_sessions(*page_size, page_token.as_deref())
                .await?;
            let view = SessionListView {
                sessions: page.items().iter().map(SessionView::from).collect(),
                next_page_token: page.next_page_token().map(str::to_string),
            };
            Ok(view.render(format)?)
        }
        SessionsCommand::Get { id } => {
            let session = client.get_session(&session_resource_name(id)).await?;
            Ok(SessionView::from(&session).render(format)?)
        }
    }
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
    async fn list_renders_sessions() {
        let server = MockServer::respond_once(
            200,
            r#"{
                "sessions": [
                    {"id": "1", "name": "sessions/1", "title": "First", "state": "QUEUED"}
                ],
                "nextPageToken": "page-2"
            }"#,
        )
        .await;
        let client = client_for(&server);
        let args = SessionsArgs {
            command: SessionsCommand::List {
                page_size: Some(10),
                page_token: None,
            },
        };

        let output = handle(&client, &args, OutputFormat::Plain).await.unwrap();
        assert!(output.contains("First"));
        assert!(output.contains("page-2"));

        let request = server.received().await;
        assert!(request.path.starts_with("/v1alpha/sessions?"));
        assert!(request.path.contains("pageSize=10"));
    }

    /// `page_size` is already asserted by `list_renders_sessions`; `page_token` was never
    /// separately asserted, so a bug that dropped or mis-serialized it would not be caught.
    #[tokio::test]
    async fn list_forwards_page_token() {
        let server = MockServer::respond_once(200, r#"{"sessions": []}"#).await;
        let client = client_for(&server);
        let args = SessionsArgs {
            command: SessionsCommand::List {
                page_size: None,
                page_token: Some("tok-1".to_string()),
            },
        };

        handle(&client, &args, OutputFormat::Plain).await.unwrap();

        let request = server.received().await;
        assert!(request.path.contains("pageToken=tok-1"));
    }

    /// Proves a non-2xx API response is mapped into `CliError::Sdk` via `?` propagation,
    /// rather than being silently swallowed or panicking.
    #[tokio::test]
    async fn get_error_response_maps_to_sdk_error() {
        let server =
            MockServer::respond_once(404, r#"{"error": {"message": "session not found"}}"#).await;
        let client = client_for(&server);
        let args = SessionsArgs {
            command: SessionsCommand::Get {
                id: "missing".to_string(),
            },
        };

        let err = handle(&client, &args, OutputFormat::Plain)
            .await
            .unwrap_err();
        assert!(matches!(err, CliError::Sdk(_)));
    }

    #[tokio::test]
    async fn get_qualifies_bare_id() {
        let server = MockServer::respond_once(
            200,
            r#"{"id": "1", "name": "sessions/1", "title": "Example", "state": "QUEUED"}"#,
        )
        .await;
        let client = client_for(&server);
        let args = SessionsArgs {
            command: SessionsCommand::Get {
                id: "1".to_string(),
            },
        };

        let output = handle(&client, &args, OutputFormat::Plain).await.unwrap();
        assert!(output.contains("Example"));

        let request = server.received().await;
        assert!(request.path.ends_with("/v1alpha/sessions/1"));
    }
}
