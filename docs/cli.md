# CLI

```bat
__ctrl__\rust-dioxus-ctrl.bat <command>
```

| Command | Effect |
|---------|--------|
| `setup-local` | Install Rust + dx if missing, `cargo fetch` backend and client |
| `backend run dev` | Infra, API, worker |
| `backend stop dev` | Stop host apps and compose |
| `frontend list` / `frontend doctor` | Client platforms; toolchain check (targets, WebView, SDK/NDK, Xcode) |
| `frontend setup <platform>` | rustup targets (and WebKitGTK on Linux) |
| `frontend run desktop\|android\|ios\|web` | `dx serve` with hot reload |
| `frontend publish <platform>\|all` | `dx bundle` into `frontend/dist/<platform>` (`--variant`, `--package-types`, `--api`) |
| `frontend clean <platform>\|all` | Remove bundles (`all` also runs `cargo clean`) |
| `test all` | Backend tests, client tests, wasm32 check |
| `test contract` | Wire contract against a running API |
| `app create <name>` | Module stub on both sides and the client route |
| `prod start` / `prod stop` | Production compose |
| `logs` | Host and production logs |
| `flatten` / `restore-flat` | Single-root git history |
| `ping`, `clone`, `env`, `start`, `stop`, `status`, `update` | SSH operations from `servers.json` |

Linux and macOS use `rust-dioxus-ctrl.sh`. Details: [`__ctrl__/README.md`](../__ctrl__/README.md).

## Seed

Demo data through the public APIs (idempotent, no raw SQL):

```bat
rust-dioxus-ctrl.bat seed all --api http://api.localhost
rust-dioxus-ctrl.bat seed users
rust-dioxus-ctrl.bat seed all --purge
```

Packs: `users`, `notes`. Auth is `FIRST_SUPERUSER` from `.env`; non-local APIs need `--allow-remote`. See [`utils/seed/README.md`](../__ctrl__/utils/seed/README.md).
