# Progress

| Stage | Status | Note |
|-------|--------|------|
| Folder contract | done | frontend, backend, tests, traefik, docs, __plans__, __ctrl__ |
| Module groups | done | apps/sample, base/auth, base/users, system (backend and client) |
| Wire contract | done | Backend copied from Rust-Svelte; follows CONTRACT.md |
| Dioxus client | done | Dioxus 0.7.10: login/signup, dashboard, notes CRUD, admin users, theme, PWA |
| Client platforms | done | web, desktop (Windows/macOS/Linux), Android, iOS through dx; `native` CLI |
| Python CLI | done | dev (dx serve), native, test, app, prod, logs, flatten, remote |
| Redis cache and jobs | done | Soft-degrading cache, Redis list worker |
| Backend tests | done | In-memory fakes, no database needed |
| Client tests | done | tests/frontend (cargo test --no-default-features) + wasm32 check |
| Native packaging verified per host | pending | Run `native build all` on Windows, macOS, and Linux hosts |

Last update: 2026-10-02
