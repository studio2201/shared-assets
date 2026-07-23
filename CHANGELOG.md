# Changelog

All notable changes to `shared-assets` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.0.35] - 2026-07-23

### Added

- **`shared_backend::rate_limit::RateLimiter`** — per-IP sliding-window
  in-memory request-budget limiter with two constructors
  (`new()` default 100 req / 60s; `with_limits()` for tests) and
  `check`, `cleanup`, `tracked_ips` methods. Five unit tests cover
  under/over-budget, per-IP isolation, window expiry, idle cleanup.
  Promoted from defend/scan/snake's identical `services/rate_limit.rs`.
- **`shared_backend::session_id::generate_session_id`** — 32-character
  lowercase hex session id drawn from `OsRng` (CSPRNG) with a
  thread-local RNG fallback (logged on use). Two unit tests cover
  format and uniqueness across 256 generations.
- **`shared_backend::cookie_auth::{build_cookie, build_clear_cookie}`** —
  single source of truth for `HttpOnly` + `SameSite=Strict` + path auth
  cookies, with the `cookie_max_age_hours` clamping range `[1 minute, 30 days]`.
  Accepts the cookie name as an argument so each app keeps its own
  brand prefix. Five unit tests cover cookie semantics and clamps.
- **`shared_frontend::components::{Login, LoginProps}`** — generic
  numeric-PIN input form with auto-submit and focus management.
  Decoupled from any HTTP client: the parent receives `on_verify(String)`
  and decides how to call the verify-pin API. Replaces the seven
  per-app `components/pin.rs` files (207 LoC avg).

### Added (workspace deps)

- `rand = { version = "0.9", features = ["std", "thread_rng"] }`
- `time = { version = "0.3", features = ["std"] }`
- `axum-extra = { version = "0.10", features = ["cookie"] }`

## [3.0.34] - 2026-07-23

### Changed

- **Workspace `Cargo.toml`** — added `[workspace.package]` and
  `[workspace.dependencies]` blocks. All three sub-crates now inherit
  `version`, `edition`, `license`, and dependencies via the workspace
  (`.workspace = true` keys). New release bumps are a one-line change
  in the workspace manifest.
- **Feature flags** — every sub-crate now exposes a `[features]` table
  (`default = []` for `shared-core` and `shared-backend`;
  `default = ["csr"]` for `shared-frontend`). `shared-backend` adds a
  `rate_limit` feature for the new request-budget limiter (added in
  the next release). `shared-frontend` adds `csr`, `locale`, and
  `notifier` features so apps can opt in granularly.
- **`web-sys` features** — `shared-frontend` enables `Element`,
  `KeyboardEvent`, `MouseEvent`, `HtmlInputElement`, and
  `HtmlOptionElement` so the `Header`'s keyboard listener compiles
  without depending on transitively-enabled features from Yew. Closes
  the latent build-break risk flagged by the v3.0.33 audit.
- **`shared_frontend::storage::DEFAULT_COOKIE_NAME`** is now `pub const`
  (was a private const). Apps that need a custom cookie name can read
  the default and shadow it locally.

### Refactored

- **`i18n/strings` (454 → 216/45/45/229 LoC)** — the giant `match` over
  22 `StringKey` × 8 `Language` translations is split into three
  private data submodules (`tooltips`, `aria`, `statuses`). The
  public surface (`StringKey`, `all()`, `lookup()`) is unchanged.
- **`i18n/common_strings` (299 → 141/81/108/55 LoC)** — split into
  `verbs`, `outcomes`, and `session_ui` data submodules. Public
  surface (`CommonString`, `lookup()`) is unchanged.
- **`components/header` (274 → 177/180 LoC)** — the four control-rendering
  helpers (`language_picker`, `theme_toggle`, `print_button`,
  `logout_button`, `tooltip_or_override`) move to a `controls` submodule.
  Pure-logic helpers `print_button_disabled` and
  `logout_button_disabled` extracted for unit-testing.
- **`shared_frontend::locale::parse_lang_cookie`** — pure helper
  extracted from `get_saved_locale` so cookie parsing is testable
  without a DOM.
- **`shared_frontend::storage::parse_cookie_value` + `unquote`** — pure
  helpers extracted from `get_item` so cookie/localStorage parsing is
  testable without a DOM.

### Added

- **Tests** — `database::establish_connection` (sqlite + WAL + FK),
  `helpers::{redacted_url, is_loopback_bind, MemoryEventLogger, LogEntry}`,
  `utils::mask_api_key`, `locale::parse_lang_cookie`,
  `storage::{parse_cookie_value, unquote}`, and the four
  `Header::controls` disabled-state helpers all gain unit tests.
  Test count: 12 → 109 passing tests.
