# Frontend (Dioxus)

One Dioxus 0.7 crate for every client. It is HTTP-only against the [FoxG wire contract](../../../../CONTRACT.md), so it runs against any FoxG backend.

```text
frontend/
├── Cargo.toml          features: web | desktop | mobile
├── Dioxus.toml         dx config: app name, /api proxy, bundle identifier, icons
├── tailwind.css        Tailwind v4 input + tokens (dx writes assets/tailwind.css)
├── assets/             icons, favicon, PWA manifest + service worker, generated tailwind.css
├── Dockerfile          production web image (dx bundle → nginx :5000)
└── src/
    ├── main.rs         launch (desktop window config)
    ├── app.rs          root: stylesheet, stores, theme, router
    ├── config/         API base URL (PUBLIC_API_BASE_URL)
    ├── modules/
    │   ├── base/       auth, users, shell (sidebar, header), stores, utils/http, ui primitives
    │   └── apps/       product domains (sample = notes)
    └── routes/         /login, and inside the dashboard layout: /, /sample/notes, /admin
```

## Run

From the kit root:

| Command | Effect |
|---------|--------|
| `__ctrl__\rust-dioxus-ctrl.bat dev run all` | API + worker + `dx serve --platform web` on :5000 (http://dashboard.localhost) |
| `__ctrl__\rust-dioxus-ctrl.bat native run desktop` | Native window for this OS, hot reload |
| `__ctrl__\rust-dioxus-ctrl.bat native run android` | Emulator or device (adb reverse to the host API) |
| `__ctrl__\rust-dioxus-ctrl.bat native run ios` | iOS simulator (macOS) |
| `__ctrl__\rust-dioxus-ctrl.bat native build <platform>` | Packages into `frontend/dist/<platform>` |

Plain `dx` works too, from this folder: `dx serve --platform web`, `dx serve --platform desktop`, `dx bundle --platform android --release`.

## API URL

| Client | Default | Override |
|--------|---------|----------|
| Web | `/api/v1` on its own origin (dx proxies to :8000 in dev) | `PUBLIC_API_BASE_URL` at build time |
| Desktop, iOS | `http://127.0.0.1:8000/api/v1` | `PUBLIC_API_BASE_URL` at build time or run time |
| Android | `http://127.0.0.1:8000/api/v1` through `adb reverse tcp:8000 tcp:8000` | same, or `--api` on `native build` |

HTTP goes through reqwest (browser fetch on wasm, hyper + rustls on native), so mobile cleartext rules for the WebView do not block local development.

## Storage

`modules/base/stores/storage.rs` persists the session (`authToken`, `currentUser`) and `theme`: `localStorage` on web, a JSON file in the app data folder on desktop and iOS, and the app's private `files/` folder on Android.

## Where UI code goes

- Pages: `src/routes/`, registered in the `Route` enum (`src/routes/mod.rs`).
- Feature API clients: `src/modules/apps/<name>/api.rs`, using `modules/base/utils/http.rs`.
- Shell and primitives: `src/modules/base/` and `src/modules/base/ui/`.

There is no `components/` folder. Style with Tailwind utilities; tokens live in `tailwind.css`.
