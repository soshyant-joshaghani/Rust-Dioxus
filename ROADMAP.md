# Roadmap

Rust-Dioxus is the full-Rust launchpad of the FoxG family: one Axum API, one Dioxus client for web, desktop, and mobile. It stays aligned with the folder contract and the wire contract in [foxg-kit](../../../README.md).

- Keep `apps/sample`, `base`, and `system` in step with Rust-Svelte and Fast-Svelte, on the backend and in the client
- Keep the wire contract in [CONTRACT.md](../../../CONTRACT.md) identical, so a project can change backend without changing the client or the data
- Keep `frontend/` backend-agnostic. It is the client for the planned Fast-Dioxus, Elysia-Dioxus, and Hono-Dioxus kits
- Keep the Python `__ctrl__` command surface (dev, native, test, app, prod, flatten, remote)
- Track Dioxus releases: bump `dioxus` and `DX_VERSION` together, then run `test all` and `native build all`
- Grow the enterprise surface inside `base/` and `system/` (audit, tenancy, policy) only when a product needs it
