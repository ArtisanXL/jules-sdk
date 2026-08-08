//! `config` subcommand: inspect and persist local CLI configuration.

use clap::{Args, Subcommand};
use serde::Serialize;

use crate::config::{self, ConfigError};
use crate::utils::{OutputFormat, Render};

/// Arguments for the `config` subcommand.
#[derive(Debug, Args)]
pub struct ConfigArgs {
    /// The config action to perform.
    #[command(subcommand)]
    pub action: ConfigAction,
}

/// Actions supported by the `config` subcommand.
#[derive(Debug, Subcommand)]
pub enum ConfigAction {
    /// Show the resolved configuration.
    Show,
    /// Set configuration values and persist them to the config file.
    Set {
        /// The Jules API key to store.
        #[arg(long)]
        api_key: Option<String>,
        /// The Jules API base URL to store.
        #[arg(long)]
        base_url: Option<String>,
    },
}

/// The rendered result of a `config` subcommand invocation.
#[derive(Debug, Serialize)]
pub struct ConfigResult {
    /// `"REDACTED"` if an API key is configured, `None` otherwise.
    pub api_key: Option<String>,
    /// The configured base URL, if any.
    pub base_url: Option<String>,
    /// The output format used to render this result.
    pub format: OutputFormat,
}

impl Render for ConfigResult {
    fn render_plain(&self) -> String {
        format!(
            "api_key: {}\nbase_url: {}\nformat: {}",
            self.api_key.as_deref().unwrap_or("<not set>"),
            self.base_url.as_deref().unwrap_or("<not set>"),
            self.format
        )
    }
}

/// Handles the `config` subcommand.
///
/// # Errors
///
/// Returns an error if the config file cannot be resolved, read, parsed, or
/// (for `config set`) written, or if rendering the result as JSON fails.
pub fn handle(args: &ConfigArgs, format: OutputFormat) -> Result<String, ConfigError> {
    let config = match &args.action {
        ConfigAction::Show => config::load(None, None, None)?,
        ConfigAction::Set { api_key, base_url } => {
            let mut current = config::load_file(None)?;
            if let Some(api_key) = api_key {
                current.api_key = Some(api_key.clone());
            }
            if let Some(base_url) = base_url {
                current.base_url = Some(base_url.clone());
            }
            config::save_file(None, &current)?;
            current
        }
    };

    let result = ConfigResult {
        api_key: config.api_key.map(|_| "REDACTED".to_string()),
        base_url: config.base_url,
        format,
    };
    Ok(result.render(format)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_api_key_when_present() {
        let result = ConfigResult {
            api_key: Some("REDACTED".to_string()),
            base_url: Some("https://example.test".to_string()),
            format: OutputFormat::Plain,
        };
        let plain = result.render_plain();
        assert!(plain.contains("api_key: REDACTED"));
        assert!(!plain.contains("secret"));
    }

    #[test]
    fn shows_not_set_placeholders_when_absent() {
        let result = ConfigResult {
            api_key: None,
            base_url: None,
            format: OutputFormat::Plain,
        };
        let plain = result.render_plain();
        assert!(plain.contains("api_key: <not set>"));
        assert!(plain.contains("base_url: <not set>"));
    }
}
