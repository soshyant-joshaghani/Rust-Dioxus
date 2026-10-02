use serde::{Deserialize, Serialize};

use crate::modules::base::utils::{api_error::ApiError, http};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ManagedUser {
    pub id: String,
    pub email: String,
    #[serde(default)]
    pub full_name: Option<String>,
    #[serde(default = "default_true")]
    pub is_active: bool,
    #[serde(default)]
    pub is_superuser: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Deserialize)]
struct UsersPage {
    data: Vec<ManagedUser>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserCreate {
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
    pub is_active: bool,
    pub is_superuser: bool,
}

/// Only the fields that are `Some` are sent, so the API changes only those.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UserUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// `Some(None)` clears the name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_superuser: Option<bool>,
}

pub async fn list_users(token: &str) -> Result<Vec<ManagedUser>, ApiError> {
    let page: UsersPage = http::get_json(Some(token), "base/users/admin", "Failed to load users").await?;
    Ok(page.data)
}

pub async fn create_user(token: &str, data: &UserCreate) -> Result<ManagedUser, ApiError> {
    http::send_json(Some(token), http::Method::POST, "base/users/admin", data, "Failed to create user").await
}

pub async fn update_user(token: &str, user_id: &str, data: &UserUpdate) -> Result<ManagedUser, ApiError> {
    let path = format!("base/users/{user_id}/admin");
    http::send_json(Some(token), http::Method::PATCH, &path, data, "Failed to update user").await
}

pub async fn delete_user(token: &str, user_id: &str) -> Result<(), ApiError> {
    let path = format!("base/users/{user_id}/admin");
    http::delete(Some(token), &path, "Failed to delete user").await
}
