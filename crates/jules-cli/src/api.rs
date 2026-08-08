//! Helpers for building a live Jules API client from resolved CLI configuration.

use jules_sdk::jules_api::auth::AuthType;
use jules_sdk::{JulesClient, JulesClientBuilder};

use crate::config::CliConfig;
use crate::error::CliError;

/// Builds a live [`JulesClient`] from the resolved CLI configuration.
///
/// # Errors
///
/// Returns [`CliError::MissingApiKey`] if no API key is configured.
pub fn build_client(config: &CliConfig) -> Result<JulesClient, CliError> {
    let api_key = config.api_key.clone().ok_or(CliError::MissingApiKey)?;

    let mut builder = JulesClientBuilder::new().auth(AuthType::google_api_key(api_key));
    if let Some(base_url) = &config.base_url {
        builder = builder.base_url(base_url.clone());
    }

    Ok(builder.build()?)
}

/// Normalizes a session identifier into the full resource name the Jules API expects (e.g.
/// `sessions/1234567890`), passing already-qualified names through unchanged.
#[must_use]
pub fn session_resource_name(session: &str) -> String {
    if session.starts_with("sessions/") {
        session.to_string()
    } else {
        format!("sessions/{session}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_when_no_api_key_configured() {
        let config = CliConfig {
            api_key: None,
            base_url: None,
        };
        assert!(matches!(
            build_client(&config),
            Err(CliError::MissingApiKey)
        ));
    }

    #[test]
    fn builds_client_when_api_key_configured() {
        let config = CliConfig {
            api_key: Some("test-key".to_string()),
            base_url: Some("https://example.test".to_string()),
        };
        assert!(build_client(&config).is_ok());
    }

    #[test]
    fn session_resource_name_passes_through_qualified_names() {
        assert_eq!(session_resource_name("sessions/123"), "sessions/123");
    }

    #[test]
    fn session_resource_name_qualifies_bare_ids() {
        assert_eq!(session_resource_name("123"), "sessions/123");
    }
}
