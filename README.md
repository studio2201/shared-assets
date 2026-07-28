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

- **Axum Security Middleware**: Standardized security headers, origin validation, and PIN brute-force protection.
- **Yew UI Design Tokens**: Harmonious color palettes and responsive layout components.
- **Zero-Dependency Rust Core**: High-efficiency shared logic compiled into all studio2201 applications.
- **Maximum DRY (v3.1+)**: session id generation, cookie builders, rate limiter, and origin helpers live here. Apps supply cookie **names** and product domain code only.

### Pin policy

Always pin `shared-core`, `shared-backend`, and `shared-frontend` to the **same git tag**:

```toml
shared-core     = { git = "https://github.com/studio2201/shared-assets.git", tag = "v3.1.0" }
shared-backend  = { git = "https://github.com/studio2201/shared-assets.git", tag = "v3.1.0" }
shared-frontend = { git = "https://github.com/studio2201/shared-assets.git", tag = "v3.1.0" }
```

Never mix tags inside one app (duplicate `shared-core` graphs and type identity bugs).

---

### License

Distributed under the Apache 2.0 License. See [LICENSE](LICENSE) for details.
