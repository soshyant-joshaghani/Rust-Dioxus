//! Port of Rust-Svelte's tests/frontend/config/api-base-url.test.ts.

use rust_dioxus_client::config::api_url::{is_local_dev_api_url, normalize_api_base_url, resolve_api_base_url};

#[test]
fn normalize_uses_fallback_and_trims_trailing_slash() {
    assert_eq!(normalize_api_base_url(None, "/api/v1"), "/api/v1");
    assert_eq!(normalize_api_base_url(Some(""), "/api/v1"), "/api/v1");
    assert_eq!(normalize_api_base_url(Some("  "), "/api/v1"), "/api/v1");
    assert_eq!(normalize_api_base_url(Some("https://api.example.com/api/v1/"), "/api/v1"), "https://api.example.com/api/v1");
}

#[test]
fn detects_local_dev_api() {
    assert!(is_local_dev_api_url("http://localhost:8000/api/v1"));
    assert!(is_local_dev_api_url("http://api.localhost/api/v1"));
    assert!(!is_local_dev_api_url("/api/v1"));
    assert!(!is_local_dev_api_url("https://api.example.com/api/v1"));
    assert!(!is_local_dev_api_url("http://localhost:5000/api/v1"));
}

#[test]
fn browser_joins_relative_base_to_origin() {
    assert_eq!(
        resolve_api_base_url("/api/v1", Some("http://dashboard.localhost")),
        "http://dashboard.localhost/api/v1"
    );
    assert_eq!(resolve_api_base_url("api/v1/", Some("http://localhost:5000/")), "http://localhost:5000/api/v1");
}

#[test]
fn browser_routes_local_dev_api_through_the_proxy() {
    assert_eq!(
        resolve_api_base_url("http://localhost:8000/api/v1", Some("http://dashboard.localhost")),
        "http://dashboard.localhost/api/v1"
    );
}

#[test]
fn browser_keeps_production_api() {
    assert_eq!(
        resolve_api_base_url("https://api.example.com/api/v1", Some("https://example.com")),
        "https://api.example.com/api/v1"
    );
}

#[test]
fn native_keeps_absolute_and_resolves_relative_to_local_api() {
    assert_eq!(resolve_api_base_url("http://127.0.0.1:8000/api/v1", None), "http://127.0.0.1:8000/api/v1");
    assert_eq!(resolve_api_base_url("https://api.example.com/api/v1", None), "https://api.example.com/api/v1");
    assert_eq!(resolve_api_base_url("/api/v1", None), "http://127.0.0.1:8000/api/v1");
}
