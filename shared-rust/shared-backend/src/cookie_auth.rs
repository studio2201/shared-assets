//! Build the authentication `Set-Cookie` value.
//!
//! # Deprecated (independence)
//!
//! Prefer **per-app** cookie builders with a baked-in cookie name
//! (`BEAM_PIN`, `DEFEND_PIN`, …) so cookie policy and blast radius stay
//! product-local. See `INDEPENDENCE.md`.
//!
//! These helpers remain available for gradual migration only.

#![deprecated(
    since = "3.2.0",
    note = "copy into each app for product independence; see INDEPENDENCE.md"
)]

use axum_extra::extract::cookie::{Cookie, SameSite};
use time::Duration;

/// One minute, in seconds — the floor for cookie lifetime when the
/// config value is nonsensical (zero or negative).
const MIN_LIFETIME_SECONDS: u64 = 60;
/// Thirty days, in seconds — the ceiling for cookie lifetime so a
/// misconfigured `cookie_max_age_hours` can't pin a session forever.
const MAX_LIFETIME_SECONDS: u64 = 30 * 24 * 3600;

/// Construct an authentication cookie with `SameSite=Strict` (default).
///
/// Prefer this for new apps. Beam and other special cases can call
/// [`build_cookie_with_same_site`] with [`SameSite::Lax`].
#[must_use]
pub fn build_cookie<'a>(
    cookie_name: &'a str,
    value: &str,
    max_age_hours: i64,
    secure: bool,
) -> Cookie<'a> {
    build_cookie_with_same_site(cookie_name, value, max_age_hours, secure, SameSite::Strict)
}

/// Construct an authentication cookie with an explicit SameSite policy.
#[must_use]
pub fn build_cookie_with_same_site<'a>(
    cookie_name: &'a str,
    value: &str,
    max_age_hours: i64,
    secure: bool,
    same_site: SameSite,
) -> Cookie<'a> {
    let max_age_seconds = clamp_seconds(max_age_hours.saturating_mul(3600));
    Cookie::build((cookie_name, value.to_string()))
        .path("/")
        .http_only(true)
        .secure(secure)
        .same_site(same_site)
        .max_age(Duration::seconds(max_age_seconds as i64))
        .build()
}

/// Build an expired cookie used to clear the session on logout (`SameSite=Strict`).
#[must_use]
pub fn build_clear_cookie<'a>(cookie_name: &'a str, secure: bool) -> Cookie<'a> {
    build_clear_cookie_with_same_site(cookie_name, secure, SameSite::Strict)
}

/// Build an expired clear-cookie with an explicit SameSite policy.
#[must_use]
pub fn build_clear_cookie_with_same_site<'a>(
    cookie_name: &'a str,
    secure: bool,
    same_site: SameSite,
) -> Cookie<'a> {
    Cookie::build((cookie_name, ""))
        .path("/")
        .http_only(true)
        .secure(secure)
        .same_site(same_site)
        .max_age(Duration::ZERO)
        .build()
}

/// Decide whether an auth cookie should be marked `Secure`. Trust, in
/// order:
///
/// 1. The `X-Forwarded-Proto: https` header (when the request is from a
///    trusted proxy, enforced by the caller).
/// 2. The configured `base_url` scheme as a fallback.
#[must_use]
pub fn cookie_should_be_secure(headers: &axum::http::HeaderMap, base_url: &str) -> bool {
    headers
        .get("x-forwarded-proto")
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.eq_ignore_ascii_case("https"))
        || base_url.starts_with("https")
}

fn clamp_seconds(seconds: i64) -> u64 {
    if seconds <= 0 {
        tracing::warn!(
            target: "cookie",
            seconds,
            "cookie lifetime non-positive; clamping to {MIN_LIFETIME_SECONDS}s"
        );
        return MIN_LIFETIME_SECONDS;
    }
    let unsigned = u64::try_from(seconds).unwrap_or(MAX_LIFETIME_SECONDS);
    if unsigned < MIN_LIFETIME_SECONDS {
        tracing::warn!(
            target: "cookie",
            seconds,
            "cookie lifetime too short; clamping to {MIN_LIFETIME_SECONDS}s"
        );
        return MIN_LIFETIME_SECONDS;
    }
    if unsigned > MAX_LIFETIME_SECONDS {
        tracing::warn!(
            target: "cookie",
            seconds,
            "cookie lifetime too long; clamping to {MAX_LIFETIME_SECONDS}s"
        );
        return MAX_LIFETIME_SECONDS;
    }
    unsigned
}

#[allow(deprecated)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_cookie_uses_session_id() {
        let c = build_cookie("APP_PIN", "deadbeef", 24, false);
        assert_eq!(c.name(), "APP_PIN");
        assert_eq!(c.value(), "deadbeef");
        assert_eq!(c.path(), Some("/"));
        assert!(c.http_only().unwrap_or(false));
        assert_eq!(c.secure(), Some(false));
        assert_eq!(c.same_site(), Some(SameSite::Strict));
    }

    #[test]
    fn auth_cookie_secure_flag_propagates() {
        let c = build_cookie("APP_PIN", "x", 24, true);
        assert_eq!(c.secure(), Some(true));
    }

    #[test]
    fn auth_cookie_same_site_lax() {
        let c = build_cookie_with_same_site("BEAM_PIN", "x", 24, false, SameSite::Lax);
        assert_eq!(c.same_site(), Some(SameSite::Lax));
    }

    #[test]
    fn clear_cookie_has_zero_max_age() {
        let c = build_clear_cookie("APP_PIN", false);
        assert_eq!(c.value(), "");
        assert_eq!(c.max_age(), Some(Duration::ZERO));
        assert_eq!(c.same_site(), Some(SameSite::Strict));
    }

    #[test]
    fn clear_cookie_same_site_lax() {
        let c = build_clear_cookie_with_same_site("BEAM_PIN", true, SameSite::Lax);
        assert_eq!(c.same_site(), Some(SameSite::Lax));
        assert_eq!(c.secure(), Some(true));
    }

    #[test]
    fn clamps_negative_hours_to_one_minute() {
        let c = build_cookie("APP_PIN", "x", -1, false);
        assert_eq!(
            c.max_age(),
            Some(Duration::seconds(MIN_LIFETIME_SECONDS as i64))
        );
    }

    #[test]
    fn clamps_overly_large_hours_to_thirty_days() {
        let c = build_cookie("APP_PIN", "x", 24 * 365 * 100, false);
        assert_eq!(
            c.max_age(),
            Some(Duration::seconds(MAX_LIFETIME_SECONDS as i64))
        );
    }

    #[test]
    fn cookie_secure_via_xfp() {
        let mut h = axum::http::HeaderMap::new();
        h.insert("x-forwarded-proto", "https".parse().unwrap());
        assert!(cookie_should_be_secure(&h, "http://example.com"));
    }

    #[test]
    fn cookie_secure_via_base_url() {
        let h = axum::http::HeaderMap::new();
        assert!(cookie_should_be_secure(&h, "https://app.example"));
        assert!(!cookie_should_be_secure(&h, "http://app.example"));
    }

    #[test]
    fn cookie_secure_handles_uppercase_https_header() {
        let mut h = axum::http::HeaderMap::new();
        h.insert("x-forwarded-proto", "HTTPS".parse().unwrap());
        assert!(cookie_should_be_secure(&h, "http://app.example"));
    }
}
