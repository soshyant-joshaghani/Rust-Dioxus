use serde::{Deserialize, Serialize};

use crate::modules::base::utils::{api_error::ApiError, http};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub owner_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct NoteCreate {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct NoteUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

pub async fn list_notes(token: &str) -> Result<Vec<Note>, ApiError> {
    http::get_json(Some(token), "sample/notes", "Failed to load notes").await
}

pub async fn create_note(token: &str, data: &NoteCreate) -> Result<Note, ApiError> {
    http::send_json(Some(token), http::Method::POST, "sample/notes", data, "Failed to create note").await
}

pub async fn update_note(token: &str, id: &str, data: &NoteUpdate) -> Result<Note, ApiError> {
    let path = format!("sample/notes/{id}");
    http::send_json(Some(token), http::Method::PATCH, &path, data, "Failed to update note").await
}

pub async fn delete_note(token: &str, id: &str) -> Result<(), ApiError> {
    let path = format!("sample/notes/{id}");
    http::delete(Some(token), &path, "Failed to delete note").await
}
