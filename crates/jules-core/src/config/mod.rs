//! Config module.

use std::error::Error;
use std::fmt;

/// An error that can occur when building a [`Config`].
#[derive(Debug)]
pub struct ConfigBuildError(String);

impl fmt::Display for ConfigBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config build error: {}", self.0)
    }
}

impl Error for ConfigBuildError {}

/// The configuration used to build a client.
#[derive(Clone)]
pub struct Config {
    api_key: String,
    timeout: Option<u64>,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("api_key", &"***REDACTED***")
            .field("timeout", &self.timeout)
            .finish()
    }
}

impl Config {
    /// Creates a new [`ConfigBuilder`] to construct a [`Config`].
    #[must_use]
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// Returns the API key.
    #[must_use]
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Returns the timeout in seconds, if configured.
    #[must_use]
    pub fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

/// A builder for constructing a [`Config`].
#[derive(Default)]
pub struct ConfigBuilder {
    api_key: Option<String>,
    timeout: Option<u64>,
}

impl fmt::Debug for ConfigBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConfigBuilder")
            .field("api_key", &self.api_key.as_ref().map(|_| "***REDACTED***"))
            .field("timeout", &self.timeout)
            .finish()
    }
}

impl ConfigBuilder {
    /// Sets the API key for the configuration.
    #[must_use]
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the timeout in seconds.
    #[must_use]
    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout = Some(seconds);
        self
    }

    /// Builds the [`Config`] from the provided configuration.
    ///
    /// # Errors
    ///
    /// Returns a [`ConfigBuildError`] if a required field is missing.
    pub fn build(self) -> Result<Config, ConfigBuildError> {
        let api_key = self
            .api_key
            .ok_or_else(|| ConfigBuildError("missing required field: api_key".to_string()))?;

        Ok(Config {
            api_key,
            timeout: self.timeout,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder_with_all_fields() {
        let config = Config::builder()
            .api_key("test_key")
            .timeout(30)
            .build()
            .unwrap();

        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.timeout(), Some(30));
    }

    #[test]
    fn test_config_builder_missing_api_key() {
        let result = Config::builder().timeout(30).build();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.0, "missing required field: api_key");
    }

    #[test]
    fn test_config_builder_only_api_key() {
        let config = Config::builder().api_key("test_key").build().unwrap();

        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.timeout(), None);
    }
}
