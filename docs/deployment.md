# Deployment

## Web and API

```bat
__ctrl__\rust-dioxus-ctrl.bat prod start
__ctrl__\rust-dioxus-ctrl.bat prod stop
```

`prod start` builds `compose.yml`: backend, worker, the web client, Postgres, Redis, Traefik, Adminer. The web client image (`frontend/Dockerfile`) runs `dx bundle --platform web --release` with `PUBLIC_API_BASE_URL=https://api.<DOMAIN>/api/v1` and serves the result with nginx on :5000. Set `DOMAIN` and the URLs in `compose.yml`, and replace the placeholder secrets in `.env`. Outside `ENVIRONMENT=local` the API refuses to start with `SECRET_KEY` or `FIRST_SUPERUSER_PASSWORD` set to `changethis`.

Remote commands use `__ctrl__/servers.json` and the example files under `__ctrl__/safe/`. Real keys stay gitignored. The API applies migrations on start, so there is no separate prestart step.

## Desktop and mobile

Native clients ship as packages, not through compose. Build them on the matching host OS, with the production API baked in:

```bat
__ctrl__\rust-dioxus-ctrl.bat native build windows --api https://api.example.com/api/v1
__ctrl__\rust-dioxus-ctrl.bat native build android --api https://api.example.com/api/v1 --package-types aab
```

```bash
__ctrl__/rust-dioxus-ctrl.sh native build macos --api https://api.example.com/api/v1
__ctrl__/rust-dioxus-ctrl.sh native build linux --api https://api.example.com/api/v1
__ctrl__/rust-dioxus-ctrl.sh native build ios --api https://api.example.com/api/v1
```

Artifacts land in `frontend/dist/<platform>`. `BACKEND_CORS_ORIGINS` only matters for the web client; native clients call the API from Rust, not from a browser origin.

Signing: Android release signing goes in `Dioxus.toml` `[bundle.android]` (jks file, key alias, passwords). Windows code signing uses `[bundle.windows]` (`certificate_thumbprint`). macOS and iOS use `--codesign` with your Apple team. Installers need the matching host: Windows packages build on Windows, macOS and iOS on a Mac, Linux packages on Linux. Android builds on any of them.
