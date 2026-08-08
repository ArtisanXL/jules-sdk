//! Endpoint module defining construction of API endpoints.

use super::Method;
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC};

/// Characters left unescaped in a percent-encoded query key/value: alphanumerics plus the
/// RFC 3986 "unreserved" punctuation (`-_.~`), which never need encoding and are common in
/// tokens/ids (e.g. UUIDs). Everything else, including `&`/`=`/space, is encoded.
const QUERY_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');

/// Represents an API endpoint builder.
#[derive(Debug, Clone)]
pub struct Endpoint {
    base_url: String,
    path: String,
    method: Method,
    query_params: Vec<(String, String)>,
}

impl Endpoint {
    /// Creates a new `Endpoint` with the given base URL and path.
    #[must_use]
    pub fn new(base_url: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            path: path.into(),
            method: Method::Get,
            query_params: Vec::new(),
        }
    }

    /// Sets the HTTP method for the endpoint.
    #[must_use]
    pub fn with_method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    /// Adds a query parameter to the endpoint.
    #[must_use]
    pub fn with_query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// Builds the full URL string for the endpoint.
    ///
    /// Query parameter keys and values are percent-encoded so characters with special meaning
    /// in a query string (e.g. `&`, `=`, spaces) in a value like an opaque pagination token
    /// don't corrupt the query string's structure.
    #[must_use]
    pub fn build_url(&self) -> String {
        let capacity = self.base_url.len()
            + self.path.len()
            + usize::from(!self.query_params.is_empty())
            + self
                .query_params
                .iter()
                .map(|(k, v)| k.len() + v.len() + 2)
                .sum::<usize>();

        let mut url = String::with_capacity(capacity);
        url.push_str(&self.base_url);
        url.push_str(&self.path);

        if !self.query_params.is_empty() {
            url.push('?');
            for (i, (k, v)) in self.query_params.iter().enumerate() {
                if i > 0 {
                    url.push('&');
                }
                url.extend(percent_encoding::utf8_percent_encode(k, QUERY_ENCODE_SET));
                url.push('=');
                url.extend(percent_encoding::utf8_percent_encode(v, QUERY_ENCODE_SET));
            }
        }

        url
    }

    /// Returns the HTTP method for the endpoint.
    #[must_use]
    pub fn method(&self) -> Method {
        self.method
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_construction() {
        let endpoint = Endpoint::new("https://api.example.com", "/v1/users")
            .with_method(Method::Post)
            .with_query("limit", "10")
            .with_query("offset", "0");

        assert_eq!(endpoint.method(), Method::Post);
        assert_eq!(
            endpoint.build_url(),
            "https://api.example.com/v1/users?limit=10&offset=0"
        );
    }

    #[test]
    fn test_endpoint_no_query() {
        let endpoint = Endpoint::new("https://api.example.com", "/v1/users");
        assert_eq!(endpoint.build_url(), "https://api.example.com/v1/users");
        assert_eq!(endpoint.method(), Method::Get);
    }

    /// Proves that special characters in a query value (like an opaque pagination token) are
    /// percent-encoded rather than corrupting the query string's `&`/`=` delimiter structure.
    #[test]
    fn test_endpoint_build_url_special_characters_in_query_value() {
        let endpoint = Endpoint::new("https://api.example.com", "/v1alpha/sessions")
            .with_query("pageToken", "abc&x=1 def");

        let url = endpoint.build_url();

        assert_eq!(
            url,
            "https://api.example.com/v1alpha/sessions?pageToken=abc%26x%3D1%20def"
        );
        // No literal delimiter characters leaked into the query string from the value.
        assert_eq!(url.matches('&').count(), 0);
        assert_eq!(url.matches('=').count(), 1);
    }
}
