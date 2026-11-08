# Architecture

Rust-Dioxus follows the FoxG folder contract. The client lives in `frontend/`: one Dioxus 0.7 crate that builds for web (wasm32), desktop (Windows, macOS, Linux), and mobile (Android, iOS).

```text
rust-dioxus/
├── frontend/                Dioxus client: src/{config,modules/{base,apps},routes}
├── backend/                 `backend/src/modules/{apps,base,system}`, `backend/src/core`, and `backend/src/bin/{api,worker}.rs`
├── tests/                   backend/ (Axum integration), frontend/ (client logic), contract/
├── traefik/
├── docs/
├── __plans__/
└── __ctrl__/                Python CLI
```

Request flow: Route (Axum handler) → Service → Repository (trait, SQLx) → PostgreSQL. The backend crate is one library plus two binaries (`api`, `worker`), identical to Rust-Svelte's.

The client never links the backend. Every client talks to the API over HTTP with the [wire contract](../../../../CONTRACT.md): `/api/v1`, `snake_case` JSON, `{"detail": "..."}` errors, form login, JWT HS256. That keeps `frontend/` usable over any FoxG backend.

```text
Dioxus client (web | desktop | android | ios)
  routes/ page → modules/<group>/<name>/api.rs → base/utils/http.rs (reqwest)
        │  HTTP /api/v1
        ▼
Axum API → Service → Repository → PostgreSQL     (Redis: cache + jobs)
```

| Client | Renderer | HTTP |
|--------|----------|------|
| Web | dioxus-web (wasm32) | browser fetch, same origin `/api/v1` |
| Desktop | WebView2 / WKWebView / WebKitGTK | hyper + rustls from Rust |
| Android, iOS | system WebView | hyper + rustls from Rust |

Data is PostgreSQL with the Fast schema. Redis is the cache and the job queue. Both degrade softly: a missing Redis never fails a request.
