//! Session-resource endpoints of the Jules `v1alpha` REST API.

use crate::client::JulesClient;
use crate::http::{HttpRequest, Method, Transport};
use jules_core::errors::{SDKError, ValidationError};
use jules_core::session::resource::{AutomationMode, SessionResource, SourceContext};
use serde::{Deserialize, Serialize};

/// The request body for [`JulesClient::create_session`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSessionRequest {
    /// The prompt to start the session with.
    pub prompt: String,
    /// The source and branch the session should operate on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_context: Option<SourceContext>,
    /// An optional title for the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Whether a generated plan requires explicit approval before execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_plan_approval: Option<bool>,
    /// The automation mode for the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_mode: Option<AutomationMode>,
}

/// Query parameters accepted by [`JulesClient::list_sessions`].
#[derive(Debug, Clone, Default)]
pub struct ListSessionsParams {
    /// The maximum number of sessions to return.
    pub page_size: Option<u32>,
    /// A page token from a previous `list_sessions` response.
    pub page_token: Option<String>,
    /// A filter expression restricting which sessions are returned.
    pub filter: Option<String>,
}

/// The response body for [`JulesClient::list_sessions`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSessionsResponse {
    /// The returned sessions.
    #[serde(default)]
    pub sessions: Vec<SessionResource>,
    /// A token to retrieve the next page of results, if any.
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Serialize)]
struct SendMessageRequest<'a> {
    prompt: &'a str,
}

fn to_json_body<T: Serialize>(value: &T) -> Result<Vec<u8>, SDKError> {
    serde_json::to_vec(value).map_err(|e| {
        SDKError::Validation(ValidationError::new(format!(
            "Failed to serialize request body: {e}"
        )))
    })
}

impl<T: Transport> JulesClient<T> {
    /// Creates a new session.
    ///
    /// # Errors
    /// Returns an [`SDKError`] if the request body cannot be serialized, the
    /// transport fails, or the response cannot be deserialized.
    pub async fn create_session(
        &self,
        request: &CreateSessionRequest,
    ) -> Result<SessionResource, SDKError> {
        let body = to_json_body(request)?;
        let endpoint = self.endpoint("/sessions").with_method(Method::Post);
        let http_request = HttpRequest::new(endpoint.method(), endpoint.build_url())
            .with_header("Content-Type", "application/json")
            .with_body(body);
        self.send_json(http_request).await
    }

    /// Fetches a single session by id (e.g. `"12345"`).
    ///
    /// # Errors
    /// Returns an [`SDKError`] if the transport fails or the response cannot be deserialized.
    pub async fn get_session(&self, id: &str) -> Result<SessionResource, SDKError> {
        let endpoint = self.endpoint(format!("/sessions/{id}"));
        let http_request = HttpRequest::new(endpoint.method(), endpoint.build_url());
        self.send_json(http_request).await
    }

