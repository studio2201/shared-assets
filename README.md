<h1 align="center">
  <img src="assets/icon.png?v=1.0.31" width="48" height="48" valign="middle"> Shared Assets
</h1>

<p align="center">
  <b>studio2201 Web UI kit + server helpers for every companion app.</b>
</p>

---

## Web UI kit (primary purpose)

**shared-assets owns the product chrome** — not product domain logic.

| Layer | Contents |
|-------|----------|
| **`styles/`** | Themes, body, shell, header, footer, login, buttons, cards, forms, notifications, print |
| **`shared-frontend`** | Yew: `AppShell`, `Header`, `Footer`, `Login`, `ToastContainer` / `ToastNotification` / `Banner`, language switcher, theme helpers |
| **`shared-core`** | Language enum, shared i18n tables, PIN wire types, utils |
| **`shared-backend`** | Security middleware, tracing, rate limit, server config (optional) |

### Styles entrypoints

Individual files (Trunk-friendly):

```text
styles/themes/themes.css
styles/components/body.css
styles/components/shell.css
styles/components/buttons.css
styles/components/cards.css
styles/components/forms.css
styles/components/notifications.css
styles/layout/header.css
styles/layout/footer.css
styles/pages/login.css
styles/pages/print.css
```

Or one import: `styles/kit.css` (if your bundler resolves `@import`).

### Sync styles into apps

```bash
./scripts/sync-web-ui.sh
# or one app:
./scripts/sync-web-ui.sh ../beam
```

That script vendors **`styles/` only** into `assets/shared-assets/styles/`.  
Do **not** vendor `shared-rust/` into apps (stale copies, unused by Cargo).

### App tab icons (favicon)

Each companion app owns its brand icon under **`assets/icon.png`** and must
ship the **same file** as **`assets/favicon.png`** for the browser tab.

- Primary `<link rel="icon">` must be the **PNG**, not a shared/stale SVG.
- Do **not** use the legacy red-check `favicon.svg` from old scaffolds.
- Validate with:

```bash
./scripts/check-app-icons.sh
./scripts/check-app-icons.sh ../mark
```

  
Rust crates come from the git tag; session/cookie helpers stay **app-local**.

### Yew usage

```rust
use shared_frontend::{
    AppShell, Footer, FooterProps, Header, HeaderProps, Login, LoginProps,
    ToastContainer, ToastNotification, ToastType,
};
```

Product screens (file explorer, game board, todo lists) stay **in the app**.  
Chrome (header/footer/login/toasts/theme) lives **here**.

### Non-Yew apps (StateSync / Maud)

Use **styles + shared-core + shared-backend**. Map theme CSS variables into the Maud layout (see StateSync). Do not depend on Yew components.

---

## Independence

Apps install and upgrade separately. See [INDEPENDENCE.md](INDEPENDENCE.md).  
Within one app, pin `shared-core` / `shared-backend` / `shared-frontend` to the **same tag**.

```toml
shared-core     = { git = "https://github.com/studio2201/shared-assets.git", tag = "v3.3.1" }
shared-backend  = { git = "https://github.com/studio2201/shared-assets.git", tag = "v3.3.1" }
shared-frontend = { git = "https://github.com/studio2201/shared-assets.git", tag = "v3.3.1" }
```

Session IDs and cookie builders stay **app-local** (auth identity blast radius).

**PIN values must be ASCII digits only (4–64).** Non-numeric env values
(e.g. `MARK_PIN=test`) are ignored at config parse time so the Login UI
(digit-only) can always enter a configured PIN.

---

### License

Apache 2.0. See [LICENSE](LICENSE).
