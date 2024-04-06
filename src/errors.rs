use std::error::Error;
use std::fmt;

/// Error type for Slack API operations.
#[derive(Debug)]
pub enum SlackApiError {
    InvalidArgument(String),
    HttpRequestFailed(String),
}

/// Implement the Error trait for SlackApiError.
impl Error for SlackApiError {}

/// Implement the Display trait for SlackApiError.
impl fmt::Display for SlackApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SlackApiError::InvalidArgument(ref msg) => write!(f, "Invalid argument: {}", msg),
            SlackApiError::HttpRequestFailed(ref msg) => write!(f, "HTTP request failed: {}", msg),
        }
    }
}

/// Implement the From trait for reqwest::Error to convert it into SlackApiError.
impl From<reqwest::Error> for SlackApiError {
    fn from(err: reqwest::Error) -> Self {
        SlackApiError::HttpRequestFailed(err.to_string())
    }
}
