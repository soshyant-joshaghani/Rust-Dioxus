# Modules

| Group | Backend | Client |
|-------|---------|--------|
| `apps/sample` | Canonical notes example | `modules/apps/sample/api.rs`, page `routes/dashboard/sample/notes.rs` |
| `base/auth`, `base/users` | Login, session, user records | `modules/base/{authentication.rs,stores/auth.rs,users/}`, page `routes/dashboard/admin.rs` |
| `system` | Health and private dev routes | dashboard home cards, dev signup |

Backend paths: `backend/src/modules/{apps,base,system}`, `backend/src/core`, and `backend/src/bin/{api,worker}.rs`.

```bat
__ctrl__\rust-dioxus-ctrl.bat app create myfeature
```

It writes the backend router stub, the client module and page, and adds the route to `frontend/src/routes/mod.rs`. Then copy the depth of `sample` before adding rules:

1. Copy `backend/src/modules/apps/sample/` to `backend/src/modules/apps/<name>/` (router, service, repository, schemas).
2. Merge the new router in `backend/src/modules/apps/mod.rs`.
3. Add `backend/migrations/NNNN_<name>.sql` when tables change.
4. Fill `frontend/src/modules/apps/<name>/api.rs` with typed calls (copy `apps/sample/api.rs`) and the page under `frontend/src/routes/dashboard/`.
5. Add a sidebar link in `frontend/src/modules/base/app_sidebar.rs`.
6. Add tests in `tests/backend/` (and `tests/frontend/` for client logic).
