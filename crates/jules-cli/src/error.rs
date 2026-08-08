//! Unified error type for CLI subcommand handlers.

use jules_sdk::jules_core::errors::SDKError;

use crate::config::ConfigError;

/// Errors that can occur while running a CLI subcommand.
#[derive(Debug)]
pub enum CliError {
    /// No Jules API key is configured, so a network command cannot run.
    MissingApiKey,
    /// A CLI argument was invalid.
    InvalidArgument(String),
    /// Loading or saving local CLI configuration failed.
    Config(ConfigError),
    /// A Jules API request failed.
    Sdk(SDKError),
    /// Rendering the result as JSON failed.
    Json(serde_json::Error),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingApiKey => write!(
                f,
                "no Jules API key configured; run `jules-cli config set --api-key <KEY>` \
                 or set the JULES_API_KEY environment variable"
            ),
            Self::InvalidArgument(message) => write!(f, "{message}"),
            Self::Config(err) => write!(f, "{err}"),
            Self::Sdk(err) => write!(f, "{err}"),
            Self::Json(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for CliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::MissingApiKey | Self::InvalidArgument(_) => None,
            Self::Config(err) => Some(err),
            Self::Sdk(err) => Some(err),
            Self::Json(err) => Some(err),
        }
    }
}

impl From<ConfigError> for CliError {
    fn from(err: ConfigError) -> Self {
        Self::Config(err)
    }
}

impl From<SDKError> for CliError {
    fn from(err: SDKError) -> Self {
        Self::Sdk(err)
    }
}

impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}
