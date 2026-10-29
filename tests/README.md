# Tests

```bat
__ctrl__\rust-dioxus-ctrl.bat test all
__ctrl__\rust-dioxus-ctrl.bat test backend
__ctrl__\rust-dioxus-ctrl.bat test frontend
__ctrl__\rust-dioxus-ctrl.bat test contract
```

| Folder | Runner | What it covers |
|--------|--------|----------------|
| `backend/` | `cargo test` in `backend/` (`[[test]]` targets point here) | Auth, users, notes isolation and caching, private routes. In-memory fakes; no database. |
| `frontend/` | `cargo test --no-default-features` in `frontend/` | API base URL resolution (web vs native), `detail` error parsing. No renderer needed. |
| `contract/` | `python tests/contract/contract_test.py --base URL --local --jobs` | A running API against [CONTRACT.md](../../../../CONTRACT.md). |

`test frontend` also runs `cargo check --target wasm32-unknown-unknown --features web`.
