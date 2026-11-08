# safe/ — keys, addresses, prod env (local only)

| Pattern | Purpose |
|---------|---------|
| `*-privatekey.pem` | SSH private key |
| `*-address.txt` | VM IP / hostname (first line) |
| `*-env.env` | Production secrets → uploaded as `~/projects/rust-dioxus/.env` |

| Files | Server id |
|-------|-----------|
| `ar-rust-dioxus-bamdad-*` | `rust-dioxus` |

Copy the `*.example` stubs, drop the `.example` suffix, and fill real values.

`*.pem`, `*.env`, `*-address.txt` are gitignored.

Upload env to VM:

```bat
rust-dioxus-ctrl.bat env
```

That copies `safe/ar-rust-dioxus-bamdad-env.env` → `~/projects/rust-dioxus/.env`.
