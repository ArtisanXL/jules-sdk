//! `config` subcommand: inspect or persist local CLI configuration.

use clap::{Args, Subcommand};
use serde::Serialize;

use crate::config::{self, CliConfig};
use crate::error::CliError;
use crate::utils::Render;

/// Arguments for the `config` subcommand.
#[derive(Debug, Args)]
pub struct ConfigArgs {
    /// The `config` subcommand to run.
    #[command(subcommand)]
    pub command: ConfigCommand,
}

/// `config` subcommands.
#[derive(Debug, Subcommand)]
pub enum ConfigCommand {
    /// Prints the resolved configuration (API key redacted).
    Show,
    /// Persists configuration values to the local config file.
    Set {
        /// The Jules API key to store.
        #[arg(long)]
        api_key: Option<String>,
        /// The Jules API base URL to store.
        #[arg(long)]
        base_url: Option<String>,
    },
}

/// A renderable view of the resolved configuration, with the API key redacted.
#[derive(Debug, Serialize)]
pub struct ConfigView {
    api_key: Option<&'static str>,
    base_url: Option<String>,
}

impl From<&CliConfig> for ConfigView {
    fn from(config: &CliConfig) -> Self {
        Self {
            api_key: config.api_key.as_ref().map(|_| "REDACTED"),
            base_url: config.base_url.clone(),
        }
    }
}

impl Render for ConfigView {
    fn render_plain(&self) -> String {
        format!(
            "api_key: {}\nbase_url: {}",
            self.api_key.unwrap_or("<not set>"),
            self.base_url.as_deref().unwrap_or("<not set>"),
        )
    }
}

/// Handles the `config` subcommand.
///
/// # Errors
///
/// Returns a [`CliError`] if the config file cannot be loaded or saved, or rendering fails.
pub fn handle(args: &ConfigArgs, format: crate::utils::OutputFormat) -> Result<String, CliError> {
    match &args.command {
        ConfigCommand::Show => {
            let resolved = config::resolve(None, None, None)?;
            Ok(ConfigView::from(&resolved).render(format)?)
        }
        ConfigCommand::Set { api_key, base_url } => {
            let mut current = config::load_file(None)?;
            if let Some(api_key) = api_key {
                current.api_key = Some(api_key.clone());
            }
            if let Some(base_url) = base_url {
                current.base_url = Some(base_url.clone());
            }
            config::save_file(None, &current)?;
            Ok(ConfigView::from(&current).render(format)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::OutputFormat;

    #[test]
    fn config_view_redacts_api_key() {
        let config = CliConfig {
            api_key: Some("secret".to_string()),
            base_url: None,
        };
        let view = ConfigView::from(&config);
        let rendered = view.render(OutputFormat::Plain).unwrap();
        assert!(!rendered.contains("secret"));
        assert!(rendered.contains("REDACTED"));
    }

    #[test]
    fn config_view_shows_not_set_when_empty() {
        let config = CliConfig::default();
        let view = ConfigView::from(&config);
        let rendered = view.render(OutputFormat::Plain).unwrap();
        assert!(rendered.contains("<not set>"));
    }
}
