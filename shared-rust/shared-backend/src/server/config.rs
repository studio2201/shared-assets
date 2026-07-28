//! Common server configuration parsed from environment variables.
//!
//! Apps extend this with app-specific fields via composition or wrap it in
//! their own `AppConfig` that holds a `ServerConfig` plus extras.

use ipnet::IpNet;
use std::env;
use std::str::FromStr;

/// Configuration shared by every studio2201 companion app backend.
///
/// Constructed via [`ServerConfig::from_env`], which reads common variables
/// like `PORT`, `SITE_TITLE`, `ALLOWED_ORIGINS`, and PIN attempts/cookie
/// settings. App-specific values (e.g. `BEAM_UPLOAD_DIR`, `GRID_DATA_DIR`)
/// should be parsed in the app's own config struct.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub site_title: String,
    pub base_url: String,
    pub allowed_origins: String,
    pub pin: Option<String>,
    pub enable_translation: bool,
    pub enable_themes: bool,
    pub enable_print: bool,
    pub show_version: bool,
    pub show_github: bool,
    pub trust_proxy: bool,
    pub trusted_proxies: Vec<IpNet>,
    pub max_attempts: u32,
    pub lockout_time_minutes: u64,
    pub cookie_max_age_hours: i64,
    pub shutdown_drain_seconds: u64,
}

impl ServerConfig {
    /// Read the common configuration from process environment.
    ///
    /// `app_prefix` is the uppercase app identifier used to disambiguate
    /// app-specific PIN/SITE_TITLE overrides (e.g. `"BEAM"`, `"GRID"`).
    /// The lookup order for each value is:
    ///
    /// - PIN: `{PREFIX}_PIN` → `PIN`
    /// - SITE_TITLE: `{PREFIX}_SITE_TITLE` → `{PREFIX}_TITLE` → `SITE_TITLE`
    pub fn from_env(app_prefix: &str) -> Self {
        #[cfg(not(test))]
        {
            let _ = dotenvy::from_path("/app/data/.env");
            let _ = dotenvy::dotenv();
        }

        let prefix = app_prefix.to_ascii_uppercase();

        let port = parse_or("PORT", 4401u16);
        let site_title = first_nonempty_env(&[
            &format!("{prefix}_SITE_TITLE"),
            &format!("{prefix}_TITLE"),
            "SITE_TITLE",
        ])
        .unwrap_or_else(|| app_prefix.to_string());

        let base_url = env::var("BASE_URL").unwrap_or_else(|_| format!("http://localhost:{port}"));

        // Prefer an explicit allowlist. Operators who need open CORS set
        // ALLOWED_ORIGINS=* deliberately (compose files often do for LAN).
        // Default empty is fail-closed in [`crate::middleware::cors_layer`].
        let allowed_origins = env::var("ALLOWED_ORIGINS").unwrap_or_default();

        // PIN must be numeric-only (matches Login UI digit filter). Non-digit
        // values like "test" are rejected so operators cannot configure a PIN
        // the browser UI cannot enter.
        let pin = first_nonempty_env(&[&format!("{prefix}_PIN"), "PIN"]).and_then(|p| {
            match parse_numeric_pin(&p) {
                Ok(pin) => Some(pin),
                Err(reason) => {
                    tracing::warn!(
                        target: "config",
                        pin_preview = %mask_pin_for_log(&p),
                        %reason,
                        "ignoring invalid PIN env (numeric 4–64 digits required)"
                    );
                    None
                }
            }
        });

        let trust_proxy = parse_bool_env("TRUST_PROXY");
        let trusted_proxies = parse_trusted_proxies("TRUSTED_PROXY_IPS");

        Self {
            port,
            site_title,
            base_url,
            allowed_origins,
            pin,
            enable_translation: parse_bool_env("ENABLE_TRANSLATION"),
            enable_themes: parse_optout_bool_env("ENABLE_THEMES", true),
            enable_print: parse_optout_bool_env("ENABLE_PRINT", true),
            show_version: parse_optout_bool_env("SHOW_VERSION", true),
            show_github: parse_optout_bool_env("SHOW_GITHUB", true),
            trust_proxy,
            trusted_proxies,
            max_attempts: parse_or("MAX_ATTEMPTS", 5u32),
            lockout_time_minutes: parse_or("LOCKOUT_TIME_MINUTES", 15u64),
            cookie_max_age_hours: parse_or("COOKIE_MAX_AGE_HOURS", 24i64),
            shutdown_drain_seconds: parse_or("SHUTDOWN_DRAIN_SECONDS", 5u64),
        }
    }

