//! Activity-resource endpoints of the Jules `v1alpha` REST API.

use crate::client::JulesClient;
use crate::http::{HttpRequest, Transport};
use jules_core::activity::ActivityEvent;
use jules_core::errors::SDKError;
use serde::Deserialize;

/// Query parameters accepted by [`JulesClient::list_activities`].
#[derive(Debug, Clone, Default)]
pub struct ListActivitiesParams {
    /// The maximum number of activities to return.
    pub page_size: Option<u32>,
    /// A page token from a previous `list_activities` response.
    pub page_token: Option<String>,
    /// A filter expression restricting which activities are returned.
    pub filter: Option<String>,
}

/// The response body for [`JulesClient::list_activities`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListActivitiesResponse {
    /// The returned activities.
    #[serde(default)]
    pub activities: Vec<ActivityEvent>,
    /// A token to retrieve the next page of results, if any.
    #[serde(default)]
    pub next_page_token: Option<String>,
}

impl<T: Transport> JulesClient<T> {
    /// Fetches a single activity from a session.
    ///
    /// # Errors
    /// Returns an [`SDKError`] if the transport fails or the response cannot be deserialized.
    pub async fn get_activity(
        &self,
        session_id: &str,
        activity_id: &str,
    ) -> Result<ActivityEvent, SDKError> {
        let endpoint = self.endpoint(format!("/sessions/{session_id}/activities/{activity_id}"));
        let http_request = HttpRequest::new(endpoint.method(), endpoint.build_url());
        self.send_json(http_request).await
    }

    /// Lists activities belonging to a session.
    ///
    /// # Errors
    /// Returns an [`SDKError`] if the transport fails or the response cannot be deserialized.
    pub async fn list_activities(
        &self,
        session_id: &str,
        params: &ListActivitiesParams,
    ) -> Result<ListActivitiesResponse, SDKError> {
        let mut endpoint = self.endpoint(format!("/sessions/{session_id}/activities"));
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
    async fn list_activities_happy_path() {
        let body = r#"{
            "activities": [
                {
                    "name": "sessions/12345/activities/1",
                    "id": "1",
                    "createTime": "2026-08-08T00:00:00Z",
                    "originator": "agent",
                    "agentMessaged": { "message": "hello" }
                }
            ],
            "nextPageToken": "page-2"
        }"#;
        let client = client(HttpResponse::new(200, vec![], body.into()));
        let result = client
            .list_activities("12345", &ListActivitiesParams::default())
            .await
            .unwrap();
        assert_eq!(result.activities.len(), 1);
        assert_eq!(result.next_page_token.as_deref(), Some("page-2"));
    }

    #[tokio::test]
    async fn get_activity_happy_path() {
        let body = r#"{
            "name": "sessions/12345/activities/1",
            "id": "1",
            "createTime": "2026-08-08T00:00:00Z",
            "originator": "user",
            "userMessaged": { "message": "hi" }
        }"#;
        let client = client(HttpResponse::new(200, vec![], body.into()));
        let activity = client.get_activity("12345", "1").await.unwrap();
        assert_eq!(activity.id, "1");
    }
}
