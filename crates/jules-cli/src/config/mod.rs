//! Local CLI configuration: loading, saving, and env/flag overrides.

use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// An error that can occur while loading or saving [`CliConfig`].
#[derive(Debug)]
pub enum ConfigError {
    /// No config directory could be located for the current user.
    NoConfigDir,
    /// Reading or writing the config file failed.
    Io(std::io::Error),
    /// The config file's contents were not valid JSON.
    Json(serde_json::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoConfigDir => write!(f, "could not locate a config directory for this user"),
            Self::Io(err) => write!(f, "failed to access config file: {err}"),
            Self::Json(err) => write!(f, "failed to parse config file: {err}"),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NoConfigDir => None,
            Self::Io(err) => Some(err),
            Self::Json(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

/// Local CLI configuration, persisted as JSON.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliConfig {
    /// The Jules API key to authenticate requests with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// An override for the Jules API base URL (defaults to the real API if unset).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
}

/// Resolves the path to the CLI's config file.
///
/// If `config_dir` is `Some`, it is used directly (this is the hook tests use to avoid
/// touching the real user's home directory). Otherwise, the path is derived from
/// [`dirs::config_dir`] joined with `jules-cli/config.json`.
///
/// # Errors
///
/// Returns [`ConfigError::NoConfigDir`] if `config_dir` is `None` and the platform config
/// directory cannot be determined.
pub fn config_file_path(config_dir: Option<&Path>) -> Result<PathBuf, ConfigError> {
    let dir = match config_dir {
        Some(dir) => dir.to_path_buf(),
        None => dirs::config_dir()
            .ok_or(ConfigError::NoConfigDir)?
            .join("jules-cli"),
    };
    Ok(dir.join("config.json"))
}

/// Loads [`CliConfig`] from the config file, returning [`CliConfig::default`] if the file
/// does not exist yet.
///
/// # Errors
///
/// Returns a [`ConfigError`] if the config directory cannot be located, the file exists but
/// cannot be read, or its contents are not valid JSON.
pub fn load_file(config_dir: Option<&Path>) -> Result<CliConfig, ConfigError> {
    let path = config_file_path(config_dir)?;
    if !path.exists() {
        return Ok(CliConfig::default());
    }
    let contents = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&contents)?)
}

/// Persists `config` to the config file, creating its parent directory if needed.
///
/// # Errors
///
/// Returns a [`ConfigError`] if the config directory cannot be located or created, or the
/// file cannot be written.
pub fn save_file(config_dir: Option<&Path>, config: &CliConfig) -> Result<(), ConfigError> {
    let path = config_file_path(config_dir)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let contents = serde_json::to_string_pretty(config)?;
    std::fs::write(path, contents)?;
    Ok(())
}

/// Resolves the effective [`CliConfig`], applying (highest to lowest precedence):
/// explicit `api_key`/`base_url` overrides (e.g. from CLI flags), then the
/// `JULES_API_KEY`/`JULES_BASE_URL` environment variables, then the config file, then
/// defaults (`None`/`None`).
///
/// # Errors
///
/// Returns a [`ConfigError`] if the config file exists but cannot be loaded.
pub fn resolve(
    config_dir: Option<&Path>,
    api_key_override: Option<String>,
    base_url_override: Option<String>,
) -> Result<CliConfig, ConfigError> {
    let file = load_file(config_dir)?;

    let api_key = api_key_override
        .or_else(|| std::env::var("JULES_API_KEY").ok())
        .or(file.api_key);
    let base_url = base_url_override
        .or_else(|| std::env::var("JULES_BASE_URL").ok())
        .or(file.base_url);

    Ok(CliConfig { api_key, base_url })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "jules-cli-test-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn load_file_defaults_when_missing() {
        let dir = temp_dir();
        let config = load_file(Some(&dir)).unwrap();
        assert_eq!(config, CliConfig::default());
    }

    #[test]
    fn save_then_load_round_trips() {
        let dir = temp_dir();
        let config = CliConfig {
            api_key: Some("test-key".to_string()),
            base_url: Some("https://example.test".to_string()),
        };
        save_file(Some(&dir), &config).unwrap();

        let loaded = load_file(Some(&dir)).unwrap();
        assert_eq!(loaded, config);
    }

    #[test]
    fn resolve_prefers_explicit_override_over_file() {
        let dir = temp_dir();
        save_file(
            Some(&dir),
            &CliConfig {
                api_key: Some("file-key".to_string()),
                base_url: None,
            },
        )
        .unwrap();

        let resolved = resolve(Some(&dir), Some("flag-key".to_string()), None).unwrap();
        assert_eq!(resolved.api_key.as_deref(), Some("flag-key"));
    }

    #[test]
    fn resolve_falls_back_to_file_when_no_override() {
        let dir = temp_dir();
        save_file(
            Some(&dir),
            &CliConfig {
                api_key: Some("file-key".to_string()),
                base_url: Some("https://file.example".to_string()),
            },
        )
        .unwrap();

        let resolved = resolve(Some(&dir), None, None).unwrap();
        assert_eq!(resolved.api_key.as_deref(), Some("file-key"));
        assert_eq!(resolved.base_url.as_deref(), Some("https://file.example"));
    }
}
