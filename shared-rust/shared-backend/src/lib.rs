pub mod auth;
pub mod cookie_auth;
pub mod database;
pub mod middleware;
pub mod rate_limit;
pub mod security;
pub mod server;
pub mod session_id;
pub mod tracing_init;

// Re-export i18n from shared-core so backend can access it via crate::i18n / shared_assets::i18n
pub use shared_core::i18n;
pub use tracing_init::{LOG_DIR_ENV, default_log_dir, init_tracing};
