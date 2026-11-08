use std::fmt;

use serde_json::Value;

/// An HTTP failure with the contract's `detail` message. `status` is 0 when the request never reached the API.
#[derive(Debug, Clone, PartialEq)]
pub struct ApiError {
    pub message: String,
    pub status: u16,
}

impl ApiError {
    pub fn new(message: impl Into<String>, status: u16) -> Self {
        Self { message: message.into(), status }
    }

    pub fn is_unauthorized(&self) -> bool {
        self.status == 401
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ApiError {}

/// `{"detail": "..."}` or FastAPI's `{"detail": [{"msg": ...}]}`; anything else is `fallback`.
pub fn format_api_error(body: &Value, fallback: &str) -> String {
    match body.get("detail") {
        Some(Value::String(detail)) => detail.clone(),
        Some(Value::Array(items)) if !items.is_empty() => items
            .iter()
            .map(|item| match item.get("msg") {
                Some(Value::String(msg)) => msg.clone(),
                Some(other) => other.to_string(),
                None => match item {
                    Value::String(s) => s.clone(),
                    other => other.to_string(),
                },
            })
            .collect::<Vec<_>>()
            .join(", "),
        _ => fallback.to_string(),
    }
}