    /// Returns `true` if PIN-based authentication is enabled.
    #[must_use]
    pub fn pin_enabled(&self) -> bool {
        self.pin.is_some()
    }

    /// Returns the lockout duration as a `std::time::Duration`.
    #[must_use]
    pub fn lockout_duration(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.lockout_time_minutes * 60)
    }
}

/// Minimum PIN length (digits). Shared with companion app verify handlers.
pub const PIN_MIN_LEN: usize = 4;
/// Maximum PIN length (digits).
pub const PIN_MAX_LEN: usize = 64;

/// Validate a companion-app access PIN.
///
/// Rules (must stay in lockstep with `shared_frontend::Login` input filter):
/// - length in [`PIN_MIN_LEN`]..=[`PIN_MAX_LEN`]
/// - **ASCII digits only** (`0`–`9`)
///
/// Returns the PIN string on success, or a short reason on failure.
pub fn parse_numeric_pin(raw: &str) -> Result<String, &'static str> {
    let pin = raw.trim();
    let len = pin.chars().count();
    if !(PIN_MIN_LEN..=PIN_MAX_LEN).contains(&len) {
        return Err("length must be 4–64");
    }
    if !pin.chars().all(|c| c.is_ascii_digit()) {
        return Err("must be digits only (0-9)");
    }
    Ok(pin.to_string())
}

/// `true` when `raw` is a valid numeric PIN (see [`parse_numeric_pin`]).
#[must_use]
pub fn is_valid_numeric_pin(raw: &str) -> bool {
    parse_numeric_pin(raw).is_ok()
}

fn mask_pin_for_log(raw: &str) -> String {
    let n = raw.chars().count();
    if n == 0 {
        return "(empty)".into();
    }
    format!("****({n} chars)")
}

/// Read an env var and parse to type `T`, falling back to `default` on missing/invalid.
fn parse_or<T>(name: &str, default: T) -> T
where
    T: FromStr,
{
    match env::var(name) {
        Ok(v) => match v.parse() {
            Ok(parsed) => parsed,
            Err(_) => {
                tracing::warn!(
                    target: "config",
                    "{name}={v:?} is not a valid value; using default",
                );
                default
            }
        },
        Err(_) => default,
    }
}

/// Read a boolean env var. Truthy = `"true"` or `"on"`.
fn parse_bool_env(name: &str) -> bool {
    env::var(name)
        .map(|v| v == "true" || v == "on")
        .unwrap_or(false)
}

/// Read an opt-out boolean: default true; `"false"` or `"off"` disables.
fn parse_optout_bool_env(name: &str, default: bool) -> bool {
    env::var(name)
        .map(|v| v != "false" && v != "off")
        .unwrap_or(default)
}

/// Try a list of env var names in order; return the first non-empty value.
fn first_nonempty_env(names: &[&str]) -> Option<String> {
    for name in names {
        if let Ok(v) = env::var(name)
            && !v.is_empty()
        {
            return Some(v);
        }
    }
    None
}

/// Parse a comma-separated list of CIDR/IP strings into `IpNet`s,
/// ignoring unparseable entries.
fn parse_trusted_proxies(name: &str) -> Vec<IpNet> {
    env::var(name)
        .ok()
        .map(|s| {
            s.split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .filter_map(|s| IpNet::from_str(s).ok())
                .collect()
        })
        .unwrap_or_default()
}
