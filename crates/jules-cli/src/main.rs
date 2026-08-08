//! Command line interface for Jules-SDK.

#![deny(missing_docs)]

pub mod api;
pub mod commands;
pub mod config;
pub mod diagnostics;
pub mod error;
pub mod interactive;
#[cfg(test)]
mod test_support;
pub mod utils;

use clap::{Parser, Subcommand};

use commands::chat::{self, ChatArgs};
use commands::config::{self as config_cmd, ConfigArgs};
use commands::sessions::{self, SessionsArgs};
use commands::sources::{self, SourcesArgs};
use error::CliError;
use utils::OutputFormat;

/// Command line interface for the Jules SDK.
#[derive(Debug, Parser)]
#[command(name = "jules-cli", version, about, propagate_version = true)]
pub struct Cli {
    /// The output format to render results in.
    #[arg(long, global = true, value_enum, default_value_t = OutputFormat::Plain)]
    pub format: OutputFormat,

    /// The subcommand to run.
    #[command(subcommand)]
    pub command: Commands,
}

/// Top-level subcommands.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Inspect or persist local CLI configuration.
    Config(ConfigArgs),
    /// Create a new Jules session, or send a message to an existing one.
    Chat(ChatArgs),
    /// List or inspect Jules sessions.
    Sessions(SessionsArgs),
    /// List Jules sources.
    Sources(SourcesArgs),
}

/// Dispatches to the handler for `cli.command`, resolving local configuration
/// and (for network subcommands) building a live Jules API client first.
async fn run(cli: &Cli) -> Result<String, CliError> {
    match &cli.command {
        Commands::Config(args) => Ok(config_cmd::handle(args, cli.format)?),
        Commands::Chat(args) => {
            let config = config::load(None, None, None)?;
            let client = api::build_client(&config)?;
            chat::handle(&client, args, cli.format).await
        }
        Commands::Sessions(args) => {
            let config = config::load(None, None, None)?;
            let client = api::build_client(&config)?;
            sessions::handle(&client, args, cli.format).await
        }
        Commands::Sources(args) => {
            let config = config::load(None, None, None)?;
            let client = api::build_client(&config)?;
            sources::handle(&client, args, cli.format).await
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match run(&cli).await {
        Ok(output) => println!("{output}"),
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use commands::config::ConfigAction;

    #[test]
    fn parses_chat_command_with_default_format() {
        let cli = Cli::try_parse_from(["jules-cli", "chat", "hello"]).unwrap();
        assert_eq!(cli.format, OutputFormat::Plain);
        match cli.command {
            Commands::Chat(args) => assert_eq!(args.message, "hello"),
            other => panic!("expected Chat command, got {other:?}"),
        }
    }

    #[test]
    fn parses_global_format_flag_before_subcommand() {
        let cli = Cli::try_parse_from(["jules-cli", "--format", "json", "chat", "hi"]).unwrap();
        assert_eq!(cli.format, OutputFormat::Json);
    }

    #[test]
    fn parses_config_show() {
        let cli = Cli::try_parse_from(["jules-cli", "config", "show"]).unwrap();
        match cli.command {
            Commands::Config(args) => assert!(matches!(args.action, ConfigAction::Show)),
            other => panic!("expected Config command, got {other:?}"),
        }
    }

    #[test]
    fn parses_config_set_with_both_flags() {
        let cli = Cli::try_parse_from([
            "jules-cli",
            "config",
            "set",
            "--api-key",
            "abc123",
            "--base-url",
            "https://example.test",
        ])
        .unwrap();
        match cli.command {
            Commands::Config(args) => match args.action {
                ConfigAction::Set { api_key, base_url } => {
                    assert_eq!(api_key.as_deref(), Some("abc123"));
                    assert_eq!(base_url.as_deref(), Some("https://example.test"));
                }
                ConfigAction::Show => panic!("expected Set action"),
            },
            other => panic!("expected Config command, got {other:?}"),
        }
    }

    #[test]
    fn parses_chat_with_session_and_source_flags() {
        let cli = Cli::try_parse_from([
            "jules-cli",
            "chat",
            "keep going",
            "--session",
            "12345",
            "--source",
            "owner/repo",
            "--branch",
            "main",
            "--title",
            "My title",
        ])
        .unwrap();
        match cli.command {
            Commands::Chat(args) => {
                assert_eq!(args.message, "keep going");
                assert_eq!(args.session.as_deref(), Some("12345"));
                assert_eq!(args.source.as_deref(), Some("owner/repo"));
                assert_eq!(args.branch.as_deref(), Some("main"));
                assert_eq!(args.title.as_deref(), Some("My title"));
            }
            other => panic!("expected Chat command, got {other:?}"),
        }
    }

    #[test]
    fn parses_sessions_list() {
        let cli = Cli::try_parse_from([
            "jules-cli",
            "sessions",
            "list",
            "--filter",
            "state=QUEUED",
            "--page-size",
            "10",
        ])
        .unwrap();
        match cli.command {
            Commands::Sessions(args) => match args.action {
                sessions::SessionsAction::List { filter, page_size } => {
                    assert_eq!(filter.as_deref(), Some("state=QUEUED"));
                    assert_eq!(page_size, Some(10));
                }
                sessions::SessionsAction::Get { .. } => panic!("expected List action"),
            },
            other => panic!("expected Sessions command, got {other:?}"),
        }
    }

    #[test]
    fn parses_sessions_get() {
        let cli = Cli::try_parse_from(["jules-cli", "sessions", "get", "12345"]).unwrap();
        match cli.command {
            Commands::Sessions(args) => match args.action {
                sessions::SessionsAction::Get { id } => assert_eq!(id, "12345"),
                sessions::SessionsAction::List { .. } => panic!("expected Get action"),
            },
            other => panic!("expected Sessions command, got {other:?}"),
        }
    }

    #[test]
    fn parses_sources_list() {
        let cli = Cli::try_parse_from(["jules-cli", "sources", "list"]).unwrap();
        match cli.command {
            Commands::Sources(args) => match args.action {
                sources::SourcesAction::List { filter, page_size } => {
                    assert_eq!(filter, None);
                    assert_eq!(page_size, None);
                }
            },
            other => panic!("expected Sources command, got {other:?}"),
        }
    }

    #[test]
    fn rejects_unknown_subcommand() {
        assert!(Cli::try_parse_from(["jules-cli", "bogus"]).is_err());
    }

    #[test]
    fn rejects_chat_without_message() {
        assert!(Cli::try_parse_from(["jules-cli", "chat"]).is_err());
    }
}
