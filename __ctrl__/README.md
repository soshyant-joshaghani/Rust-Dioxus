# rust-dioxus `__ctrl__`

The **control layer** for Rust-Dioxus — one predictable CLI for the project lifecycle.

`__ctrl__` is not a loose collection of scripts. It is the official interface for:

- Starting and stopping dev infrastructure and apps
- Selecting runtime profiles (Full / Slim)
- Scaffolding app modules
- Running tests
- Deploying to production via SSH

```text
Rust-Dioxus
│
├── Application Layer     (backend + frontend)
├── Infrastructure Layer  (db, redis, workers, proxy)
└── Control Layer         __ctrl__/  ← you are here
```

Prefer `__ctrl__` commands over ad-hoc `docker compose` or manual process management unless you have a specific reason.

This is **not** foxg-ctrl. FoxG platform VMs stay under `foxg-ctrl`; Rust-Dioxus kit ops live here.

## Quick start (Windows)

From `rust-dioxus/__ctrl__/`:

```bat
rust-dioxus-ctrl.bat
```

Interactive prompt, or one-shot:

```bat
rust-dioxus-ctrl.bat setup-local
rust-dioxus-ctrl.bat dev run all
rust-dioxus-ctrl.bat test all
rust-dioxus-ctrl.bat list
rust-dioxus-ctrl.bat connect
```

Linux/mac:

```bash
chmod +x rust-dioxus-ctrl.sh
./rust-dioxus-ctrl.sh status
```

## Command map

| Area | Commands |
|------|----------|
| Local tooling | `setup-local [--force]` |
| Dev stack | `dev run\|stop\|down\|purge\|reset {infra,apps,all}` · `--slim` for lightweight runtime |
| Clients | `native list` · `native doctor` · `native setup <platform>` · `native run <platform>` · `native build <platform>\|all` · `native clean <platform>\|all` |
| App scaffold | `app create <name>` |
| Tests | `test {all,backend,frontend,contract}` |
| Local prod smoke | `prod start\|stop\|reset\|backup-acme\|…` |
| SSH / VM | `setup`, `pubkey`, `clone`, `env`, `start`, `stop`, `update`, `reset`, `backup-acme`, `connect`, … |

On-VM bash/bat scripts (what SSH `start`/`stop` invoke) live in [`remote/`](remote/README.md).

## Layout

| Path | Role |
|------|------|
| `platforms.json` | Client platforms: dx flag, host OSes, package types |
| `servers.json` | Single VM entry (`rust-dioxus`) |
| `safe/` | PEM, address, prod `.env` |
| `static/gpg` | Docker Ubuntu GPG (Iran bootstrap) |
| `remote/` | On-VM / local-prod compose scripts |
| `rust-dioxus-ctrl.bat` / `.sh` | CLI entry |

## Typical first deploy (SSH)

```bat
rust-dioxus-ctrl.bat setup
rust-dioxus-ctrl.bat pubkey
REM add VM pubkey to GitHub
rust-dioxus-ctrl.bat clone
rust-dioxus-ctrl.bat env
rust-dioxus-ctrl.bat start
```

Day-2:

```bat
rust-dioxus-ctrl.bat update
rust-dioxus-ctrl.bat status
rust-dioxus-ctrl.bat backup-acme
```

## Local dev (Docker Desktop / host apps)

```bat
rust-dioxus-ctrl.bat setup-local
rust-dioxus-ctrl.bat dev run all
rust-dioxus-ctrl.bat dev stop all
rust-dioxus-ctrl.bat dev down all
rust-dioxus-ctrl.bat dev purge infra
rust-dioxus-ctrl.bat dev reset all
```

| Action | Infra (compose.dev.yml) | Apps (host) |
|--------|-------------------------|-------------|
| `run` / `start` | `up -d` db, redis (full), proxy, adminer | Rust (Axum) API :8000 (runs SQL migrations), Redis queue worker (full), `dx serve --platform web` :5000 |
| `stop` | `compose stop` — containers kept | kill host processes |
| `down` | `compose down` — volumes kept | kill host processes |
| `purge` | `compose down -v` — wipe data, stay down | kill host processes |
| `reset` | wipe then `run` | stop then run |

