"""Scaffold new app modules: backend router + Dioxus client module, page, and route."""

from __future__ import annotations

import argparse
import re
from pathlib import Path

from lib.config import ROOT

PROJECT = ROOT.parent
CTRL = "rust-dioxus-ctrl"
KIND = "rust"

_NAME_RE = re.compile(r"^[a-z][a-z0-9_]*$")


def _validate_name(name: str) -> str:
    name = name.strip().lower().replace("-", "_")
    if not _NAME_RE.match(name):
        raise SystemExit(
            "Module name must start with a letter and contain only lowercase "
            "letters, digits, and underscores."
        )
    if name in {"sample", "base", "system", "global"}:
        raise SystemExit(f"Reserved module name: {name}")
    return name


def _write_if_missing(path: Path, content: str) -> bool:
    if path.exists():
        print(f"  skip (exists): {path.relative_to(PROJECT)}")
        return False
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")
    print(f"  created: {path.relative_to(PROJECT)}")
    return True


def _title(name: str) -> str:
    return name.replace("_", " ").title()


def _pascal(name: str) -> str:
    return "".join(part.capitalize() for part in name.split("_"))


def _backend_files(name: str) -> None:
    title = _title(name)
    base = PROJECT / "backend/src/modules/apps" / name
    _write_if_missing(base / "mod.rs", "pub mod router;\n")
    _write_if_missing(
        base / "router.rs",
        "use axum::{routing::get, Json, Router};\n"
        "use serde_json::{json, Value};\n\n"
        "use crate::core::state::AppState;\n\n"
        "pub fn router() -> Router<AppState> {\n"
        f'    Router::new().route("/{name}", get(root))\n'
        "}\n\n"
        "async fn root() -> Json<Value> {\n"
        f'    Json(json!({ "message": "{title} module" }))\n'
        "}\n",
    )


CLIENT_MOD = """//! {title}. Page: `routes/dashboard/{name}.rs`.

pub mod api;
"""

CLIENT_API = """use serde_json::Value;

use crate::modules::base::utils::{{api_error::ApiError, http}};

pub async fn get_root(token: &str) -> Result<Value, ApiError> {{
    http::get_json(Some(token), "{name}", "Failed to load {name}").await
}}
"""

CLIENT_PAGE = """use dioxus::prelude::*;
use serde_json::Value;

use crate::modules::apps::{name}::api::get_root;
use crate::modules::base::stores::auth::use_auth;

#[component]
pub fn {component}() -> Element {{
    let auth = use_auth();
    let data = use_resource(move || async move {{
        let token = auth.token().unwrap_or_default();
        get_root(&token).await
    }});
    let message = match &*data.read() {{
        None => "Loading…".to_string(),
        Some(Ok(body)) => body.get("message").and_then(Value::as_str).unwrap_or("ok").to_string(),
        Some(Err(err)) => err.message.clone(),
    }};

    rsx! {{
        section {{ class: "rounded-xl border p-6",
            h2 {{ class: "text-2xl font-bold", "{title}" }}
            p {{ class: "mt-4 font-mono text-sm text-muted-foreground", "GET /api/v1/{name} → {{message}}" }}
        }}
    }}
}}
"""


def _append_line(path: Path, line: str) -> None:
    text = path.read_text(encoding="utf-8")
    if line in text.splitlines():
        return
    path.write_text(text.rstrip("\n") + "\n" + line + "\n", encoding="utf-8")
    print(f"  updated: {path.relative_to(PROJECT)}")


def _register_route(name: str, component: str) -> None:
    """Add the page to the Route enum, inside the dashboard layout."""
    routes = PROJECT / "frontend/src/routes/mod.rs"
    text = routes.read_text(encoding="utf-8")
    if f"{component} {{}}," in text:
        return
    text = text.replace("use login::Login;", f"use login::Login;\nuse dashboard::{name}::{component};", 1)
    variant = f'        #[route("/{name.replace("_", "-")}")]\n        {component} {{}},\n'
    text = text.replace("    #[end_layout]", variant + "    #[end_layout]", 1)
    routes.write_text(text, encoding="utf-8")
    print(f"  updated: {routes.relative_to(PROJECT)}")


def _frontend_files(name: str) -> None:
    root = PROJECT / "frontend"
    if not (root / "Cargo.toml").is_file():
        print("  skip frontend: no frontend/Cargo.toml")
        return
    values = {"name": name, "title": _title(name), "component": _pascal(name)}
    module = root / "src/modules/apps" / name
    _write_if_missing(module / "mod.rs", CLIENT_MOD.format(**values))
    _write_if_missing(module / "api.rs", CLIENT_API.format(**values))
    _write_if_missing(root / "src/routes/dashboard" / f"{name}.rs", CLIENT_PAGE.format(**values))
    _append_line(root / "src/modules/apps/mod.rs", f"pub mod {name};")
    _append_line(root / "src/routes/dashboard/mod.rs", f"pub mod {name};")
    _register_route(name, values["component"])


def cmd_app_create(args: argparse.Namespace) -> int:
    name = _validate_name(args.name)
    print(f"[{CTRL}] Scaffolding app module: {name}")
    _backend_files(name)
    _frontend_files(name)
    print()
    print("Next steps:")
    print("  1. Copy the depth of the sample module.")
    print(f"  2. Merge {name}::router::router() in backend/src/modules/apps/mod.rs.")
    print("  3. Add a sidebar link in frontend/src/modules/base/app_sidebar.rs.")
    print(f"  4. Run: __ctrl__\\{CTRL}.bat test all")
    return 0


def build_app_subparser(sub: argparse._SubParsersAction) -> None:
    sp = sub.add_parser("app", help="Scaffold application modules")
    actions = sp.add_subparsers(dest="app_action", required=True)
    create_sp = actions.add_parser("create", help="Create a new app module skeleton")
    create_sp.add_argument("name", help="module name (e.g. bookmarks, orders)")
    create_sp.add_argument("--force", action="store_true", help="kept for the shared command surface")
    create_sp.set_defaults(func=cmd_app_create)
