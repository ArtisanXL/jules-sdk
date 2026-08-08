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

async fn run(cli: &Cli) -> Result<String, CliError> {
    match &cli.command {
        Commands::Config(args) => config_cmd::handle(args, cli.format, None),
        Commands::Chat(args) => {
            let resolved = config::resolve(None, None, None)?;
            let client = api::build_client(&resolved)?;
            chat::handle(&client, args, cli.format).await
        }
        Commands::Sessions(args) => {
            let resolved = config::resolve(None, None, None)?;
            let client = api::build_client(&resolved)?;
            sessions::handle(&client, args, cli.format).await
        }
        Commands::Sources(args) => {
            let resolved = config::resolve(None, None, None)?;
            let client = api::build_client(&resolved)?;
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

    #[test]
    fn parses_chat_with_flags() {
        let cli = Cli::try_parse_from([
            "jules-cli",
            "--format",
            "json",
            "chat",
            "hello",
            "--source",
            "owner/repo",
            "--branch",
            "main",
        ])
        .unwrap();
        assert_eq!(cli.format, OutputFormat::Json);
        match cli.command {
            Commands::Chat(args) => {
                assert_eq!(args.message, "hello");
                assert_eq!(args.source.as_deref(), Some("owner/repo"));
                assert_eq!(args.branch.as_deref(), Some("main"));
            }
            other => panic!("expected Chat, got {other:?}"),
        }
    }

    #[test]
    fn parses_config_set() {
        let cli =
            Cli::try_parse_from(["jules-cli", "config", "set", "--api-key", "test-key"]).unwrap();
        match cli.command {
            Commands::Config(_) => {}
            other => panic!("expected Config, got {other:?}"),
        }
    }

    #[test]
    fn parses_sessions_get() {
        let cli = Cli::try_parse_from(["jules-cli", "sessions", "get", "12345"]).unwrap();
        match cli.command {
            Commands::Sessions(SessionsArgs {
                command: sessions::SessionsCommand::Get { id },
            }) => assert_eq!(id, "12345"),
            other => panic!("expected Sessions Get, got {other:?}"),
        }
    }

    #[test]
    fn parses_sources_list() {
        let cli = Cli::try_parse_from(["jules-cli", "sources", "list"]).unwrap();
        assert!(matches!(cli.command, Commands::Sources(_)));
    }

    #[test]
    fn rejects_missing_chat_message() {
        assert!(Cli::try_parse_from(["jules-cli", "chat"]).is_err());
    }

    #[test]
    fn default_format_is_plain() {
        let cli = Cli::try_parse_from(["jules-cli", "chat", "hi"]).unwrap();
        assert_eq!(cli.format, OutputFormat::Plain);
    }
}
