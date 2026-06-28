pub mod auth;
pub mod middleware;
pub mod server;
pub mod security;

// Re-export i18n from shared-core so backend can access it via crate::i18n / shared_assets::i18n
pub use shared_core::i18n;
