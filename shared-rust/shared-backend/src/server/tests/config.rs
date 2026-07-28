use super::{with_clean_env, with_env};
use crate::server::ServerConfig;
use std::env;

#[test]
fn defaults_when_no_env_set() {
    with_clean_env(
        &[
            "PORT",
            "SITE_TITLE",
            "BASE_URL",
            "ALLOWED_ORIGINS",
            "BEAM_PIN",
            "ENABLE_TRANSLATION",
            "ENABLE_THEMES",
            "ENABLE_PRINT",
            "MAX_ATTEMPTS",
            "LOCKOUT_TIME_MINUTES",
            "COOKIE_MAX_AGE_HOURS",
            "TRUST_PROXY",
            "TRUSTED_PROXY_IPS",
            "BEAM_SITE_TITLE",
            "BEAM_TITLE",
        ],
        || {
            let cfg = ServerConfig::from_env("BEAM");
            assert_eq!(cfg.port, 4401);
            assert_eq!(cfg.site_title, "BEAM");
            assert_eq!(cfg.base_url, "http://localhost:4401");
            // Empty default is fail-closed for CORS (set ALLOWED_ORIGINS=* deliberately).
            assert_eq!(cfg.allowed_origins, "");
            assert!(cfg.pin.is_none());
            assert!(!cfg.enable_translation);
            assert!(cfg.enable_themes);
            assert!(cfg.enable_print);
            assert!(cfg.show_version);
            assert!(cfg.show_github);
            assert_eq!(cfg.max_attempts, 5);
            assert_eq!(cfg.lockout_time_minutes, 15);
            assert_eq!(cfg.cookie_max_age_hours, 24);
        },
    );
}

#[test]
fn pin_prefix_lookup_order() {
    with_clean_env(
        &[
            "PIN",
            "BEAM_PIN",
            "SITE_TITLE",
            "BEAM_SITE_TITLE",
            "BEAM_TITLE",
        ],
        || {
            unsafe { env::set_var("PIN", "12345678") };
            assert_eq!(
                ServerConfig::from_env("BEAM").pin.as_deref(),
                Some("12345678")
            );

            // Prefix wins, and value must still be numeric digits.
            unsafe { env::set_var("BEAM_PIN", "99998888") };
            assert_eq!(
                ServerConfig::from_env("BEAM").pin.as_deref(),
                Some("99998888"),
                "prefix wins"
            );
        },
    );
}

#[test]
fn pin_rejected_when_too_short() {
    with_clean_env(&["BEAM_PIN"], || {
        unsafe { env::set_var("BEAM_PIN", "123") };
        assert!(ServerConfig::from_env("BEAM").pin.is_none());
    });
}

#[test]
fn pin_rejected_when_non_numeric() {
    // Regression: operators set MARK_PIN=test and the Login UI (digits-only)
    // could never match. ServerConfig must refuse non-digit PINs.
    with_clean_env(&["BEAM_PIN", "PIN"], || {
        for bad in ["test", "abc1", "12ab", "12 34", "12-34", "１２３４"] {
            unsafe { env::set_var("BEAM_PIN", bad) };
            assert!(
                ServerConfig::from_env("BEAM").pin.is_none(),
                "expected non-numeric PIN {bad:?} to be ignored"
            );
        }
    });
}

#[test]
fn pin_accepted_when_numeric_in_range() {
    with_clean_env(&["BEAM_PIN", "PIN"], || {
        for good in ["1234", "0000", "9876543210"] {
            unsafe { env::set_var("BEAM_PIN", good) };
            assert_eq!(
                ServerConfig::from_env("BEAM").pin.as_deref(),
                Some(good),
                "expected numeric PIN {good:?} to be accepted"
            );
        }
    });
}

#[test]
fn parse_numeric_pin_unit_rules() {
    use crate::server::config::{is_valid_numeric_pin, parse_numeric_pin};

    assert_eq!(parse_numeric_pin("1234").as_deref(), Ok("1234"));
    assert_eq!(parse_numeric_pin("  5678  ").as_deref(), Ok("5678"));
    assert!(parse_numeric_pin("test").is_err());
    assert!(parse_numeric_pin("12a4").is_err());
    assert!(parse_numeric_pin("123").is_err());
    assert!(parse_numeric_pin(&"1".repeat(65)).is_err());
    assert!(is_valid_numeric_pin("4242"));
    assert!(!is_valid_numeric_pin("test"));
}

#[test]
fn site_title_prefix_lookup_order() {
    with_clean_env(&["SITE_TITLE", "BEAM_SITE_TITLE", "BEAM_TITLE"], || {
        unsafe { env::set_var("SITE_TITLE", "FromGeneric") };
        assert_eq!(ServerConfig::from_env("BEAM").site_title, "FromGeneric");

        unsafe { env::set_var("BEAM_TITLE", "FromTitle") };
        assert_eq!(
            ServerConfig::from_env("BEAM").site_title,
            "FromTitle",
            "_TITLE beats generic"
        );

        unsafe { env::set_var("BEAM_SITE_TITLE", "FromSiteTitle") };
        assert_eq!(
            ServerConfig::from_env("BEAM").site_title,
            "FromSiteTitle",
            "_SITE_TITLE beats _TITLE"
        );
    });
}

#[test]
fn booleans_truthy_values() {
    with_env(
        &[("ENABLE_TRANSLATION", "true"), ("ENABLE_THEMES", "on")],
        || {
            let cfg = ServerConfig::from_env("X");
            assert!(cfg.enable_translation);
            assert!(cfg.enable_themes);
        },
    );
}

#[test]
fn opt_out_booleans_default_true() {
    with_env(&[], || {
        let cfg = ServerConfig::from_env("X");
        assert!(cfg.show_version);
        assert!(cfg.show_github);
    });
    with_env(&[("SHOW_VERSION", "false")], || {
        assert!(!ServerConfig::from_env("X").show_version);
    });
}
