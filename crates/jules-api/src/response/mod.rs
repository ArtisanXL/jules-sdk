//! Response module for deserialization and error mapping.

use crate::http::HttpResponse;
use jules_core::errors::{ApiError, SDKError};
use jules_core::response::ClientResponse;
use serde::de::DeserializeOwned;
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

/// Maps a non-2xx `HttpResponse` into an `SDKError::Api`, extracting a message from a
/// `{"error": {"message": ...}}` body when present, falling back to the raw body text.
pub(crate) fn map_error_response(response: &HttpResponse) -> SDKError {
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

    SDKError::Api(ApiError::with_status(msg, response.status))
}

/// Deserializes a successful (2xx) `HttpResponse` body as JSON into `R`, or maps a non-2xx
/// response into an `SDKError::Api`.
///
/// # Errors
///
/// Returns `SDKError::Api` if the HTTP response status is not successful, or if the JSON
/// response body fails to deserialize properly.
pub(crate) fn deserialize_json<R: DeserializeOwned>(
    response: &HttpResponse,
) -> Result<R, SDKError> {
    if response.status >= 200 && response.status < 300 {
        serde_json::from_slice::<R>(&response.body).map_err(|e| {
            SDKError::Api(ApiError::with_status(
                format!("Failed to deserialize response: {e}"),
                response.status,
            ))
        })
    } else {
        Err(map_error_response(response))
    }
}

/// Deserializes an HTTP response into a `ClientResponse` or an `SDKError`.
///
/// # Errors
///
/// Returns `SDKError::Api` if the HTTP response status is not successful,
/// or if the JSON response body fails to deserialize properly.
pub fn deserialize_response(response: &HttpResponse) -> Result<ClientResponse, SDKError> {
    deserialize_json::<ClientResponse>(response)
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

    /// Proves that a 2xx response whose body fails to deserialize into the target type (e.g.
    /// the API returned a changed/unexpected schema) is mapped into an `SDKError::Api` that
    /// preserves the success status code and explains the deserialization failure, rather than
    /// being confused with a genuine non-2xx API error.
    #[test]
    fn test_deserialize_malformed_json_with_success_status() {
        let body = b"not valid json".to_vec();

        let http_resp = HttpResponse::new(200, vec![], body);
        let err = deserialize_response(&http_resp).unwrap_err();

        match err {
            SDKError::Api(e) => {
                assert_eq!(e.status_code, Some(200));
                assert!(
                    e.message.contains("Failed to deserialize response"),
                    "unexpected message: {}",
                    e.message
                );
            }
            _ => panic!("Expected SDKError::Api"),
        }
    }
}
