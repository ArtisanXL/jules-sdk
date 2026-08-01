//! Auth module.

use crate::http::HttpRequest;

/// The type of authentication to use.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
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

impl AuthType {
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
    fn test_auth_type_none() {
        let auth = AuthType::None;
        let req = HttpRequest::new(Method::Get, "https://api.example.com");
        let req = auth.apply(req);

        assert!(req.headers.is_empty());
    }
}