- **`tempfile = "3"` and `serde_json = "1.0"`** as
  `[dev-dependencies]` of `shared-backend` to support the new tests.

### Fixed

- **Clippy `collapsible_if`** — collapsed the nested `if let` chain
  in `Header`'s keyboard handler.
- **Clippy `redundant_closure`** — `Closure::wrap(Box::new(callback))`
  in `EventListener::new`.
- **Clippy `needless_borrows_for_generic_args`** — `pragma_update`
  calls no longer pass `&"WAL"` / `&"ON"`.
- **Clippy `unused-mut`** — `EventListener::new` no longer marks
  `callback` mutable.
- **Unused `let` bindings** in `Footer` (`github_url`, `coffee_url`,
  `aria_github`) renamed to `_`-prefixed because the original code
  bound them without rendering them.

### CI

- **Workflow renamed** `Run Tests` → `CI`, split into four jobs:
  `lint` (fmt + clippy with `-D warnings`), `wasm-build` (verifies
  `shared-frontend` builds for `wasm32-unknown-unknown`),
  `deny` (`cargo deny`), and `test` (cargo test). Toolchain pinned
  to `1.96.0` to match `rust-toolchain.toml`.

## [3.0.19] - 2026-07-09

### Security
- HTML-escape `SITE_TITLE` before injection into HTML responses
- Add `Secure` flag on session cookies when `base_url` is HTTPS
- Default `ALLOWED_ORIGINS` to empty (fail-closed); only explicit `*` is permissive
- Empty `ALLOWED_ORIGINS` no longer enables `CorsLayer::permissive()`

## [3.0.18] - 2026-07-02

### Added

- **`shared_frontend::storage::StorageService`** — localStorage helper
  with `super_metroid_theme` cookie mirror for the `theme` key.
  Replaces per-app `frontend/src/storage.rs` (was duplicated across
  beam, pad, todo, trace, grid, pulse, snake).
- **`shared_frontend::locale`** — browser locale detection and persistence
  (`detect_browser_locale`, `get_saved_locale`, `set_saved_locale`).
  Pure Rust, no Yew deps, so Leptos apps (aura) can call it too.
- **`shared_frontend::components::LanguageSwitcher`** — Yew `<select>`
  component that renders the 8-language dropdown and persists the
  choice via `set_saved_locale`. Replaces the per-app language
  switcher JSX.
- **`shared_frontend::i18n::common_strings`** — base set of common UI
  strings (Cancel, Save, Delete, Confirm, Loading, Error, Failed,
  Success, Close, Yes, No, Back, Settings, Logout, Print, Theme,
  Language) in all 8 languages. Apps can use these directly via
  `common_strings::lookup(CommonString::Cancel, language)`.
- **`shared_backend::tracing_init::init_tracing`** — EnvFilter +
  optional file logging initialization. Replaces per-app inline
  tracing setup (was duplicated across beam, pad, todo, trace, grid,
  pulse, Rustle). Adopts snake's polished factored design.

### Changed

- All three crate versions bumped `3.0.13` → `3.0.18`.
- `shared_frontend::web-sys` features extended with `HtmlDocument` (for
  cookie getter/setter on `Document`).
- `shared_backend` gains `tracing-subscriber = { version = "0.3",
  features = ["env-filter"] }` as a direct dependency.

### Migration

Consumers (beam, pad, todo, trace, grid, pulse, snake, Rustle, aura)
should bump their shared-assets pin from `tag = "v3.0.17"` to
`tag = "v3.0.18"` and delete their per-app duplicates of the modules
above. See each app's PR for the exact diff.

## [3.0.1] - 2026-06-28

### Added

- **`shared_core::types`** — new module with wire-format / on-disk data
  types shared between the Yew frontend and the axum backend:
  - `TodoItem` — single todo record (`id`, `text`, `completed`)
  - `TodoLists` — `HashMap<list_name, Vec<TodoItem>>` (alias)
  - `SiteConfig` — `GET /api/config` response (`siteTitle`, `singleList`,
    `enableThemes`, `enablePrint`, `showVersion`, `showGithub`,
    camelCase JSON)
  - `PinRequiredResponse` — `GET /api/pin-required` response
  - `VerifyPinRequest` / `VerifyPinResponse` — `POST /api/verify-pin`
    request and (camelCase) response, with `Option`-typed fields that
    serialize only on failure
  - 4 unit tests covering round-trip JSON and default-value behavior
- 72 unit tests across the workspace (was 68)

### Why a 3.0.1 (not 3.1.0)

