//! Where the client calls the API. Same variable as Rust-Svelte: `PUBLIC_API_BASE_URL`.
//!
//! Resolution order:
//! 1. native only: `PUBLIC_API_BASE_URL` in the process environment at run time (point a packaged app elsewhere)
//! 2. `PUBLIC_API_BASE_URL` at compile time (`dx bundle` in Docker passes the production URL)
//! 3. default: web `/api/v1` (same origin, proxied by dx in dev), native `http://127.0.0.1:8000/api/v1`
//!
//! Android reaches the host's 127.0.0.1:8000 through `adb reverse tcp:8000 tcp:8000` (`native run android` sets it).

pub mod api_url;

use api_url::{normalize_api_base_url, resolve_api_base_url};

pub const WEB_DEFAULT_API_BASE_URL: &str = "/api/v1";
pub const NATIVE_DEFAULT_API_BASE_URL: &str = "http://127.0.0.1:8000/api/v1";

fn configured() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    if let Ok(value) = std::env::var("PUBLIC_API_BASE_URL") {
        if !value.trim().is_empty() {
            return normalize_api_base_url(Some(&value), NATIVE_DEFAULT_API_BASE_URL);
        }
    }
    let fallback = if cfg!(target_arch = "wasm32") {
        WEB_DEFAULT_API_BASE_URL
    } else {
        NATIVE_DEFAULT_API_BASE_URL
    };
    normalize_api_base_url(option_env!("PUBLIC_API_BASE_URL"), fallback)
}

/// Absolute API base URL, e.g. `http://dashboard.localhost/api/v1` or `https://api.example.com/api/v1`.
pub fn api_base_url() -> String {
    resolve_api_base_url(&configured(), browser_origin().as_deref())
}

/// `<base>/<path>` with exactly one slash between them.
pub fn api_url(path: &str) -> String {
    format!("{}/{}", api_base_url(), path.trim_start_matches('/'))
}

#[cfg(target_arch = "wasm32")]
fn browser_origin() -> Option<String> {
    web_sys::window()?.location().origin().ok()
}

#[cfg(not(target_arch = "wasm32"))]
fn browser_origin() -> Option<String> {
    None
}

/// Which client is running: `web`, `windows`, `macos`, `linux`, `android`, `ios`.
pub fn platform_name() -> &'static str {
    if cfg!(target_arch = "wasm32") {
        "web"
    } else {
        std::env::consts::OS
    }
}
