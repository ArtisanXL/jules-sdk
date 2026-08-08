//! Test-only [`Transport`] mock shared by command unit tests.
//!
//! This is a local implementation of the public [`Transport`] trait — it
//! does not depend on any test-only support from `jules-api` — and never
//! performs real network I/O.

#![cfg(test)]

use std::sync::{Arc, Mutex};

use jules_api::http::{HttpRequest, HttpResponse, Transport};
use jules_sdk::jules_core::errors::SDKError;

struct MockState {
    response: Mutex<Option<HttpResponse>>,
    last_request: Mutex<Option<HttpRequest>>,
}

/// A [`Transport`] mock that returns a pre-programmed response and records
/// the last request sent through it.
///
/// Cheaply [`Clone`]-able (backed by an [`Arc`]) so a handle can be kept
/// outside a [`jules_api::client::JulesClient`] that has taken ownership of
/// one clone, in order to inspect the last request it sent.
#[derive(Clone)]
pub struct MockTransport {
    state: Arc<MockState>,
}

impl MockTransport {
    /// Creates a mock that returns `response` for the next `send` call.
    pub fn new(response: HttpResponse) -> Self {
        Self {
            state: Arc::new(MockState {
                response: Mutex::new(Some(response)),
                last_request: Mutex::new(None),
            }),
        }
    }

    /// Returns the last request sent through this transport, if any.
    pub fn last_request(&self) -> Option<HttpRequest> {
        self.state.last_request.lock().unwrap().clone()
    }
}

impl Transport for MockTransport {
    async fn send(&self, request: HttpRequest) -> Result<HttpResponse, SDKError> {
        *self.state.last_request.lock().unwrap() = Some(request);
        Ok(self.state.response.lock().unwrap().take().unwrap())
    }
}