    /// Lists sessions.
    ///
    /// # Errors
    /// Returns an [`SDKError`] if the transport fails or the response cannot be deserialized.
    pub async fn list_sessions(
        &self,
        params: &ListSessionsParams,
    ) -> Result<ListSessionsResponse, SDKError> {
        let mut endpoint = self.endpoint("/sessions");
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

    /// Approves the pending plan for the given session.
    ///
    /// # Errors
    /// Returns an [`SDKError`] if the transport fails or the response status is not successful.
    pub async fn approve_plan(&self, id: &str) -> Result<(), SDKError> {
        let endpoint = self
            .endpoint(format!("/sessions/{id}:approvePlan"))
            .with_method(Method::Post);
        let http_request = HttpRequest::new(endpoint.method(), endpoint.build_url());
        self.send_status(http_request).await
    }

    /// Sends a message to the given session.
    ///
    /// # Errors
    /// Returns an [`SDKError`] if the request body cannot be serialized, the
    /// transport fails, or the response status is not successful.
    pub async fn send_message(&self, id: &str, prompt: &str) -> Result<(), SDKError> {
        let body = to_json_body(&SendMessageRequest { prompt })?;
        let endpoint = self
            .endpoint(format!("/sessions/{id}:sendMessage"))
            .with_method(Method::Post);
        let http_request = HttpRequest::new(endpoint.method(), endpoint.build_url())
            .with_header("Content-Type", "application/json")
            .with_body(body);
        self.send_status(http_request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::AuthType;
    use crate::http::HttpResponse;
    use jules_core::session::resource::SessionState;
    use std::sync::Mutex;

    struct MockTransport {
        response: Mutex<Option<HttpResponse>>,
        last_request: Mutex<Option<HttpRequest>>,
    }

    impl MockTransport {
        fn new(response: HttpResponse) -> Self {
            Self {
                response: Mutex::new(Some(response)),
                last_request: Mutex::new(None),
            }
        }
    }

    impl Transport for MockTransport {
        async fn send(&self, request: HttpRequest) -> Result<HttpResponse, SDKError> {
            *self.last_request.lock().unwrap() = Some(request);
            Ok(self.response.lock().unwrap().take().unwrap())
        }
    }

    fn client(response: HttpResponse) -> JulesClient<MockTransport> {
        JulesClient::new(MockTransport::new(response), AuthType::jules_api_key("k"))
    }

    fn session_json() -> &'static str {
        r#"{
            "name": "sessions/12345",
            "id": "12345",
            "prompt": "Fix the bug",
            "sourceContext": { "source": "sources/github/owner/repo" },
            "title": "Fix the bug",
            "createTime": "2026-08-08T00:00:00Z",
            "updateTime": "2026-08-08T00:00:00Z",
            "state": "QUEUED",
            "url": "https://jules.google.com/session/12345"
        }"#
    }

    #[tokio::test]
    async fn create_session_happy_path() {
        let client = client(HttpResponse::new(200, vec![], session_json().into()));
        let request = CreateSessionRequest {
            prompt: "Fix the bug".to_string(),
            source_context: None,
            title: None,
            require_plan_approval: None,
            automation_mode: None,
        };
        let session = client.create_session(&request).await.unwrap();
        assert_eq!(session.id, "12345");
        assert_eq!(session.state, SessionState::Queued);
    }

    #[tokio::test]
    async fn get_session_happy_path() {
        let client = client(HttpResponse::new(200, vec![], session_json().into()));
        let session = client.get_session("12345").await.unwrap();
        assert_eq!(session.name, "sessions/12345");
    }

    #[tokio::test]
    async fn list_sessions_with_pagination() {
        let body = format!(
            r#"{{ "sessions": [{}], "nextPageToken": "page-2" }}"#,
            session_json()
        );
        let client = client(HttpResponse::new(200, vec![], body.into_bytes()));
        let result = client
            .list_sessions(&ListSessionsParams {
                page_size: Some(10),
                page_token: Some("page-1".to_string()),
                filter: None,
            })
            .await
            .unwrap();
        assert_eq!(result.sessions.len(), 1);
        assert_eq!(result.next_page_token.as_deref(), Some("page-2"));
    }

    #[tokio::test]
    async fn approve_plan_happy_path() {
        let client = client(HttpResponse::new(200, vec![], b"{}".to_vec()));
        client.approve_plan("12345").await.unwrap();
    }

    #[tokio::test]
    async fn send_message_happy_path() {
        let client = client(HttpResponse::new(200, vec![], b"{}".to_vec()));
        client.send_message("12345", "keep going").await.unwrap();
    }

    #[tokio::test]
    async fn get_session_not_found_maps_to_api_error() {
        let body = br#"{"error": {"message": "Session not found"}}"#.to_vec();
        let client = client(HttpResponse::new(404, vec![], body));
        let err = client.get_session("missing").await.unwrap_err();
        match err {
            SDKError::Api(e) => {
                assert_eq!(e.status_code, Some(404));
                assert_eq!(e.message, "Session not found");
            }
            other => panic!("expected SDKError::Api, got {other:?}"),
        }
    }
}
