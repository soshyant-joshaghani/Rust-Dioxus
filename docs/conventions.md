# Conventions

Product code goes in `apps/<name>/`. Platform code stays in `base/` and `system/`. The client mirrors this: `frontend/src/modules/apps/<name>/` and `frontend/src/modules/base/`.

JSON fields are `snake_case`. HTTP paths stay under `/api/v1`. Errors are `{"detail": "..."}`. Database tables and columns are the ones in [CONTRACT.md](../../../../CONTRACT.md).

Do not add a frontend `components/` folder. Dioxus UI lives in `frontend/src/modules/base` (primitives in `base/ui`) and `frontend/src/modules/apps`. Pages live in `frontend/src/routes`.

Client code is shared by every platform. Gate platform behaviour with `cfg(target_arch = "wasm32")` or `cfg(target_os = "...")` inside the module that needs it; do not fork pages per platform.
