//! PIN-based authentication primitives shared by every companion app.
//!
//! Provides:
//!
//! - [`attempts`] — failed-attempt tracking with per-IP lockout
//! - [`middleware`] — axum middleware factory that gates routes behind a PIN
//! - [`origin_check`] — CSRF origin-check helpers
//! - [`session`] — cookie issuance helpers

pub mod attempts;
pub mod middleware;
pub mod origin_check;
pub mod session;

pub use attempts::{
    Attempt, attempts_left, current_attempts, is_locked_out, lockout_remaining_secs,
    record_attempt, reset_attempts,
};
pub use middleware::{PinState, pin_auth_layer};
pub use session::{issue_cookie, read_pin_cookie};
