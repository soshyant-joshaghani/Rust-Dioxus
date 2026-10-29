//! `{"detail": ...}` parsing: the contract's string form and FastAPI's list form.

use rust_dioxus_client::modules::base::utils::api_error::format_api_error;
use serde_json::json;

#[test]
fn string_detail() {
    let body = json!({ "detail": "Incorrect email or password" });
    assert_eq!(format_api_error(&body, "fallback"), "Incorrect email or password");
}

#[test]
fn list_detail_joins_messages() {
    let body = json!({ "detail": [{ "msg": "field required" }, { "msg": "too short" }] });
    assert_eq!(format_api_error(&body, "fallback"), "field required, too short");
}

#[test]
fn missing_or_unknown_detail_uses_fallback() {
    assert_eq!(format_api_error(&json!({}), "fallback"), "fallback");
    assert_eq!(format_api_error(&json!(null), "fallback"), "fallback");
    assert_eq!(format_api_error(&json!({ "detail": 42 }), "fallback"), "fallback");
}
