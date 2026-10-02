# Testing

```bat
__ctrl__\rust-dioxus-ctrl.bat test all
__ctrl__\rust-dioxus-ctrl.bat test backend
__ctrl__\rust-dioxus-ctrl.bat test frontend
__ctrl__\rust-dioxus-ctrl.bat test contract
```

`test backend` runs `cargo test` in `backend/`. Integration tests live in `tests/backend/*.rs`; `backend/Cargo.toml` points its `[[test]]` targets there. They inject in-memory repositories, cache, and job queue, so they need neither Postgres nor Redis. They cover auth, the superuser routes, notes isolation and caching, and the local-only private routes.

`test frontend` runs `cargo test --no-default-features` in `frontend/`. The tests live in `tests/frontend/*.rs` (API URL resolution, error parsing) and need no renderer. Then it runs `cargo check --target wasm32-unknown-unknown --features web`, the equivalent of `svelte-check`.

`test contract` runs `tests/contract/contract_test.py` against a running API (default `http://localhost:8000`).

Native platforms are checked by building them: `native build all` on each host OS.
