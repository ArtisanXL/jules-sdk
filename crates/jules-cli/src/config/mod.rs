//! CLI configuration loading, resolution, and persistence.
//!
//! Configuration is resolved with the following precedence (highest wins):
//! explicit CLI flag values, environment variables (`JULES_API_KEY`,
//! `JULES_BASE_URL`), the on-disk JSON config file, and finally defaults
//! (`None`).

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Environment variable that overrides the Jules API key.
pub const API_KEY_ENV: &str = "JULES_API_KEY";

/// Environment variable that overrides the Jules API base URL.
pub const BASE_URL_ENV: &str = "JULES_BASE_URL";

/// Errors that can occur while loading or saving CLI configuration.
#[derive(Debug)]
pub enum ConfigError {
    /// The user config directory could not be determined.
    NoConfigDir,
    /// An I/O error occurred while reading or writing the config file.
    Io(std::io::Error),
    /// The config file contained invalid JSON.
    Json(serde_json::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoConfigDir => write!(f, "could not determine the user config directory"),
            Self::Io(err) => write!(f, "config file I/O error: {err}"),
            Self::Json(err) => write!(f, "invalid config file JSON: {err}"),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
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

/// Persisted CLI configuration.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliConfig {
    /// The Jules API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// The Jules API base URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
}

/// Returns the path to the config file.
///
/// If `config_dir` is `Some`, it is used directly (this is the hook tests
/// use to avoid touching the real user's home directory). If `None`, the
/// OS-specific per-user config directory is resolved via
/// [`dirs::config_dir`] and joined with `jules-cli`.
///
/// # Errors
///
/// Returns [`ConfigError::NoConfigDir`] if `config_dir` is `None` and the
/// OS-specific config directory cannot be determined.
pub fn config_file_path(config_dir: Option<&Path>) -> Result<PathBuf, ConfigError> {
    let dir = match config_dir {
        Some(dir) => dir.to_path_buf(),
        None => dirs::config_dir()
            .ok_or(ConfigError::NoConfigDir)?
            .join("jules-cli"),
    };
    Ok(dir.join("config.json"))
}

/// Reads the on-disk config file, returning [`CliConfig::default`] if it
/// does not exist.
///
/// # Errors
///
/// Returns an error if the config directory cannot be resolved, or if the
/// file exists but cannot be read or parsed as JSON.
pub fn load_file(config_dir: Option<&Path>) -> Result<CliConfig, ConfigError> {
    let path = config_file_path(config_dir)?;
    if !path.exists() {
        return Ok(CliConfig::default());
    }
    let contents = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&contents)?)
}

/// Writes `config` to the on-disk config file, creating parent directories
/// as needed.
///
/// # Errors
///
/// Returns an error if the config directory cannot be resolved, the parent
/// directory cannot be created, or the file cannot be written.
pub fn save_file(config_dir: Option<&Path>, config: &CliConfig) -> Result<(), ConfigError> {
    let path = config_file_path(config_dir)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let contents = serde_json::to_string_pretty(config)?;
    fs::write(path, contents)?;
    Ok(())
}

/// Resolves the effective CLI configuration, merging (highest precedence
/// first) `cli_api_key`/`cli_base_url`, the `JULES_API_KEY`/`JULES_BASE_URL`
/// environment variables, the on-disk config file, and finally defaults
/// (`None`).
///
/// # Errors
///
/// Returns an error if the config file exists but cannot be read or parsed.
pub fn load(
    config_dir: Option<&Path>,
    cli_api_key: Option<&str>,
    cli_base_url: Option<&str>,
) -> Result<CliConfig, ConfigError> {
    let file = load_file(config_dir)?;
    let api_key = cli_api_key
        .map(ToOwned::to_owned)
        .or_else(|| std::env::var(API_KEY_ENV).ok())
        .or(file.api_key);
    let base_url = cli_base_url
        .map(ToOwned::to_owned)
        .or_else(|| std::env::var(BASE_URL_ENV).ok())
        .or(file.base_url);
    Ok(CliConfig { api_key, base_url })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    /// Returns a fresh, unused directory path under the system temp
    /// directory, so each test gets its own isolated config location.
    fn unique_temp_dir(label: &str) -> PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time is after the unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("jules-cli-test-{label}-{nanos}-{id}"))
    }

    #[test]
    fn load_file_defaults_when_missing() {
        let dir = unique_temp_dir("load-missing");
        let config = load_file(Some(&dir)).unwrap();
        assert_eq!(config, CliConfig::default());
    }

    #[test]
    fn save_and_load_file_round_trips() {
        let dir = unique_temp_dir("round-trip");
        let config = CliConfig {
            api_key: Some("secret".to_string()),
            base_url: Some("https://example.test".to_string()),
        };
        save_file(Some(&dir), &config).unwrap();
        let loaded = load_file(Some(&dir)).unwrap();
        assert_eq!(loaded, config);
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn config_file_path_joins_config_json() {
        let dir = unique_temp_dir("path");
        let path = config_file_path(Some(&dir)).unwrap();
        assert_eq!(path, dir.join("config.json"));
    }

    #[test]
    fn load_precedence_cli_env_file_default() {
        let dir = unique_temp_dir("precedence");

        // Defaults: nothing set anywhere.
        std::env::remove_var(API_KEY_ENV);
        std::env::remove_var(BASE_URL_ENV);
        let resolved = load(Some(&dir), None, None).unwrap();
        assert_eq!(resolved, CliConfig::default());

        // File sets both values.
        let file_config = CliConfig {
            api_key: Some("file-key".to_string()),
            base_url: Some("file-url".to_string()),
        };
        save_file(Some(&dir), &file_config).unwrap();
        let resolved = load(Some(&dir), None, None).unwrap();
        assert_eq!(resolved, file_config);

        // Env overrides the file.
        std::env::set_var(API_KEY_ENV, "env-key");
        std::env::set_var(BASE_URL_ENV, "env-url");
        let resolved = load(Some(&dir), None, None).unwrap();
        assert_eq!(resolved.api_key.as_deref(), Some("env-key"));
        assert_eq!(resolved.base_url.as_deref(), Some("env-url"));

        // CLI flags override env and file.
        let resolved = load(Some(&dir), Some("cli-key"), Some("cli-url")).unwrap();
        assert_eq!(resolved.api_key.as_deref(), Some("cli-key"));
        assert_eq!(resolved.base_url.as_deref(), Some("cli-url"));

        std::env::remove_var(API_KEY_ENV);
        std::env::remove_var(BASE_URL_ENV);
        fs::remove_dir_all(&dir).ok();
    }
}
