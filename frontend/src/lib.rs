//! Rust-Dioxus client. One codebase for web, desktop (Windows, macOS, Linux) and mobile (Android, iOS).
//!
//! Layout mirrors Rust-Svelte's `frontend/src`:
//! - `config`   API base URL
//! - `modules`  only `base/` (auth, users, shell, ui primitives) and `apps/<name>/` (product domains)
//! - `routes`   pages, one file per path in the `Route` enum

pub mod app;
pub mod config;
pub mod modules;
pub mod routes;

pub use app::App;