| Target | Notes |
|--------|-------|
| `infra` | Docker only. SQL migrations run when the API starts |
| `apps` | host processes (needs infra already up) |
| `all` | run: infra→apps · stop/down/purge/reset: apps→infra |

Opens browser tabs for Adminer / Traefik / dashboard / API docs after a successful run.

**Runtime profiles:** `dev run all` (full — includes Redis + worker) · `dev run all --slim` (no Redis/worker). See [docs/runtime-profiles.md](../docs/runtime-profiles.md).

## Native clients

`frontend/` is one Dioxus crate. `native` wraps `dx` for every platform in [`platforms.json`](platforms.json).

```bat
rust-dioxus-ctrl.bat native list
rust-dioxus-ctrl.bat native doctor
rust-dioxus-ctrl.bat native setup android
rust-dioxus-ctrl.bat native run desktop
rust-dioxus-ctrl.bat native run android
rust-dioxus-ctrl.bat native build windows
rust-dioxus-ctrl.bat native build android --package-types apk,aab --api https://api.example.com/api/v1
rust-dioxus-ctrl.bat native build all
```

| Command | Runs |
|---------|------|
| `native run <platform>` | `dx serve --platform <platform>`: build, launch, hot reload. Android first runs `adb reverse tcp:8000 tcp:8000` |
| `native build <platform>` | `dx bundle --platform <platform> --release --out-dir frontend/dist/<platform>` (`--variant debug` drops `--release`) |
| `native build all` | Every platform this host can build (Windows: windows + android; macOS: macos + ios + android; Linux: linux + android) |
| `native setup <platform>` | Rust + dx + rustup targets; Linux also installs WebKitGTK via apt |
| `native doctor` | cargo, dx, rustup targets, WebView2 / WebKitGTK / Xcode, Android SDK, NDK, JDK, adb |

`desktop` is an alias for the desktop platform of the current machine. `--api URL` sets `PUBLIC_API_BASE_URL` for the build (compiled in). `dx` itself is pinned to `DX_VERSION` in `lib/runtimes.py` and installed with `cargo binstall`.

## Tests

```bat
rust-dioxus-ctrl.bat test all
rust-dioxus-ctrl.bat test backend
rust-dioxus-ctrl.bat test frontend
rust-dioxus-ctrl.bat test contract
```

Backend tests use in-memory fakes and need neither Postgres nor Redis. `test frontend` runs `cargo test --no-default-features` on `tests/frontend` and `cargo check --target wasm32-unknown-unknown`. `test contract` checks a running API against the wire contract.

## Local production smoke

```bat
rust-dioxus-ctrl.bat prod start
rust-dioxus-ctrl.bat prod stop
rust-dioxus-ctrl.bat prod reset
rust-dioxus-ctrl.bat prod backup-acme
```

Same scripts SSH uses under `remote/`. Prefer SSH `start`/`stop` when operating the real VM from your laptop.

## Setup (ctrl tool itself)

```bat
python -m venv .venv
.venv\Scripts\pip install -r requirements.txt
```

On first `setup-local` / `dev run all`, the ctrl entry installs system **Python 3.10+** (via winget / Homebrew / apt) if missing. `_setup_local` then installs **Rust** (rustup: winget `Rustlang.Rustup` on Windows, `sh.rustup.rs` elsewhere), **dx** (`cargo binstall dioxus-cli`), and the `wasm32-unknown-unknown` target, and runs `cargo fetch` for `backend/` and `frontend/`. No Node.js is involved. When Rust cannot be installed but Docker is available, the backend still runs in a `rust:1` container. The client always needs host Rust.

Iran VMs (`iran_setup: true`) keep provider DNS, rewrite apt to Arvan `apt_mirror`, and use Arvan Docker `registry_mirror`. `clone` routes GitHub SSH via `ssh.github.com:443`.
