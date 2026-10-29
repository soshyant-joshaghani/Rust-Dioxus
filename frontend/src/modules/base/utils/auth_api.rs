use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::modules::base::stores::auth::AuthUser;
use crate::modules::base::utils::{api_error::ApiError, http};

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Serialize)]
struct SignupBody<'a> {
    email: &'a str,
    password: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_name: Option<&'a str>,
}

pub async fn fetch_current_user(token: &str) -> Result<AuthUser, ApiError> {
    http::get_json(Some(token), "base/login/me", "Failed to fetch user profile").await
}

/// Form login (`username`, `password`) → JWT.
pub async fn login_with_password(email: &str, password: &str) -> Result<String, ApiError> {
    let form = [("username", email.trim()), ("password", password)];
    let token: TokenResponse = http::send_form("base/login/access-token", &form, "Invalid email or password").await?;
    Ok(token.access_token)
}

/// Dev signup through `POST /private/users` (only when the API runs with `ENVIRONMENT=local`).
pub async fn signup_with_private_route(email: &str, password: &str, full_name: &str) -> Result<(), ApiError> {
    let full_name = full_name.trim();
    let body = SignupBody {
        email: email.trim(),
        password,
        full_name: (!full_name.is_empty()).then_some(full_name),
    };
    let _: Value = http::send_json(None, http::Method::POST, "private/users", &body, "Sign up failed").await?;
    Ok(())
}
