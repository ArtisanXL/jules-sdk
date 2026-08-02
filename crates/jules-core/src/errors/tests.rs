use super::*;

#[test]
fn test_authentication_error() {
    let err = AuthenticationError::new("invalid key");
    assert_eq!(err.to_string(), "invalid key");

    let sdk_err: SDKError = err.into();
    assert_eq!(sdk_err.to_string(), "Authentication error: invalid key");
}

#[test]
fn test_api_error() {
    let err = ApiError::new("internal error");
    assert_eq!(err.to_string(), "internal error");

    let sdk_err: SDKError = err.into();
    assert_eq!(sdk_err.to_string(), "API error: internal error");
}

#[test]
fn test_api_error_with_status() {
    let err = ApiError::with_status("not found", 404);
    assert_eq!(err.to_string(), "[HTTP 404] not found");

    let sdk_err: SDKError = err.into();
    assert_eq!(sdk_err.to_string(), "API error: [HTTP 404] not found");
}

#[test]
fn test_network_error() {
    let err = NetworkError::new("timeout");
    assert_eq!(err.to_string(), "timeout");

    let sdk_err: SDKError = err.into();
    assert_eq!(sdk_err.to_string(), "Network error: timeout");
}

#[test]
fn test_streaming_error() {
    let err = StreamingError::new("unexpected EOF");
    assert_eq!(err.to_string(), "unexpected EOF");

    let sdk_err: SDKError = err.into();
    assert_eq!(sdk_err.to_string(), "Streaming error: unexpected EOF");
}

#[test]
fn test_tool_error() {
    let err = ToolError::new("execution failed");
    assert_eq!(err.to_string(), "execution failed");

    let sdk_err: SDKError = err.into();
    assert_eq!(sdk_err.to_string(), "Tool error: execution failed");
}

#[test]
fn test_validation_error() {
    let err = ValidationError::new("invalid parameter");
    assert_eq!(err.to_string(), "invalid parameter");

    let sdk_err: SDKError = err.into();
    assert_eq!(sdk_err.to_string(), "Validation error: invalid parameter");
}

#[test]
fn test_sdk_error_source() {
    use std::error::Error;

    let sdk_err: SDKError = AuthenticationError::new("auth").into();
    let source = sdk_err.source().unwrap();
    assert_eq!(source.to_string(), "auth");
    assert!(source.downcast_ref::<AuthenticationError>().is_some());

    let sdk_err: SDKError = ApiError::new("api").into();
    let source = sdk_err.source().unwrap();
    assert_eq!(source.to_string(), "api");
    assert!(source.downcast_ref::<ApiError>().is_some());

    let sdk_err: SDKError = NetworkError::new("network").into();
    let source = sdk_err.source().unwrap();
    assert_eq!(source.to_string(), "network");
    assert!(source.downcast_ref::<NetworkError>().is_some());

    let sdk_err: SDKError = StreamingError::new("stream").into();
    let source = sdk_err.source().unwrap();
    assert_eq!(source.to_string(), "stream");
    assert!(source.downcast_ref::<StreamingError>().is_some());

    let sdk_err: SDKError = ToolError::new("tool").into();
    let source = sdk_err.source().unwrap();
    assert_eq!(source.to_string(), "tool");
    assert!(source.downcast_ref::<ToolError>().is_some());

    let sdk_err: SDKError = ValidationError::new("val").into();
    let source = sdk_err.source().unwrap();
    assert_eq!(source.to_string(), "val");
    assert!(source.downcast_ref::<ValidationError>().is_some());
}
