//! Response module for deserialization and error mapping.

use crate::http::HttpResponse;
use jules_core::errors::{ApiError, SDKError};
use jules_core::response::ClientResponse;
use serde::Deserialize;

/// Helper struct for deserializing API errors.
#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: Option<ErrorDetail>,
}

#[derive(Debug, Deserialize)]
struct ErrorDetail {
    message: String,
}

/// Deserializes an HTTP response into a `ClientResponse` or an `SDKError`.
///
/// # Errors
///
/// Returns `SDKError::Api` if the HTTP response status is not successful,
/// or if the JSON response body fails to deserialize properly.
pub fn deserialize_response(response: &HttpResponse) -> Result<ClientResponse, SDKError> {
    if response.status >= 200 && response.status < 300 {
        let client_response =
            serde_json::from_slice::<ClientResponse>(&response.body).map_err(|e| {
                SDKError::Api(ApiError::with_status(
                    format!("Failed to deserialize response: {e}"),
                    response.status,
                ))
            })?;
        Ok(client_response)
    } else {
        let message = match serde_json::from_slice::<ErrorResponse>(&response.body) {
            Ok(err_resp) => err_resp.error.map_or_else(
                || String::from_utf8_lossy(&response.body).into_owned(),
                |e| e.message,
            ),
            Err(_) => String::from_utf8_lossy(&response.body).into_owned(),
        };

        let mut msg = message.trim().to_string();
        if msg.is_empty() {
            msg = format!("HTTP {}", response.status);
        }

        Err(SDKError::Api(ApiError::with_status(msg, response.status)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jules_core::message::Role;

    #[test]
    fn test_deserialize_success() {
        let body = r#"{
            "message": {
                "role": "assistant",
                "content": "Hello, world!"
            }
        }"#
        .as_bytes()
        .to_vec();

        let http_resp = HttpResponse::new(200, vec![], body);
        let result = deserialize_response(&http_resp).unwrap();

        assert_eq!(*result.message.role(), Role::Assistant);
        assert_eq!(result.message.content(), "Hello, world!");
    }

    #[test]
    fn test_deserialize_api_error_json() {
        let body = r#"{
            "error": {
                "message": "Invalid API key"
            }
        }"#
        .as_bytes()
        .to_vec();

        let http_resp = HttpResponse::new(401, vec![], body);
        let err = deserialize_response(&http_resp).unwrap_err();

        match err {
            SDKError::Api(e) => {
                assert_eq!(e.status_code, Some(401));
                assert_eq!(e.message, "Invalid API key");
            }
            _ => panic!("Expected SDKError::Api"),
        }
    }

    #[test]
    fn test_deserialize_api_error_raw() {
        let body = "Internal Server Error".as_bytes().to_vec();

        let http_resp = HttpResponse::new(500, vec![], body);
        let err = deserialize_response(&http_resp).unwrap_err();

        match err {
            SDKError::Api(e) => {
                assert_eq!(e.status_code, Some(500));
                assert_eq!(e.message, "Internal Server Error");
            }
            _ => panic!("Expected SDKError::Api"),
        }
    }

    #[test]
    fn test_deserialize_api_error_empty_body() {
        let body = vec![];

        let http_resp = HttpResponse::new(500, vec![], body);
        let err = deserialize_response(&http_resp).unwrap_err();

        match err {
            SDKError::Api(e) => {
                assert_eq!(e.status_code, Some(500));
                assert_eq!(e.message, "HTTP 500");
            }
            _ => panic!("Expected SDKError::Api"),
        }
    }
}
