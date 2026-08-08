//! `sources` subcommand group: list Jules sources.

use clap::{Args, Subcommand};
use jules_api::client::JulesClient;
use jules_api::http::Transport;
use jules_api::source::ListSourcesParams;
use serde::Serialize;

use crate::commands::view::SourceView;
use crate::error::CliError;
use crate::utils::{OutputFormat, Render};

/// Arguments for the `sources` subcommand.
#[derive(Debug, Args)]
pub struct SourcesArgs {
    /// The sources action to perform.
    #[command(subcommand)]
    pub action: SourcesAction,
}

/// Actions supported by the `sources` subcommand.
#[derive(Debug, Subcommand)]
pub enum SourcesAction {
    /// List sources.
    List {
        /// A filter expression restricting which sources are returned.
        #[arg(long)]
        filter: Option<String>,
        /// The maximum number of sources to return.
        #[arg(long)]
        page_size: Option<u32>,
    },
}

/// The rendered result of a `sources list` subcommand invocation.
#[derive(Debug, Serialize)]
pub struct SourcesListResult {
    /// The returned sources.
    pub sources: Vec<SourceView>,
}

impl Render for SourcesListResult {
    fn render_plain(&self) -> String {
        if self.sources.is_empty() {
            return "No sources found.".to_string();
        }
        self.sources
            .iter()
            .map(SourceView::render_plain_line)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Handles the `sources` subcommand.
///
/// # Errors
///
/// Returns an error if the Jules API request fails or rendering the result
/// fails.
pub async fn handle<T: Transport>(
    client: &JulesClient<T>,
    args: &SourcesArgs,
    format: OutputFormat,
) -> Result<String, CliError> {
    match &args.action {
        SourcesAction::List { filter, page_size } => {
            let params = ListSourcesParams {
                page_size: *page_size,
                page_token: None,
                filter: filter.clone(),
            };
            let response = client.list_sources(&params).await?;
            let sources = response.sources.iter().map(SourceView::from).collect();
            Ok(SourcesListResult { sources }.render(format)?)
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

    fn source_json() -> &'static str {
        r#"{
            "name": "sources/github/owner/repo",
            "id": "github/owner/repo",
            "githubRepo": {
                "owner": "owner",
                "repo": "repo",
                "isPrivate": false
            }
        }"#
    }

    #[tokio::test]
    async fn list_renders_sources_in_plain_and_json() {
        let args = SourcesArgs {
            action: SourcesAction::List {
                filter: None,
                page_size: None,
            },
        };

        let body = format!(r#"{{ "sources": [{}] }}"#, source_json());
        let (client, mock) = make_client(HttpResponse::new(200, vec![], body.clone().into_bytes()));
        let plain = handle(&client, &args, OutputFormat::Plain).await.unwrap();
        assert!(plain.contains("github/owner/repo"));
        assert!(plain.contains("owner"));
        assert!(plain.contains("repo"));
        let sent = mock.last_request().unwrap();
        assert_eq!(sent.url, "https://jules.googleapis.com/v1alpha/sources");

        let (client, _mock) = make_client(HttpResponse::new(200, vec![], body.into_bytes()));
        let json = handle(&client, &args, OutputFormat::Json).await.unwrap();
        assert!(json.contains("\"owner\": \"owner\""));
        assert!(json.contains("\"repo\": \"repo\""));
    }

    #[tokio::test]
    async fn list_renders_empty_result() {
        let (client, _mock) = make_client(HttpResponse::new(
            200,
            vec![],
            b"{ \"sources\": [] }".to_vec(),
        ));
        let args = SourcesArgs {
            action: SourcesAction::List {
                filter: None,
                page_size: None,
            },
        };

        let plain = handle(&client, &args, OutputFormat::Plain).await.unwrap();
        assert_eq!(plain, "No sources found.");
    }
}
