//! Auth module.

use crate::http::HttpRequest;

/// The type of authentication to use.
#[derive(Clone, PartialEq, Eq, Default)]
pub enum AuthType {
    /// Bearer token authentication (e.g. `OAuth2` or JWT).
    Bearer(String),
    /// API key authentication, usually provided in a custom header.
    ApiKey {
        /// The header name (e.g., "x-api-key").
        header: String,
        /// The API key value.
        key: String,
    },
    /// Custom header authentication.
    Custom {
        /// The header name.
        header: String,
        /// The header value.
        value: String,
    },
    /// No authentication.
    #[default]
    None,
}

impl std::fmt::Debug for AuthType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bearer(_) => f.debug_tuple("Bearer").field(&"***REDACTED***").finish(),
            Self::ApiKey { header, key: _ } => f
                .debug_struct("ApiKey")
                .field("header", header)
                .field("key", &"***REDACTED***")
                .finish(),
            Self::Custom { header, value: _ } => f
                .debug_struct("Custom")
                .field("header", header)
                .field("value", &"***REDACTED***")
                .finish(),
            Self::None => write!(f, "None"),
        }
    }
}

impl AuthType {
    /// Creates the authentication used by the Jules `v1alpha` REST API: an API
    /// key sent via the `X-Goog-Api-Key` header.
    #[must_use]
    pub fn jules_api_key(key: impl Into<String>) -> Self {
        Self::ApiKey {
            header: "X-Goog-Api-Key".to_string(),
            key: key.into(),
        }
    }

    /// Applies the authentication to the given HTTP request.
    #[must_use]
    pub fn apply(self, request: HttpRequest) -> HttpRequest {
        match self {
            Self::Bearer(token) => request.with_header("Authorization", format!("Bearer {token}")),
            Self::ApiKey { header, key } => request.with_header(header, key),
            Self::Custom { header, value } => request.with_header(header, value),
            Self::None => request,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::Method;

    #[test]
    fn test_auth_type_bearer() {
        let auth = AuthType::Bearer("secret-token".to_string());
        let req = HttpRequest::new(Method::Get, "https://api.example.com");
        let req = auth.apply(req);

        assert_eq!(req.headers.len(), 1);
        assert_eq!(req.headers[0].0, "Authorization");
        assert_eq!(req.headers[0].1, "Bearer secret-token");
    }

    #[test]
    fn test_auth_type_api_key() {
        let auth = AuthType::ApiKey {
            header: "x-api-key".to_string(),
            key: "my-key".to_string(),
        };
        let req = HttpRequest::new(Method::Get, "https://api.example.com");
        let req = auth.apply(req);

        assert_eq!(req.headers.len(), 1);
        assert_eq!(req.headers[0].0, "x-api-key");
        assert_eq!(req.headers[0].1, "my-key");
    }

    #[test]
    fn test_auth_type_custom() {
        let auth = AuthType::Custom {
            header: "X-Custom-Auth".to_string(),
            value: "custom-value".to_string(),
        };
        let req = HttpRequest::new(Method::Get, "https://api.example.com");
        let req = auth.apply(req);

        assert_eq!(req.headers.len(), 1);
        assert_eq!(req.headers[0].0, "X-Custom-Auth");
        assert_eq!(req.headers[0].1, "custom-value");
    }

    #[test]
    fn test_auth_type_jules_api_key() {
        let auth = AuthType::jules_api_key("my-jules-key");
        let req = HttpRequest::new(Method::Get, "https://jules.googleapis.com/v1alpha");
        let req = auth.apply(req);

        assert_eq!(req.headers.len(), 1);
        assert_eq!(req.headers[0].0, "X-Goog-Api-Key");
        assert_eq!(req.headers[0].1, "my-jules-key");
    }

    #[test]
    fn test_auth_type_none() {
        let auth = AuthType::None;
        let req = HttpRequest::new(Method::Get, "https://api.example.com");
        let req = auth.apply(req);

        assert!(req.headers.is_empty());
    }
}
