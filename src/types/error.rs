use std::fmt::Formatter;

#[derive(Debug)]
pub enum OpenApiError {
    ReqwestError(reqwest::Error),
    StdIoError(std::io::Error),
    StdParseError(std::string::ParseError), // Might be able to remove this, added by mistake (std != url)
    UrlParseError(url::ParseError),
    SerdeJsonError(serde_json::Error),
    NotArray,
}

impl std::fmt::Display for OpenApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "OpenApiError has occurred.")
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