The new module is purely additive: no breaking changes to the public API
shipped in v3.0.0, no `Cargo.toml` changes required for consumers that
don't import the new types. Apps that previously vendored these types in
their own crate (notably `todo`) can now depend on `shared-core` and
delete their copies.

## [3.0.0] - 2026-06-26

### ⚠ BREAKING CHANGES

This release splits the previous single `shared-assets` crate into a
3-crate Cargo workspace. Consumers must update their `Cargo.toml` to
depend on each of the three crates and update `use` paths accordingly.

#### Migration guide

**Before (2.x):**
```rust
use shared_assets::print_unauthorized_console_message;
use shared_assets::header::Header;

shared_assets::print_unauthorized_console_message();
```

**After (3.x):**
```rust
use shared_backend::security::print_unauthorized_console_message;
use shared_frontend::components::Header;

shared_backend::security::print_unauthorized_console_message();
```

**Cargo dependency:**
```toml
# Before (2.x):
shared-assets = { path = "...", features = ["frontend"] }

# After (3.x): three crates, pinned to tag v3.0.0:
shared-core    = { git = "https://github.com/studio2201/shared-assets", tag = "v3.0.0" }
shared-backend = { git = "https://github.com/studio2201/shared-assets", tag = "v3.0.0" }
shared-frontend = { git = "https://github.com/studio2201/shared-assets", tag = "v3.0.0" }

# Or for local development:
shared-core    = { path = "Assets/shared-assets/shared-rust/shared-core" }
shared-backend = { path = "Assets/shared-assets/shared-rust/shared-backend" }
shared-frontend = { path = "Assets/shared-assets/shared-rust/shared-frontend" }
```

#### New modules

- **`server`** (in `shared-backend`) — Backend server primitives
  - `ServerConfig` — common env-driven config (port, pin, attempts, cookie age, CORS, enable_*, show_*, trust_proxy)
  - `server::serve` — bind + graceful shutdown on SIGINT/SIGTERM
  - `server::ServerError` — `IntoResponse` error type with HTTP status mapping
  - `server::ip::get_client_ip` — trusted-proxy-aware client IP extraction
  - `server::version::CARGO_PKG_VERSION` — re-export of the consuming crate's version

- **`auth`** (in `shared-backend`) — PIN authentication
  - `auth::pin_auth_layer` — axum middleware that gates routes behind a PIN
  - `auth::attempts::{is_locked_out, record_attempt, reset_attempts, lockout_remaining_secs}`
  - `auth::session::issue_cookie` — session cookie helpers

- **`middleware`** (in `shared-backend`) — Shared axum middleware factories
  - `middleware::cors_layer` — CORS layer from `ALLOWED_ORIGINS`
  - `middleware::security_headers_layer` — CSP, X-Frame-Options, etc.
  - `middleware::title_injection_layer` — `{{SITE_TITLE}}` → config
  - `middleware::hsts_layer` — HSTS when HTTPS

#### Removed

- The 2.x single-crate layout. There is no longer a `shared-assets`
  crate; the repository now ships the three crates `shared-core`,
  `shared-backend`, `shared-frontend`. There is no top-level
  `print_unauthorized_console_message` re-export — use
  `shared_backend::security::print_unauthorized_console_message`.

#### Changed

- Bumped edition 2021 → 2024 (let-chains used throughout)
- `web-sys` pinned to `=0.3.98` (matches the Yew 0.23 expected version)
- `ipnet`, `tokio`, `tower-http`, `axum`, `thiserror`, `anyhow`, `dotenvy`,
  `constant_time_eq`, `tracing`, `http-body-util` are now direct dependencies
  of `shared-backend` (consumers don't need to declare them just to use
  the new shared modules)
- `Header` prop API: `disable_print` + `enable_print` collapsed into a
  single `print_disabled: bool`; `on_print` is now `Option<Callback<…>>`

### Added

- 68 unit tests (was 22)
- `cargo clippy` clean, `cargo fmt` clean
- 27 `.rs` files all ≤ 250 lines

## [2.1.1] - 2026-06-25

Last 2.x release. Provides Yew components, theme management, and i18n only.

- `components::Header`, `components::Footer` — Yew UI chrome
- `theme::Theme` — Super Metroid theme enum (Crateria, Brinstar, Norfair, WreckedShip, Maridia, Tourian)
- `theme::mapping::Scheme` — User-facing scheme (light/sepia/dracula/nord) → Theme mapping
- `i18n::Language` — 8-language enum (en/zh/es/de/ja/fr/pt/ru)
- `i18n::strings::lookup` — Centralized UI string lookup
- `security::print_unauthorized_console_message` — anti-shell alert (also re-exported at crate root for 2.x compat)