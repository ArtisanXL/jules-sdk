//! Core error types for the Jules SDK.

use std::fmt;

/// The primary error type for the Jules SDK.
#[derive(Debug)]
pub enum SDKError {
    /// Authentication errors (e.g., missing API key, invalid token).
    Authentication(AuthenticationError),
    /// API errors returned by the Jules API.
    Api(ApiError),
    /// Network errors (e.g., connection failed, timeout).
    Network(NetworkError),
    /// Streaming errors (e.g., unexpected stream termination).
    Streaming(StreamingError),
    /// Tool calling errors.
    Tool(ToolError),
    /// Validation errors (e.g., invalid configuration).
    Validation(ValidationError),
}

impl fmt::Display for SDKError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Authentication(e) => write!(f, "Authentication error: {e}"),
            Self::Api(e) => write!(f, "API error: {e}"),
            Self::Network(e) => write!(f, "Network error: {e}"),
            Self::Streaming(e) => write!(f, "Streaming error: {e}"),
            Self::Tool(e) => write!(f, "Tool error: {e}"),
            Self::Validation(e) => write!(f, "Validation error: {e}"),
        }
    }
}

impl std::error::Error for SDKError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Authentication(e) => Some(e),
            Self::Api(e) => Some(e),
            Self::Network(e) => Some(e),
            Self::Streaming(e) => Some(e),
            Self::Tool(e) => Some(e),
            Self::Validation(e) => Some(e),
        }
    }
}

macro_rules! impl_from_for_sdk_error {
    ($variant:ident, $err_type:ty) => {
        impl From<$err_type> for SDKError {
            fn from(err: $err_type) -> Self {
                Self::$variant(err)
            }
        }
    };
}

impl_from_for_sdk_error!(Authentication, AuthenticationError);
impl_from_for_sdk_error!(Api, ApiError);
impl_from_for_sdk_error!(Network, NetworkError);
impl_from_for_sdk_error!(Streaming, StreamingError);
impl_from_for_sdk_error!(Tool, ToolError);
impl_from_for_sdk_error!(Validation, ValidationError);

/// Authentication error.
#[derive(Debug)]
pub struct AuthenticationError {
    /// The error message.
    pub message: String,
}

impl AuthenticationError {
    /// Creates a new `AuthenticationError`.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AuthenticationError {}

/// API error.
#[derive(Debug)]
pub struct ApiError {
    /// The error message.
    pub message: String,
    /// The HTTP status code, if applicable.
    pub status_code: Option<u16>,
}

impl ApiError {
    /// Creates a new `ApiError`.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status_code: None,
        }
    }

    /// Creates a new `ApiError` with a status code.
    #[must_use]
    pub fn with_status(message: impl Into<String>, status_code: u16) -> Self {
        Self {
            message: message.into(),
            status_code: Some(status_code),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(code) = self.status_code {
            write!(f, "[HTTP {}] {}", code, self.message)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for ApiError {}

/// Network error.
#[derive(Debug)]
pub struct NetworkError {
    /// The error message.
    pub message: String,
}

impl NetworkError {
    /// Creates a new `NetworkError`.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for NetworkError {}

/// Streaming error.
#[derive(Debug)]
pub struct StreamingError {
    /// The error message.
    pub message: String,
}

impl StreamingError {
    /// Creates a new `StreamingError`.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for StreamingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StreamingError {}

/// Tool error.
#[derive(Debug)]
pub struct ToolError {
    /// The error message.
    pub message: String,
}

impl ToolError {
    /// Creates a new `ToolError`.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ToolError {}

/// Validation error.
#[derive(Debug)]
pub struct ValidationError {
    /// The error message.
    pub message: String,
}

impl ValidationError {
    /// Creates a new `ValidationError`.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests;
