//! CSRF origin-check helpers shared by every state-changing handler.
//!
//! Apps wrap these in an `axum::middleware::from_fn_with_state` adapter
//! that supplies their own `AuthState`-shaped state. This module only
//! contains the pure helpers that don't need a concrete state type.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Env-var enabling the `Origin: null` bypass for `curl` clients.
/// Defaults to `false`.
pub const ALLOW_NULL_ORIGIN_ENV: &str = "ALLOW_NULL_ORIGIN";

/// Truthy values: `"true"`, `"1"`, `"on"` (case-insensitive).
#[must_use]
pub fn allow_null_origin_from_env() -> bool {
    matches!(
        std::env::var(ALLOW_NULL_ORIGIN_ENV).ok().as_deref(),
        Some("true" | "TRUE" | "True" | "1" | "on" | "ON" | "On")
    )
}

/// Standard 403 response used by every origin-check failure.
#[must_use]
pub fn forbidden_response() -> Response {
    (
        StatusCode::FORBIDDEN,
        axum::Json(serde_json::json!({ "error": "forbidden" })),
    )
        .into_response()
}

/// Match `origin` against `base`. Pass when exact, or when `base` is
/// on `localhost`/`127.0.0.1` and the origin is the same scheme+host
/// on any port (developer ergonomics).
#[must_use]
pub fn origin_matches(origin: &str, base: &str) -> bool {
    if origin == base {
        return true;
    }
    for prefix in ["http://localhost", "http://127.0.0.1"] {
        if !base.starts_with(prefix) {
            continue;
        }
        if let Some(rest) = origin.strip_prefix(prefix) {
            if rest.is_empty() {
                return true;
            }
            if let Some(port) = rest.strip_prefix(':')
                && !port.is_empty()
                && port.chars().all(|c| c.is_ascii_digit())
            {
                return true;
            }
        }
    }
    false
}

/// Strip the scheme from an `Origin` header value, returning the host
/// (or `host:port`) portion.
#[must_use]
pub fn strip_scheme(origin: &str) -> &str {
    origin
        .strip_prefix("http://")
        .or_else(|| origin.strip_prefix("https://"))
        .unwrap_or(origin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_matches_exact() {
        assert!(origin_matches("https://app.example", "https://app.example"));
    }

    #[test]
    fn origin_matches_localhost_any_port() {
        assert!(origin_matches(
            "http://localhost:3000",
            "http://localhost:8080"
        ));
        assert!(origin_matches("http://127.0.0.1:3000", "http://127.0.0.1"));
    }

    #[test]
    fn origin_does_not_match_cross_origin() {
        assert!(!origin_matches(
            "https://attacker.example",
            "https://app.example"
        ));
        assert!(!origin_matches("http://app.example", "https://app.example"));
    }

    #[test]
    fn origin_does_not_match_non_localhost_with_port() {
        assert!(!origin_matches(
            "https://app.example:443",
            "https://app.example"
        ));
    }

    #[test]
    fn strip_scheme_works() {
        assert_eq!(strip_scheme("http://host"), "host");
        assert_eq!(strip_scheme("https://host:443"), "host:443");
        assert_eq!(strip_scheme("host"), "host");
    }

    #[test]
    fn forbidden_response_is_403() {
        let resp = forbidden_response();
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }
}
