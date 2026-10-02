//! Pure helpers, tested in tests/frontend/api_url.rs (port of Rust-Svelte's `api-url.ts`).

/// Trim whitespace and the trailing slash; use `fallback` when unset or empty.
pub fn normalize_api_base_url(value: Option<&str>, fallback: &str) -> String {
    let raw = value.map(str::trim).filter(|v| !v.is_empty()).unwrap_or(fallback);
    raw.trim_end_matches('/').to_string()
}

/// `http://localhost:8000/...` or `http://api.localhost/...`: the local dev API.
pub fn is_local_dev_api_url(configured: &str) -> bool {
    let rest = match configured
        .strip_prefix("http://")
        .or_else(|| configured.strip_prefix("https://"))
    {
        Some(rest) => rest,
        None => return false,
    };
    let host = rest.split('/').next().unwrap_or_default();
    host == "localhost:8000" || host == "api.localhost" || host.starts_with("api.localhost:")
}

/// Final base URL.
///
/// In a browser (`origin` is set) a relative base becomes `origin + base`, and the local dev API is
/// replaced by the same-origin `/api/v1` the dx proxy serves (no CORS in dev). Native clients keep
/// absolute URLs as they are; a relative base on native is joined to the local dev API.
pub fn resolve_api_base_url(configured: &str, origin: Option<&str>) -> String {
    let base = normalize_api_base_url(Some(configured), "/api/v1");
    match origin {
        Some(origin) => {
            let origin = origin.trim_end_matches('/');
            if !base.starts_with("http") {
                format!("{origin}/{}", base.trim_start_matches('/'))
            } else if is_local_dev_api_url(&base) {
                format!("{origin}/api/v1")
            } else {
                base
            }
        }
        None => {
            if base.starts_with("http") {
                base
            } else {
                format!("http://127.0.0.1:8000/{}", base.trim_start_matches('/'))
            }
        }
    }
}
