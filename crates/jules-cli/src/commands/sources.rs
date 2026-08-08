//! `sources` subcommand: list connected Jules sources (read-only).

use clap::{Args, Subcommand};
use jules_sdk::JulesClient;

use crate::commands::view::{SourceListView, SourceView};
use crate::error::CliError;
use crate::utils::{OutputFormat, Render};

/// Arguments for the `sources` subcommand.
#[derive(Debug, Args)]
pub struct SourcesArgs {
    /// The `sources` subcommand to run.
    #[command(subcommand)]
    pub command: SourcesCommand,
}

/// `sources` subcommands.
#[derive(Debug, Subcommand)]
pub enum SourcesCommand {
    /// Lists connected sources (e.g. GitHub repositories).
    List {
        /// The maximum number of sources to return.
        #[arg(long)]
        page_size: Option<i32>,
        /// A page token from a previous `sources list` response.
        #[arg(long)]
        page_token: Option<String>,
    },
}

/// Handles the `sources` subcommand.
///
/// # Errors
///
/// Returns a [`CliError`] if the Jules API request fails or rendering the result fails.
pub async fn handle(
    client: &JulesClient,
    args: &SourcesArgs,
    format: OutputFormat,
) -> Result<String, CliError> {
    let SourcesCommand::List {
        page_size,
        page_token,
    } = &args.command;
    let page = client
        .list_sources(*page_size, page_token.as_deref())
        .await?;
    let view = SourceListView {
        sources: page.items().iter().map(SourceView::from).collect(),
        next_page_token: page.next_page_token().map(str::to_string),
    };
    view.render(format)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::MockServer;
    use jules_sdk::jules_api::auth::AuthType;
    use jules_sdk::JulesClientBuilder;
    use std::time::Duration;

    #[tokio::test]
    async fn list_renders_sources() {
        let server = MockServer::respond_once(
            200,
            r#"{
                "sources": [
                    {
                        "id": "github/owner/repo",
                        "name": "sources/github/owner/repo",
                        "githubRepo": {"owner": "owner", "repo": "repo", "isPrivate": false}
                    }
                ]
            }"#,
        )
        .await;
        let client = JulesClientBuilder::new()
            .base_url(server.base_url())
            .timeout(Duration::from_secs(5))
            .auth(AuthType::Bearer("test-token".to_string()))
            .build()
            .unwrap();
        let args = SourcesArgs {
            command: SourcesCommand::List {
                page_size: None,
                page_token: None,
            },
        };

        let output = handle(&client, &args, OutputFormat::Plain).await.unwrap();
        assert!(output.contains("owner/repo"));

        let request = server.received().await;
        assert!(request.path.starts_with("/v1alpha/sources"));
    }
}
