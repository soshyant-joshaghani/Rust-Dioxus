//! Session store. Same keys as Rust-Svelte (`authToken`, `currentUser`).

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use super::storage;
use crate::modules::base::utils::auth_api::fetch_current_user;

const STORAGE_TOKEN: &str = "authToken";
const STORAGE_USER: &str = "currentUser";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthUser {
    #[serde(default)]
    pub id: Option<String>,
    pub email: String,
    #[serde(default)]
    pub full_name: Option<String>,
    #[serde(default)]
    pub is_active: Option<bool>,
    #[serde(default)]
    pub is_superuser: Option<bool>,
}

impl AuthUser {
    pub fn is_superuser(&self) -> bool {
        self.is_superuser.unwrap_or(false)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthState {
    pub user: Option<AuthUser>,
    pub token: Option<String>,
    pub is_loading: bool,
}

impl AuthState {
    pub fn is_authenticated(&self) -> bool {
        self.user.is_some() && self.token.is_some()
    }

    pub fn is_superuser(&self) -> bool {
        self.user.as_ref().is_some_and(AuthUser::is_superuser)
    }
}

/// Copyable handle shared through context. Provided once in `App`.
#[derive(Clone, Copy)]
pub struct Auth(pub Signal<AuthState>);

impl Auth {
    pub fn state(&self) -> AuthState {
        self.0.read().clone()
    }

    pub fn token(&self) -> Option<String> {
        self.0.read().token.clone()
    }

    pub fn login(mut self, token: String, user: AuthUser) {
        storage::set(STORAGE_TOKEN, &token);
        if let Ok(raw) = serde_json::to_string(&user) {
            storage::set(STORAGE_USER, &raw);
        }
        self.0.set(AuthState { user: Some(user), token: Some(token), is_loading: false });
    }

    pub fn logout(mut self) {
        storage::remove(STORAGE_TOKEN);
        storage::remove(STORAGE_USER);
        self.0.set(AuthState { user: None, token: None, is_loading: false });
    }
}

pub fn use_auth() -> Auth {
    use_context::<Auth>()
}

/// Create the store and restore the stored session (re-validated with `GET /base/login/me`).
pub fn use_auth_provider() -> Auth {
    let auth = use_context_provider(|| {
        Auth(Signal::new(AuthState { user: None, token: None, is_loading: true }))
    });
    use_hook(move || {
        spawn(async move {
            let Some(token) = storage::get(STORAGE_TOKEN) else {
                auth.logout();
                return;
            };
            match fetch_current_user(&token).await {
                Ok(user) => auth.login(token, user),
                Err(_) => auth.logout(),
            }
        });
    });
    auth
}
