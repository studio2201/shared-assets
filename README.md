<h1 align="center">
  <img src="assets/icon.png?v=1.0.31" width="48" height="48" valign="middle"> Shared Assets
</h1>

<p align="center">
  <b>Centralized design system tokens, CSS utilities, icons, and shared Rust crates for studio2201 applications.</b>
</p>

---

### Shared Sub-Crates & Assets

This repository provides reusable core building blocks for the entire studio2201 org:

- `shared-core`: Common data types, internationalization (i18n) tables, and domain validation.
- `shared-backend`: Axum security middleware, PIN authentication, CORS, HSTS, and rate-limiting.
- `shared-frontend`: Yew UI components, theme definitions, and CSS design system utilities.
- `styles/`: Global CSS design tokens, glassmorphism utilities, and color themes.

---

### Architecture & Security

- **Independent products**: each app installs, runs, and upgrades alone. See [INDEPENDENCE.md](INDEPENDENCE.md).
- **Axum Security Middleware**: optional CORS / HSTS / security headers / origin helpers.
- **Yew UI Design Tokens**: themes, Header / Footer (optional Login).
- **App-local auth secrets path**: session IDs and cookie builders live **in each app** (isolation). Shared `session_id` / `cookie_auth` are deprecated convenience only.
- **Shared utilities**: rate limiter, PIN attempt counters, server config, tracing — optional.

### What to take from this repo

| Use shared | Keep in the app |
|------------|-----------------|
| Themes + layout CSS | Session ID generation |
| Header / Footer | Cookie name + builders |
| CORS / HSTS / security headers | `verify_pin` / logout handlers |
| Origin-check helpers | Domain models & routes |
| RateLimiter (optional) | Deploy (Docker / Unraid) |
| ServerConfig / tracing | App CSS & product i18n |

### Pin policy (within one app only)

Inside a single app, pin `shared-core` / `shared-backend` / `shared-frontend` to the **same** tag (avoid dual crate graphs). **Different apps may use different tags.**

```toml
shared-core     = { git = "https://github.com/studio2201/shared-assets.git", tag = "v3.2.0" }
shared-backend  = { git = "https://github.com/studio2201/shared-assets.git", tag = "v3.2.0" }
shared-frontend = { git = "https://github.com/studio2201/shared-assets.git", tag = "v3.2.0" }
```

---

### License

Distributed under the Apache 2.0 License. See [LICENSE](LICENSE) for details.
