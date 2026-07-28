# Product independence

studio2201 apps are **independent products**.

- Installed separately (own container, Unraid template, port, data dir)
- Used separately (own process, config, lifecycle, failure domain)
- Upgraded separately (no requirement to bump all apps together)

`shared-assets` is an **optional build-time toolkit**, not a runtime platform and not a monorepo mandate.

## What belongs in shared-assets

| Keep shared | Why |
|-------------|-----|
| `styles/` themes + chrome CSS | Brand consistency without process coupling |
| `shared-frontend` Header / Footer / optional Login | Shared look; each app still owns its API calls |
| `shared-backend` CORS, HSTS, security headers, title injection | Same security baseline, pure middleware |
| PIN **attempt** lockout helpers | Generic IP counter; no product secret |
| `origin_check` pure helpers | Pure functions; app owns middleware wiring |
| `RateLimiter` | Generic utility; optional |
| `ServerConfig` / tracing bootstrap / `AppError` | Infrastructure helpers |
| `shared-core` i18n tables, PIN wire types, utilities | Cross-crate contracts only |

## What belongs in each app

| Keep local | Why |
|------------|-----|
| Session ID generation | Blast radius: a shared RNG bug must not hit every product at once |
| Cookie builders + cookie **names** | Product-specific names (`BEAM_PIN`, …) and SameSite policy |
| `verify_pin` / `logout` / `require_pin` handlers | Product state shape and response DTOs |
| Domain models & routes | The product itself |
| App CSS, app i18n | Product UX |
| Deploy (Dockerfile, compose, Unraid XML) | Independent install |

## Pin policy

- Apps **may** use the same `shared-assets` git tag, but **must not be forced** to.
- Prefer the same tag for `shared-core` / `shared-backend` / `shared-frontend` *within one app* (avoid dual graphs).
- Different apps may sit on different tags indefinitely.

## Adding a new app

1. Own repo, port (see [PORTS.md](PORTS.md)), container, template.
2. Pull shared chrome + middleware as needed.
3. **Copy or write** local session/cookie auth; do not treat shared session/cookie as required.
4. Ship and version alone.

## History

- **v3.0.39** — de-merged session/cookie for isolation.
- **v3.1.x** — temporarily re-centralized (Maximum DRY experiment).
- **v3.2.0** — independence is the policy again; session/cookie shared modules are deprecated.
