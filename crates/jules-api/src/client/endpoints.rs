//! Real `v1alpha` Jules API endpoints (sessions, sessions.activities, sources) on
//! [`JulesClient`].
//!
//! Read endpoints (`list_sessions`, `get_session`, `list_sources`, `list_activities`) were
//! built directly from real response payloads captured against the live Jules API on
//! 2026-08-08. Write endpoints (`create_session`, `send_message`, `approve_plan`) were NOT
//! exercised against the live API (doing so would create/mutate a real session) — their
//! request shapes are a best-effort reading of `ROADMAP.md` and Google API "custom method"
//! (`:methodName`) conventions, and are only verified against local mock servers. Treat them
//! as unverified until confirmed against the real API.

use super::JulesClient;
use crate::http::endpoint::Endpoint;
use crate::http::{HttpRequest, HttpResponse, Method, Transport};
use crate::response::{deserialize_json, map_error_response};
use crate::retry::{ExponentialBackoff, RetryPolicy};
use jules_core::activity::Activity;
use jules_core::errors::{SDKError, ValidationError};
use jules_core::pagination::Page;
use jules_core::session::{Session, SourceContext};
use jules_core::source::Source;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SessionsListResponse {
    #[serde(default)]
    sessions: Vec<Session>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SourcesListResponse {
    #[serde(default)]
    sources: Vec<Source>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActivitiesListResponse {
    #[serde(default)]
    activities: Vec<Activity>,
    #[serde(default)]
    next_page_token: Option<String>,
}

/// Parameters for [`JulesClient::create_session`].
///
/// **Unverified against the live API**: the exact required/accepted fields for session
/// creation were not confirmed against the real Jules API (creating a real session was
/// intentionally avoided). This is a best-effort shape based on the fields returned by
/// `GET`/`LIST` and `ROADMAP.md`'s description of the endpoint.
#[derive(Clone, Default)]
pub struct CreateSessionParams {
    /// A human-readable title for the session.
    pub title: Option<String>,
    /// The prompt to start the session with.
    pub prompt: Option<String>,
    /// The source (e.g. a GitHub repo) the session should operate on.
    pub source_context: Option<SourceContext>,
}

impl std::fmt::Debug for CreateSessionParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateSessionParams")
            .field("title", &self.title)
            .field("prompt", &self.prompt.as_ref().map(|_| "***REDACTED***"))
            .field("source_context", &self.source_context)
            .finish()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateSessionRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_context: Option<&'a SourceContext>,
}

#[derive(Serialize)]
struct ApprovePlanRequest {}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SendMessageRequest<'a> {
    prompt: &'a str,
}

fn page_size_str(page_size: Option<i32>) -> Option<String> {
    page_size.map(|n| n.to_string())
}

impl JulesClient {
    /// Sends a single HTTP request built by `build`, retrying on retriable errors using
    /// [`ExponentialBackoff`], and returns the raw successful [`HttpResponse`].
    ///
    /// `build` is called once per attempt so a fresh, unconsumed [`HttpRequest`] can be built
    /// each time (auth is (re)applied on every attempt).
    async fn send_with_retry<F>(&self, build: F) -> Result<HttpResponse, SDKError>
    where
        F: Fn() -> HttpRequest,
    {
        let policy = ExponentialBackoff::default();
        let mut attempt = 0u32;
        loop {
            let request = self.auth.clone().apply(build());
            let err: SDKError = match self.transport.send(request).await {
                Ok(response) if (200..300).contains(&response.status) => return Ok(response),
                Ok(response) => map_error_response(&response),
                Err(err) => err,
            };

            match policy.should_retry(attempt, &err) {
                Some(delay_ms) => {
                    attempt += 1;
                    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                }
                None => return Err(err),
            }
        }
    }

    async fn get_json<R: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<R, SDKError> {
        let build = || {
            let mut endpoint = Endpoint::new(self.base_url(), path).with_method(Method::Get);
            for (k, v) in query {
                endpoint = endpoint.with_query(*k, v.clone());
            }
            HttpRequest::new(Method::Get, endpoint.build_url())
        };
        let response = self.send_with_retry(build).await?;
        deserialize_json(&response)
    }

    async fn post_json<B: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<R, SDKError> {
        let bytes = serde_json::to_vec(body).map_err(|e| {
            SDKError::Validation(ValidationError::new(format!(
                "failed to serialize request: {e}"
            )))
        })?;
        let url = format!("{}{path}", self.base_url());
        let build = || {
            HttpRequest::new(Method::Post, url.clone())
                .with_header("Content-Type", "application/json")
                .with_body(bytes.clone())
        };
        let response = self.send_with_retry(build).await?;
        deserialize_json(&response)
    }

