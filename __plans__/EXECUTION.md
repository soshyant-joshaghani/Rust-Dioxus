# Execution

Rust-Dioxus is a launchpad, not a finished product.

Shared with every FoxG template: `frontend`, `backend`, `tests`, `traefik`, `docs`, `__plans__`, and a Python `__ctrl__`.

Stage loop: mark a stage `in_progress` in PROGRESS.md, implement, run `__ctrl__\rust-dioxus-ctrl.bat test backend` (and `test all` when the client changed; `native build <platform>` when platform code changed), then mark it `done`.
