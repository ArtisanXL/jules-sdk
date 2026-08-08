//! `sessions` subcommand group: list and inspect Jules sessions.

use clap::{Args, Subcommand};
use jules_api::client::JulesClient;
use jules_api::http::Transport;
use jules_api::session::ListSessionsParams;
use serde::Serialize;

use crate::commands::view::SessionView;
use crate::error::CliError;
use crate::utils::{OutputFormat, Render};

/// Arguments for the `sessions` subcommand.
#[derive(Debug, Args)]
pub struct SessionsArgs {
    /// The sessions action to perform.
    #[command(subcommand)]
    pub action: SessionsAction,
}

/// Actions supported by the `sessions` subcommand.
#[derive(Debug, Subcommand)]
pub enum SessionsAction {
    /// List sessions.
    List {
        /// A filter expression restricting which sessions are returned.
        #[arg(long)]
        filter: Option<String>,
        /// The maximum number of sessions to return.
        #[arg(long)]
        page_size: Option<u32>,
    },
    /// Fetch a single session by id.
    Get {
        /// The session id.
        id: String,
    },
}

/// The rendered result of a `sessions list` subcommand invocation.
#[derive(Debug, Serialize)]
pub struct SessionsListResult {
    /// The returned sessions.
    pub sessions: Vec<SessionView>,
}

impl Render for SessionsListResult {
    fn render_plain(&self) -> String {
        if self.sessions.is_empty() {
            return "No sessions found.".to_string();
        }
        self.sessions
            .iter()
            .map(SessionView::render_plain_line)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Handles the `sessions` subcommand.
///
/// # Errors
///
/// Returns an error if the Jules API request fails or rendering the result
/// fails.
pub async fn handle<T: Transport>(
    client: &JulesClient<T>,
    args: &SessionsArgs,
    format: OutputFormat,
) -> Result<String, CliError> {
    match &args.action {
        SessionsAction::List { filter, page_size } => {
            let params = ListSessionsParams {
                page_size: *page_size,
                page_token: None,
                filter: filter.clone(),
            };
            let response = client.list_sessions(&params).await?;
            let sessions = response.sessions.iter().map(SessionView::from).collect();
            Ok(SessionsListResult { sessions }.render(format)?)
        }
        SessionsAction::Get { id } => {
            let session = client.get_session(id).await?;
            Ok(SessionView::from(&session).render(format)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::MockTransport;
    use jules_api::auth::AuthType;
    use jules_api::http::HttpResponse;

    fn make_client(response: HttpResponse) -> (JulesClient<MockTransport>, MockTransport) {
        let mock = MockTransport::new(response);
        let client = JulesClient::new(mock.clone(), AuthType::jules_api_key("k"));
        (client, mock)
    }

    fn session_json() -> &'static str {
        r#"{
            "name": "sessions/12345",
            "id": "12345",
            "prompt": "Fix the bug",
            "sourceContext": { "source": "sources/github/owner/repo" },
            "title": "Fix the bug",
            "createTime": "2026-08-08T00:00:00Z",
            "updateTime": "2026-08-08T00:00:00Z",
            "state": "IN_PROGRESS",
            "url": "https://jules.google.com/session/12345"
        }"#
    }

    #[tokio::test]
    async fn list_renders_sessions_and_sends_query_params() {
        let body = format!(r#"{{ "sessions": [{}] }}"#, session_json());
        let (client, mock) = make_client(HttpResponse::new(200, vec![], body.into_bytes()));
        let args = SessionsArgs {
            action: SessionsAction::List {
                filter: Some("state=IN_PROGRESS".to_string()),
                page_size: Some(5),
            },
        };

        let plain = handle(&client, &args, OutputFormat::Plain).await.unwrap();
        assert!(plain.contains("12345"));
        assert!(plain.contains("InProgress"));

        let sent = mock.last_request().unwrap();
        assert!(sent.url.contains("pageSize=5"));
        assert!(sent.url.contains("filter=state=IN_PROGRESS"));
    }

    #[tokio::test]
    async fn list_renders_empty_result() {
        let (client, _mock) = make_client(HttpResponse::new(
            200,
            vec![],
            b"{ \"sessions\": [] }".to_vec(),
        ));
        let args = SessionsArgs {
            action: SessionsAction::List {
                filter: None,
                page_size: None,
            },
        };

        let plain = handle(&client, &args, OutputFormat::Plain).await.unwrap();
        assert_eq!(plain, "No sessions found.");
    }

    #[tokio::test]
    async fn get_renders_single_session() {
        let (client, mock) = make_client(HttpResponse::new(200, vec![], session_json().into()));
        let args = SessionsArgs {
            action: SessionsAction::Get {
                id: "12345".to_string(),
            },
        };

        let json = handle(&client, &args, OutputFormat::Json).await.unwrap();
        assert!(json.contains("\"id\": \"12345\""));
        assert!(json.contains("\"state\": \"InProgress\""));

        let sent = mock.last_request().unwrap();
        assert_eq!(
            sent.url,
            "https://jules.googleapis.com/v1alpha/sessions/12345"
        );
    }
}
