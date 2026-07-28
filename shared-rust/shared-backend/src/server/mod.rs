//! Backend server primitives shared by every studio2201 companion app.
//!
//! Provides:
//!
//! - [`ServerConfig`] — common env-driven configuration struct
//! - [`bootstrap::serve`] — bind + serve with graceful shutdown
//! - [`error::ServerError`] — `IntoResponse` error type
//! - [`ip`] — client IP extraction helpers
//! - [`version`] — `CARGO_PKG_VERSION` helper

pub mod bootstrap;
pub mod config;
pub mod error;
pub mod helpers;
pub mod ip;
pub mod version;

#[cfg(test)]
mod tests;

pub use bootstrap::serve;
pub use config::{
    PIN_MAX_LEN, PIN_MIN_LEN, ServerConfig, is_valid_numeric_pin, parse_numeric_pin,
};
pub use error::ServerError;
pub use helpers::{LogEntry, MemoryEventLogger, is_loopback_bind, redacted_url};
pub use ip::{get_client_ip, normalize_ip};
pub use version::CARGO_PKG_VERSION;
