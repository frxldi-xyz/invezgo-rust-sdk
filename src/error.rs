//! Custom SDK errors

#[derive(Debug, thiserror::Error)]
pub enum InvezgoError {
    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),

    #[error("JSON deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API error (status {status_code}): {message}")]
    ApiError {
        status_code: reqwest::StatusCode,
        message: String,
        error: Option<String>,
    },

    #[error("Other error: {0}")]
    Other(String),
}

#[derive(serde::Deserialize, Debug)]
pub struct ApiErrorResponse {
    pub message: serde_json::Value,
    pub error: Option<String>,
}
