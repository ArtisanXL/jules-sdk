//! `config` subcommand: inspect or persist local CLI configuration.

use std::path::Path;

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
#[derive(Subcommand)]
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

impl std::fmt::Debug for ConfigCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Show => write!(f, "Show"),
            Self::Set { api_key, base_url } => f
                .debug_struct("Set")
                .field("api_key", &api_key.as_ref().map(|_| "***REDACTED***"))
                .field("base_url", base_url)
                .finish(),
        }
    }
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
/// `config_dir` overrides where the config file is read from/written to; production callers
/// pass `None` (the real OS config directory). This is the hook tests use to avoid touching
/// the real user's home directory.
///
/// # Errors
///
/// Returns a [`CliError`] if the config file cannot be loaded or saved, or rendering fails.
pub fn handle(
    args: &ConfigArgs,
    format: crate::utils::OutputFormat,
    config_dir: Option<&Path>,
) -> Result<String, CliError> {
    match &args.command {
        ConfigCommand::Show => {
            let resolved = config::resolve(config_dir, None, None)?;
            Ok(ConfigView::from(&resolved).render(format)?)
        }
        ConfigCommand::Set { api_key, base_url } => {
            let mut current = config::load_file(config_dir)?;
            if let Some(api_key) = api_key {
                current.api_key = Some(api_key.clone());
            }
            if let Some(base_url) = base_url {
                current.base_url = Some(base_url.clone());
            }
            config::save_file(config_dir, &current)?;
            Ok(ConfigView::from(&current).render(format)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::OutputFormat;
    use std::path::PathBuf;

    fn temp_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "jules-cli-config-cmd-test-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let mut builder = std::fs::DirBuilder::new();
        builder.recursive(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder.create(&dir).unwrap();
        dir
    }

    /// Proves that `ConfigCommand::Set`'s merge logic preserves a previously-set field when a
    /// later `Set` call only supplies the other field, rather than clobbering it with `None`.
    #[test]
    fn set_preserves_unset_fields() {
        let dir = temp_dir();

        let set_api_key = ConfigArgs {
            command: ConfigCommand::Set {
                api_key: Some("secret-key".to_string()),
                base_url: None,
            },
        };
        handle(&set_api_key, OutputFormat::Plain, Some(&dir)).unwrap();

        let set_base_url = ConfigArgs {
            command: ConfigCommand::Set {
                api_key: None,
                base_url: Some("https://example.test".to_string()),
            },
        };
        handle(&set_base_url, OutputFormat::Plain, Some(&dir)).unwrap();

        let stored = config::load_file(Some(&dir)).unwrap();
        assert_eq!(stored.api_key.as_deref(), Some("secret-key"));
        assert_eq!(stored.base_url.as_deref(), Some("https://example.test"));
    }

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

#[test]
fn test_config_args_debug() {
    let args = ConfigArgs {
        command: ConfigCommand::Set {
            api_key: Some("my_super_secret_api_key".to_string()),
            base_url: None,
        },
    };
    let output = format!("{:?}", args);
    assert!(!output.contains("my_super_secret_api_key"));
}