    async fn post_no_content<B: Serialize>(&self, path: &str, body: &B) -> Result<(), SDKError> {
        let bytes = serde_json::to_vec(body).map_err(|e| {
            SDKError::Validation(ValidationError::new(format!(
                "failed to serialize request: {e}"
            )))
        })?;
        let url = format!("{}{path}", self.base_url());
        let build = || {
            HttpRequest::new(Method::Post, url.clone())
                .with_header("Content-Type", "application/json")
                .with_body(bytes.clone())
        };
        self.send_with_retry(build).await.map(|_| ())
    }

    /// Lists sessions. `GET /v1alpha/sessions`.
    ///
    /// # Errors
    /// Returns `SDKError` on network, auth, or deserialization failure.
    pub async fn list_sessions(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
    ) -> Result<Page<Session>, SDKError> {
        let mut query = Vec::new();
        if let Some(size) = page_size_str(page_size) {
            query.push(("pageSize", size));
        }
        if let Some(token) = page_token {
            query.push(("pageToken", token.to_string()));
        }
        let response: SessionsListResponse = self.get_json("/v1alpha/sessions", &query).await?;
        Ok(Page::new(response.sessions, response.next_page_token))
    }

    /// Gets a single session by resource name (e.g. `sessions/1234567890`).
    /// `GET /v1alpha/{name}`.
    ///
    /// # Errors
    /// Returns `SDKError` on network, auth, or deserialization failure.
    pub async fn get_session(&self, name: &str) -> Result<Session, SDKError> {
        self.get_json(&format!("/v1alpha/{name}"), &[]).await
    }

    /// Creates a new session. `POST /v1alpha/sessions`.
    ///
    /// **Unverified against the live API** — see [`CreateSessionParams`].
    ///
    /// # Errors
    /// Returns `SDKError` on network, auth, or deserialization failure.
    pub async fn create_session(&self, params: CreateSessionParams) -> Result<Session, SDKError> {
        let request = CreateSessionRequest {
            title: params.title.as_deref(),
            prompt: params.prompt.as_deref(),
            source_context: params.source_context.as_ref(),
        };
        self.post_json("/v1alpha/sessions", &request).await
    }

    /// Sends a message to an existing session. `POST /v1alpha/{session_name}:sendMessage`.
    ///
    /// **Live API note**: The session must be in a ready state (e.g., `AWAITING_USER_FEEDBACK`)
    /// before calling this endpoint. If called while the session is still in the `QUEUED` state
    /// (such as immediately after creation), the API may return a `404 Requested entity was not found` error.
    ///
    /// # Errors
    /// Returns `SDKError` on network, auth, or deserialization failure.
    pub async fn send_message(&self, session_name: &str, message: &str) -> Result<(), SDKError> {
        let request = SendMessageRequest { prompt: message };
        self.post_no_content(&format!("/v1alpha/{session_name}:sendMessage"), &request)
            .await
    }

    /// Approves the currently proposed plan for a session.
    /// `POST /v1alpha/{session_name}:approvePlan`.
    ///
    /// **Live API note**: the request body must be `{}`.
    ///
    /// # Errors
    /// Returns `SDKError` on network, auth, or deserialization failure.
    pub async fn approve_plan(&self, session_name: &str) -> Result<(), SDKError> {
        self.post_no_content(
            &format!("/v1alpha/{session_name}:approvePlan"),
            &ApprovePlanRequest {},
        )
        .await
    }

    /// Lists sources (e.g. connected GitHub repositories). `GET /v1alpha/sources`.
    ///
    /// # Errors
    /// Returns `SDKError` on network, auth, or deserialization failure.
    pub async fn list_sources(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
    ) -> Result<Page<Source>, SDKError> {
        let mut query = Vec::new();
        if let Some(size) = page_size_str(page_size) {
            query.push(("pageSize", size));
        }
        if let Some(token) = page_token {
            query.push(("pageToken", token.to_string()));
        }
        let response: SourcesListResponse = self.get_json("/v1alpha/sources", &query).await?;
        Ok(Page::new(response.sources, response.next_page_token))
    }

    /// Lists activities for a session. `GET /v1alpha/{session_name}/activities`.
    ///
    /// # Errors
    /// Returns `SDKError` on network, auth, or deserialization failure.
    pub async fn list_activities(
        &self,
        session_name: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
    ) -> Result<Page<Activity>, SDKError> {
        let mut query = Vec::new();
        if let Some(size) = page_size_str(page_size) {
            query.push(("pageSize", size));
        }
        if let Some(token) = page_token {
            query.push(("pageToken", token.to_string()));
        }
        let response: ActivitiesListResponse = self
            .get_json(&format!("/v1alpha/{session_name}/activities"), &query)
            .await?;
        Ok(Page::new(response.activities, response.next_page_token))
    }
}
