//! Endpoint module defining construction of API endpoints.

use super::Method;

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
    #[must_use]
    pub fn build_url(&self) -> String {
        let mut url = format!("{}{}", self.base_url, self.path);

        if !self.query_params.is_empty() {
            url.push('?');
            let query_string = self
                .query_params
                .iter()
                .map(|(k, v)| format!("{k}={v}")) // Note: Needs URL encoding in production
                .collect::<Vec<String>>()
                .join("&");
            url.push_str(&query_string);
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
}
