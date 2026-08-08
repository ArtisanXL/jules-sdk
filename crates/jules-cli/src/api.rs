//! Helpers for building a live Jules API client from resolved CLI configuration.

use jules_api::auth::AuthType;
use jules_api::client::JulesClient;
use jules_api::http::reqwest_transport::ReqwestTransport;

use crate::config::CliConfig;
use crate::error::CliError;

/// Builds a live [`JulesClient`] backed by [`ReqwestTransport`] from the
/// resolved CLI configuration.
///
/// # Errors
///
/// Returns [`CliError::MissingApiKey`] if no API key is configured.
pub fn build_client(config: &CliConfig) -> Result<JulesClient<ReqwestTransport>, CliError> {
    let api_key = config.api_key.clone().ok_or(CliError::MissingApiKey)?;
    let auth = AuthType::jules_api_key(api_key);
    let transport = ReqwestTransport::new();
    Ok(match &config.base_url {
        Some(base_url) => JulesClient::with_base_url(transport, base_url.clone(), auth),
        None => JulesClient::new(transport, auth),
    })
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
}
