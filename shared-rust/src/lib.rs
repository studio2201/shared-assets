//! Shared code for UberMetroid companion apps.
//!
//! Provides:
//!
//! - Yew components (frontend): [`components::Header`], [`components::Footer`]
//! - Theme management: [`theme::Theme`], [`theme::mapping::Scheme`]
//! - Internationalization: [`i18n::Language`], [`i18n::strings::lookup`]
//! - Backend primitives: [`server::ServerConfig`], [`server::serve`]
//! - Authentication: [`auth::pin_auth_layer`], [`auth::attempts`]
//! - Shared middleware: [`middleware::cors_layer`], [`middleware::security_headers_layer`],
//!   [`middleware::title_injection_layer`], [`middleware::hsts_layer`]
//! - Security helpers: [`security::print_unauthorized_console_message`]
//!
//! ## Cargo dependency
//!
//! ```toml
//! [dependencies]
//! shared-assets = { git = "https://github.com/UberMetroid/shared-assets", tag = "v3.0.0" }
//! ```
//!
//! ## Example: minimal backend
//!
//! ```no_run
//! use shared_assets::server::{ServerConfig, serve};
//! use shared_assets::middleware::{cors_layer, security_headers_layer};
//! use axum::{Router, routing::get};
//!
//! async fn run() {
//!     let config = ServerConfig::from_env("BEAM");
//!     let app = Router::new()
//!         .route("/health", get(|| async { "ok" }))
//!         .layer(axum::middleware::from_fn(security_headers_layer))
//!         .layer(cors_layer(&config));
//!
//!     serve(config, app).await.unwrap();
//! }
//! ```

pub mod auth;
pub mod i18n;
pub mod middleware;
pub mod security;
pub mod server;

#[cfg(feature = "frontend")]
pub mod components;

#[cfg(feature = "frontend")]
pub mod theme;

// Re-exports for ergonomics.
#[cfg(feature = "frontend")]
pub use components::{footer::Footer, header::Header};
