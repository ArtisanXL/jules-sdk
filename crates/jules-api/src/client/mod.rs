//! Client module.
//!
//! Provides [`JulesClient`], a thin client for the Jules `v1alpha` REST API.
//! Endpoint methods are implemented on `JulesClient` in the sibling
//! [`crate::session`], [`crate::activity`], and [`crate::source`] modules.

use crate::auth::AuthType;
use crate::http::endpoint::Endpoint;
use crate::http::{HttpRequest, Transport};
use crate::response::{check_status, deserialize_json};
use jules_core::errors::SDKError;
use serde::de::DeserializeOwned;

/// The default base URL for the Jules `v1alpha` REST API.
pub const DEFAULT_BASE_URL: &str = "https://jules.googleapis.com/v1alpha";

/// A client for the Jules `v1alpha` REST API.
///
/// Generic over the [`Transport`] used to send HTTP requests, so callers can
/// plug in any transport implementation (e.g. a `reqwest`-backed one, or a
/// mock transport for testing).
pub struct JulesClient<T: Transport> {
    pub(crate) transport: T,
    pub(crate) base_url: String,
    pub(crate) auth: AuthType,
}

impl<T: Transport> JulesClient<T> {
    /// Creates a new client targeting the default Jules `v1alpha` base URL
    /// (`https://jules.googleapis.com/v1alpha`).
    #[must_use]
    pub fn new(transport: T, auth: AuthType) -> Self {
        Self::with_base_url(transport, DEFAULT_BASE_URL, auth)
    }

    /// Creates a new client targeting a custom base URL.
    ///
    /// Useful for pointing at a proxy or a test server.
    #[must_use]
    pub fn with_base_url(transport: T, base_url: impl Into<String>, auth: AuthType) -> Self {
        Self {
            transport,
            base_url: base_url.into(),
            auth,
        }
    }

    /// Creates a new [`Endpoint`] rooted at this client's base URL.
    pub(crate) fn endpoint(&self, path: impl Into<String>) -> Endpoint {
        Endpoint::new(self.base_url.clone(), path)
    }

    /// Applies this client's authentication to a request.
    fn authenticate(&self, request: HttpRequest) -> HttpRequest {
        self.auth.clone().apply(request)
    }

    /// Sends `request`, applying authentication, and deserializes the JSON response body.
    ///
    /// # Errors
    /// Returns an [`SDKError`] if the transport fails, the response status is
    /// not successful, or the response body cannot be deserialized.
    pub(crate) async fn send_json<R: DeserializeOwned>(
        &self,
        request: HttpRequest,
    ) -> Result<R, SDKError> {
        let request = self.authenticate(request);
        let response = self.transport.send(request).await?;
        deserialize_json(&response)
    }

    /// Sends `request`, applying authentication, and validates the response status
    /// without deserializing a body.
    ///
    /// # Errors
    /// Returns an [`SDKError`] if the transport fails or the response status is not successful.
    pub(crate) async fn send_status(&self, request: HttpRequest) -> Result<(), SDKError> {
        let request = self.authenticate(request);
        let response = self.transport.send(request).await?;
        check_status(&response)
    }
}
