# CLI

```bat
__ctrl__\rust-dioxus-ctrl.bat <command>
```

| Command | Effect |
|---------|--------|
| `setup-local` | Install Rust + dx if missing, `cargo fetch` backend and client |
| `dev run all` | Infra, API, worker, `dx serve --platform web` on :5000 |
| `dev stop all` | Stop host apps and compose |
| `native list` / `native doctor` | Client platforms; toolchain check (targets, WebView, SDK/NDK, Xcode) |
| `native setup <platform>` | rustup targets (and WebKitGTK on Linux) |
| `native run desktop\|android\|ios\|web` | `dx serve` with hot reload |
| `native build <platform>\|all` | `dx bundle` into `frontend/dist/<platform>` (`--variant`, `--package-types`, `--api`) |
| `native clean <platform>\|all` | Remove bundles (`all` also runs `cargo clean`) |
| `test all` | Backend tests, client tests, wasm32 check |
| `test contract` | Wire contract against a running API |
| `app create <name>` | Module stub on both sides and the client route |
| `prod start` / `prod stop` | Production compose |
| `logs` | Host and production logs |
| `flatten` / `restore-flat` | Single-root git history |
| `ping`, `clone`, `env`, `start`, `stop`, `status`, `update` | SSH operations from `servers.json` |

Linux and macOS use `rust-dioxus-ctrl.sh`. Details: [`__ctrl__/README.md`](../__ctrl__/README.md).
