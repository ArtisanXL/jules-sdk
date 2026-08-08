//! Source-resource endpoints of the Jules `v1alpha` REST API.

use crate::client::JulesClient;
use crate::http::{HttpRequest, Transport};
use jules_core::errors::SDKError;
use jules_core::source::SourceResource;
use serde::Deserialize;

/// Query parameters accepted by [`JulesClient::list_sources`].
#[derive(Debug, Clone, Default)]
pub struct ListSourcesParams {
    /// The maximum number of sources to return.
    pub page_size: Option<u32>,
    /// A page token from a previous `list_sources` response.
    pub page_token: Option<String>,
    /// A filter expression restricting which sources are returned.
    pub filter: Option<String>,
}

/// The response body for [`JulesClient::list_sources`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSourcesResponse {
    /// The returned sources.
    #[serde(default)]
    pub sources: Vec<SourceResource>,
    /// A token to retrieve the next page of results, if any.
    #[serde(default)]
    pub next_page_token: Option<String>,
}

impl<T: Transport> JulesClient<T> {
    /// Fetches a single GitHub-backed source by owner and repository name.
    ///
    /// # Errors
    /// Returns an [`SDKError`] if the transport fails or the response cannot be deserialized.
    pub async fn get_source(&self, owner: &str, repo: &str) -> Result<SourceResource, SDKError> {
        let endpoint = self.endpoint(format!("/sources/github/{owner}/{repo}"));
        let http_request = HttpRequest::new(endpoint.method(), endpoint.build_url());
        self.send_json(http_request).await
    }

    /// Lists sources.
    ///
    /// # Errors
    /// Returns an [`SDKError`] if the transport fails or the response cannot be deserialized.
    pub async fn list_sources(
        &self,
        params: &ListSourcesParams,
    ) -> Result<ListSourcesResponse, SDKError> {
        let mut endpoint = self.endpoint("/sources");
        if let Some(page_size) = params.page_size {
            endpoint = endpoint.with_query("pageSize", page_size.to_string());
        }
        if let Some(page_token) = &params.page_token {
            endpoint = endpoint.with_query("pageToken", page_token);
        }
        if let Some(filter) = &params.filter {
            endpoint = endpoint.with_query("filter", filter);
        }
        let http_request = HttpRequest::new(endpoint.method(), endpoint.build_url());
        self.send_json(http_request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::AuthType;
    use crate::client::JulesClient;
    use crate::http::HttpResponse;
    use std::sync::Mutex;

    struct MockTransport {
        response: Mutex<Option<HttpResponse>>,
    }

    impl Transport for MockTransport {
        async fn send(&self, _request: HttpRequest) -> Result<HttpResponse, SDKError> {
            Ok(self.response.lock().unwrap().take().unwrap())
        }
    }

    fn client(response: HttpResponse) -> JulesClient<MockTransport> {
        JulesClient::new(
            MockTransport {
                response: Mutex::new(Some(response)),
            },
            AuthType::jules_api_key("k"),
        )
    }

    #[tokio::test]
    async fn get_source_happy_path() {
        let body = r#"{
            "name": "sources/github/owner/repo",
            "id": "github/owner/repo",
            "githubRepo": {
                "owner": "owner",
                "repo": "repo",
                "isPrivate": false
            }
        }"#;
        let client = client(HttpResponse::new(200, vec![], body.into()));
        let source = client.get_source("owner", "repo").await.unwrap();
        assert_eq!(source.id, "github/owner/repo");
    }

    #[tokio::test]
    async fn list_sources_error_maps_to_api_error() {
        let body = br#"{"error": {"message": "Bad filter"}}"#.to_vec();
        let client = client(HttpResponse::new(400, vec![], body));
        let err = client
            .list_sources(&ListSourcesParams {
                filter: Some("invalid".to_string()),
                ..Default::default()
            })
            .await
            .unwrap_err();
        match err {
            SDKError::Api(e) => {
                assert_eq!(e.status_code, Some(400));
                assert_eq!(e.message, "Bad filter");
            }
            other => panic!("expected SDKError::Api, got {other:?}"),
        }
    }
}
