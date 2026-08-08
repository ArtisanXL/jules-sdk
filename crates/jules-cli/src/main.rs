//! Command line interface for Jules-SDK.

#![deny(missing_docs)]

pub mod commands;
pub mod config;
pub mod diagnostics;
pub mod interactive;
pub mod utils;

use clap::{Parser, Subcommand};

use commands::chat::{self, ChatArgs};
use commands::config::{self as config_cmd, ConfigArgs};
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
    /// Build a local conversation from a single chat message.
    Chat(ChatArgs),
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Config(args) => {
            config_cmd::handle(args, cli.format).map_err(|err| err.to_string())
        }
        Commands::Chat(args) => chat::handle(args, cli.format).map_err(|err| err.to_string()),
    };

    match result {
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
            Commands::Config(_) => panic!("expected Chat command"),
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
            Commands::Chat(_) => panic!("expected Config command"),
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
            Commands::Chat(_) => panic!("expected Config command"),
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
