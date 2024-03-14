use std::fmt::{write, Formatter};

#[derive(Debug)]
struct OperationError {
    created: u64,
    origin: String,
    reason: String,
    thread_id: String,
}

impl OperationError {
    pub fn new(origin: String, reason: String) -> Self {
        Self {
            origin: origin.to_string(),
            reason: reason.to_string(),
            created: std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            thread_id: format!("{:?}", std::thread::current().id()),
        }
    }
}

#[derive(Debug)]
pub enum OpenApiError {
    ReqwestError(reqwest::Error),
    StdIoError(std::io::Error),
    StdParseError(std::string::ParseError), // Might be able to remove this, added by mistake (std != url)
    UrlParseError(url::ParseError),
    SerdeJsonError(serde_json::Error),
    // Library Errors
    NotArray,
    InvalidLength(usize, usize),
    RestrictedValue(String),
    ClientError(String),
    OperationError(OperationError), //
}

impl OpenApiError {
    pub fn new_operation_err<O: Into<String>, R: Into<String>>(origin: O, reason: R) -> Self {
        OpenApiError::OperationError(OperationError::new(origin.into(), reason.into()))
    }
}

impl std::fmt::Display for OpenApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenApiError::InvalidLength(len, max) => {
                write!(f, "Length of {} exceeds maximum of {}", len, max)
            }
            OpenApiError::ClientError(val) => {
                write!(f, "ClientError: {}", val)
            }

            _ => write!(f, "OpenApiError has occurred."),
        }
    }
}

impl std::error::Error for OpenApiError {}

impl From<reqwest::Error> for OpenApiError {
    fn from(value: reqwest::Error) -> Self {
        OpenApiError::ReqwestError(value)
    }
}

impl From<std::io::Error> for OpenApiError {
    fn from(value: std::io::Error) -> Self {
        OpenApiError::StdIoError(value)
    }
}

impl From<std::string::ParseError> for OpenApiError {
    fn from(value: std::string::ParseError) -> Self {
        OpenApiError::StdParseError(value)
    }
}

impl From<url::ParseError> for OpenApiError {
    fn from(value: url::ParseError) -> Self {
        OpenApiError::UrlParseError(value)
    }
}

impl From<serde_json::Error> for OpenApiError {
    fn from(value: serde_json::Error) -> Self {
        OpenApiError::SerdeJsonError(value)
    }
}
