# Development

```bat
__ctrl__\rust-dioxus-ctrl.bat setup-local
__ctrl__\rust-dioxus-ctrl.bat dev run all
```

| Surface | URL |
|---------|-----|
| Dashboard | http://dashboard.localhost |
| API docs | http://api.localhost/docs |
| Scalar | http://api.localhost/sdoc |
| Direct API | http://localhost:8000/docs |
| dx (web client) | http://localhost:5000 |

The API runs with `cargo run --bin api`; the worker with `cargo run --bin worker`; the web client with `dx serve --platform web` (from `frontend/`). `dev run all --slim` skips Redis and the worker. Only one Traefik stack can bind port 80: stop the other proxy, or run `dev run apps` and use the direct URLs.

For AI-assisted work point the agent at [AGENTS.md](../AGENTS.md) and the `sample` module.

## Native clients

```bat
__ctrl__\rust-dioxus-ctrl.bat native doctor
__ctrl__\rust-dioxus-ctrl.bat native run desktop
__ctrl__\rust-dioxus-ctrl.bat native run android
```

Start the API first (`dev run all` or `dev run apps`). `native run` wraps `dx serve`, so edits to `rsx!` hot reload in the window, emulator, or simulator. The first build of each platform compiles the whole crate and takes a few minutes. Android needs the SDK, the NDK, and JDK 17+; `native doctor` shows what is missing.
