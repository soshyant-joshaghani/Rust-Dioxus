"""rust-dioxus-ctrl test."""

from __future__ import annotations

import argparse
import subprocess
import sys

from lib.runtimes import ensure_targets
from utils.dev.cli import _setup_local, _tool_argv, PROJECT

CTRL = "rust-dioxus-ctrl"


def _run(argv_name: str, args: list[str], cwd) -> int:
    argv = _tool_argv(argv_name, *args)
    if not argv:
        return 1
    print(f"[rust-dioxus] {' '.join(argv)}")
    return subprocess.run(argv, cwd=cwd).returncode


def _run_backend() -> int:
    print("[rust-dioxus] Backend tests (cargo test)")
    from utils.dev.cli import _rust_docker_argv, _rust_in_docker

    if _rust_in_docker():
        print("[rust-dioxus] cargo not found on PATH: running cargo test in a rust:1 container")
        return subprocess.run(_rust_docker_argv("test"), cwd=PROJECT).returncode
    return _run("cargo", ["test"], PROJECT / "backend")


def _run_frontend() -> int:
    front = PROJECT / "frontend"
    if not (front / "Cargo.toml").is_file():
        print("[rust-dioxus] No frontend crate. Skipping frontend tests.")
        return 0
    # Pure logic (tests/frontend/*.rs) runs natively without a renderer.
    print("[rust-dioxus] Frontend tests (cargo test --no-default-features)")
    code = _run("cargo", ["test", "--no-default-features"], front)
    if code != 0:
        return code
    # The svelte-check equivalent: the web client must compile for wasm32.
    print("[rust-dioxus] Frontend check (cargo check --target wasm32-unknown-unknown)")
    ensure_targets("web")
    return _run("cargo", ["check", "--target", "wasm32-unknown-unknown", "--features", "web"], front)


def _run_contract(args: argparse.Namespace) -> int:
    script = PROJECT / "tests" / "contract" / "contract_test.py"
    if not script.is_file():
        print(f"error: missing {script}", file=sys.stderr)
        return 1
    cmd = [sys.executable, str(script), "--base", args.base, "--local", "--jobs"]
    print("[contract] " + " ".join(cmd))
    return subprocess.run(cmd, cwd=PROJECT).returncode


def cmd_test(args: argparse.Namespace) -> int:
    if args.target == "contract":
        return _run_contract(args)
    code = _setup_local(force_install=False)
    if code != 0:
        return code
    target = args.target
    if target in ("backend", "all"):
        code = _run_backend()
        if code != 0:
            print("Backend tests failed.", file=sys.stderr)
            return code
    if target in ("frontend", "all"):
        code = _run_frontend()
        if code != 0:
            print("Frontend tests failed.", file=sys.stderr)
            return code
    if target == "all":
        print("\nAll tests passed.")
    return 0


def build_test_subparser(sub: argparse._SubParsersAction) -> None:
    sp = sub.add_parser("test", help="Run backend and frontend tests")
    sp.add_argument("target", nargs="?", default="all", choices=("all", "backend", "frontend", "contract"))
    sp.add_argument("--base", default="http://localhost:8000", help="API origin for `test contract`")
    sp.set_defaults(func=cmd_test)
