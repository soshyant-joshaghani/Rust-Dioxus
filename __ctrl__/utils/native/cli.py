"""Native clients of the one Dioxus crate in frontend/: desktop (Windows, macOS, Linux), Android, iOS.

Every command wraps dx (the Dioxus CLI):
  native run <platform>     dx serve  (hot reload, launches the app / emulator / simulator)
  native build <platform>   dx bundle (installers and packages into frontend/dist/<platform>)
"""

from __future__ import annotations

import argparse
import json
import os
import platform as host_platform
import shutil
import subprocess
import sys
from pathlib import Path
from urllib.parse import urlparse

from utils.native import android_res
from lib.runtimes import PLATFORM_TARGETS, dx_version, ensure_client_toolchain, ensure_rust, refresh_path

CTRL = Path(__file__).resolve().parents[2]
PROJECT = CTRL.parent
FRONTEND = PROJECT / "frontend"
DIST = FRONTEND / "dist"
PLATFORMS_FILE = CTRL / "platforms.json"
CTRL_NAME = "rust-dioxus-ctrl"
API_PORT = 8000

# Short ids used by the other native kits (fast-native, rust-native): native run win.
ALIASES = {"win": "windows", "mac": "macos"}

LINUX_DESKTOP_PACKAGES = (
    "libwebkit2gtk-4.1-dev",
    "libgtk-3-dev",
    "libayatana-appindicator3-dev",
    "libxdo-dev",
    "libssl-dev",
    "build-essential",
    "pkg-config",
)


def echo(message: str) -> None:
    print(message, flush=True)


def host_os() -> str:
    system = host_platform.system().lower()
    if system.startswith("win"):
        return "windows"
    if system == "darwin":
        return "macos"
    return "linux"


def load_platforms() -> list[dict]:
    return json.loads(PLATFORMS_FILE.read_text(encoding="utf-8"))["platforms"]


def get_platform(platform_id: str) -> dict:
    platform_id = ALIASES.get(platform_id, platform_id)
    for item in load_platforms():
        if item["id"] == platform_id:
            return item
    known = ", ".join(item["id"] for item in load_platforms())
    raise SystemExit(f"Unknown platform {platform_id!r}. Known: {known}")


def resolve_desktop(platform: dict) -> dict:
    """`desktop` means the desktop platform of this machine."""
    return get_platform(host_os()) if platform["id"] == "desktop" else platform


def require_buildable(platform: dict) -> None:
    if platform.get("status") != "ready":
        raise SystemExit(f"{platform['id']} is {platform.get('status', 'planned')}, not ready to build")
    if host_os() not in platform.get("hosts", []):
        hosts = ", ".join(platform.get("hosts", []))
        raise SystemExit(f"{platform['label']} builds on {hosts}; this machine is {host_os()}.")


def _quote(cmd: list[str]) -> str:
    return " ".join(f'"{part}"' if " " in part else part for part in cmd)


def _argv(cmd: list[str]) -> list[str]:
    """Windows: launch *.cmd shims through cmd /c."""
    exe = shutil.which(cmd[0])
    if exe is None:
        raise SystemExit(f"'{cmd[0]}' not found on PATH. Run: {CTRL_NAME}.bat native setup <platform>")
    if sys.platform == "win32" and exe.lower().endswith((".cmd", ".bat")):
        return ["cmd", "/c", exe, *cmd[1:]]
    return [exe, *cmd[1:]]


def run(cmd: list[str], *, cwd: Path = FRONTEND, env: dict[str, str] | None = None, check: bool = True) -> int:
    echo(f"$ {_quote(cmd)}")
    code = subprocess.run(_argv(cmd), cwd=str(cwd), env=env).returncode
    if check and code != 0:
        raise SystemExit(code)
    return code


# --- Android tooling ---------------------------------------------------------------------------


def android_sdk() -> Path | None:
    for var in ("ANDROID_HOME", "ANDROID_SDK_ROOT"):
        value = os.environ.get(var)
        if value and Path(value).is_dir():
            return Path(value)
    candidates = []
    if sys.platform == "win32" and os.environ.get("LOCALAPPDATA"):
        candidates.append(Path(os.environ["LOCALAPPDATA"]) / "Android" / "Sdk")
    candidates += [Path.home() / "Library" / "Android" / "sdk", Path.home() / "Android" / "Sdk"]
    return next((c for c in candidates if c.is_dir()), None)


def android_ndk(sdk: Path | None) -> Path | None:
    for var in ("ANDROID_NDK_HOME", "NDK_HOME", "ANDROID_NDK_ROOT"):
        value = os.environ.get(var)
        if value and Path(value).is_dir():
            return Path(value)
    if sdk and (sdk / "ndk").is_dir():
        versions = sorted((p for p in (sdk / "ndk").iterdir() if p.is_dir()), key=lambda p: p.name)
        if versions:
            return versions[-1]
    return None


def java_home() -> Path | None:
    value = os.environ.get("JAVA_HOME")
    if value and Path(value).is_dir():
        return Path(value)
    candidates = [
        Path(os.environ.get("ProgramFiles", r"C:\Program Files")) / "Android" / "Android Studio" / "jbr",
        Path("/Applications/Android Studio.app/Contents/jbr/Contents/Home"),
        Path("/opt/android-studio/jbr"),
    ]
    return next((c for c in candidates if c.is_dir()), None)


def adb_path() -> Path | None:
    sdk = android_sdk()
    if sdk:
        path = sdk / "platform-tools" / ("adb.exe" if sys.platform == "win32" else "adb")
        if path.is_file():
            return path
    found = shutil.which("adb")
    return Path(found) if found else None


def android_env(env: dict[str, str]) -> dict[str, str]:
    """Fill ANDROID_HOME / ANDROID_NDK_HOME / JAVA_HOME for dx when they can be found."""
    sdk = android_sdk()
    ndk = android_ndk(sdk)
    java = java_home()
    if sdk:
        env.setdefault("ANDROID_HOME", str(sdk))
    if ndk:
        env.setdefault("ANDROID_NDK_HOME", str(ndk))
    if java:
        env.setdefault("JAVA_HOME", str(java))
    gradle_proxy_opts(env)
    android_res.apply(env, FRONTEND)
    return env


def gradle_proxy_opts(env: dict[str, str]) -> None:
    """Gradle (Java) ignores HTTP(S)_PROXY. Pass them as JVM proxy properties so the wrapper,
    AGP, and Maven downloads go through the same proxy curl and cargo already use."""
    current = env.get("GRADLE_OPTS", "")
    if "proxyHost" in current:
        return
    opts: list[str] = []
    for scheme in ("http", "https"):
        raw = env.get(f"{scheme.upper()}_PROXY") or env.get(f"{scheme}_proxy")
        if not raw:
            continue
        parsed = urlparse(raw if "://" in raw else f"http://{raw}")
        if not parsed.hostname:
            continue
        opts += [f"-D{scheme}.proxyHost={parsed.hostname}", f"-D{scheme}.proxyPort={parsed.port or 80}"]
    if not opts:
        return
    # No http.nonProxyHosts: its `|` separator breaks gradlew.bat, and Gradle only fetches remote hosts.
    env["GRADLE_OPTS"] = " ".join([current, *opts]).strip()
    echo(f"gradle proxy: {' '.join(opts)}")


def adb_devices() -> list[str]:
    """Attached device serials, physical phones before emulators."""
    adb = adb_path()
    if adb is None:
        return []
    devices = subprocess.run([str(adb), "devices"], capture_output=True, text=True, check=False).stdout or ""
    attached = [line.split()[0] for line in devices.splitlines()[1:] if line.strip().endswith("device")]
    return sorted(attached, key=lambda serial: serial.startswith("emulator-"))


def adb_reverse_api() -> None:
    """Make 127.0.0.1:8000 on the device/emulator reach the host API (the native default base URL)."""
    adb = adb_path()
    if adb is None:
        echo("warn: adb not found; the app cannot reach the host API on 127.0.0.1:8000 until you run adb reverse.")
        return
    attached = adb_devices()
    if not attached:
        echo("No Android device or emulator attached yet. dx will start one if it can; then run:")
        echo(f"  {adb} reverse tcp:{API_PORT} tcp:{API_PORT}")
        return
    for serial in attached:
        subprocess.run(
            [str(adb), "-s", serial, "reverse", f"tcp:{API_PORT}", f"tcp:{API_PORT}"],
            check=False,
        )
        echo(f"adb reverse tcp:{API_PORT} -> host :{API_PORT} on {serial}")


# --- commands ----------------------------------------------------------------------------------


def _client_env(args: argparse.Namespace, platform: dict) -> dict[str, str]:
    env = dict(os.environ)
    api = getattr(args, "api", None)
    if api:
        # Baked in at compile time (option_env!), so a release build calls this API.
        env["PUBLIC_API_BASE_URL"] = api
    if platform["id"] == "android":
        android_env(env)
    return env


def _prepare(platform: dict) -> None:
    require_buildable(platform)
    if not (FRONTEND / "Cargo.toml").is_file():
        raise SystemExit(f"missing {FRONTEND / 'Cargo.toml'}")
    if ensure_client_toolchain(platform["id"]) != 0:
        raise SystemExit(1)


def cmd_list(_: argparse.Namespace) -> int:
    here = host_os()
    print(f"{'ID':<9} {'LABEL':<19} {'HERE':<5} {'PACKAGES':<16} NOTES")
    print("-" * 100)
    for item in load_platforms():
        ok = "yes" if here in item.get("hosts", []) else "no"
        packages = ",".join(item.get("package_types", [])) or "-"
        print(f"{item['id']:<9} {item['label']:<19} {ok:<5} {packages:<16} {item.get('notes', '')}")
    print(f"\nThis machine: {here}. Run: {CTRL_NAME}.bat native run desktop")
    return 0


def _check(label: str, ok: bool, detail: str) -> bool:
    print(f"  [{'ok' if ok else '--'}] {label:<22} {detail}")
    return ok


def cmd_doctor(args: argparse.Namespace) -> int:
    refresh_path()
    print(f"Host: {host_os()}  (frontend: {FRONTEND})")
    cargo = shutil.which("cargo")
    _check("cargo", bool(cargo), cargo or "missing: native setup <platform> installs rustup")
    dx = dx_version()
    _check("dx", bool(dx), dx or "missing: native setup <platform> installs dioxus-cli")

    installed: list[str] = []
    if shutil.which("rustup"):
        installed = subprocess.run(
            ["rustup", "target", "list", "--installed"], capture_output=True, text=True, check=False
        ).stdout.split()
    for pid, targets in PLATFORM_TARGETS.items():
        missing = [t for t in targets if t not in installed]
        _check(f"targets ({pid})", not missing, "installed" if not missing else "missing " + ", ".join(missing))

    here = host_os()
    if here == "windows":
        webview = Path(os.environ.get("ProgramFiles(x86)", r"C:\Program Files (x86)")) / "Microsoft" / "EdgeWebView"
        _check("WebView2", webview.is_dir(), str(webview) if webview.is_dir() else "install the WebView2 runtime")
        _check("MSVC link.exe", bool(shutil.which("link")) or _vswhere_has_vc(), "Visual Studio Build Tools (C++)")
    elif here == "linux":
        pkg = shutil.which("pkg-config")
        has_webkit = bool(pkg) and subprocess.run(
            ["pkg-config", "--exists", "webkit2gtk-4.1"], check=False
        ).returncode == 0
        _check("webkit2gtk-4.1", has_webkit, "ok" if has_webkit else f"run: {CTRL_NAME}.sh native setup linux")
    elif here == "macos":
        xcode = shutil.which("xcrun")
        _check("Xcode (xcrun)", bool(xcode), xcode or "xcode-select --install (full Xcode for iOS)")

    sdk = android_sdk()
    ndk = android_ndk(sdk)
    java = java_home()
    adb = adb_path()
    _check("Android SDK", bool(sdk), str(sdk) if sdk else "install Android Studio or set ANDROID_HOME")
    _check("Android NDK", bool(ndk), str(ndk) if ndk else "SDK Manager > SDK Tools > NDK (Side by side)")
    _check("Java (JDK 17+)", bool(java) or bool(shutil.which("java")), str(java or shutil.which("java") or "set JAVA_HOME"))
    _check("adb", bool(adb), str(adb) if adb else "SDK platform-tools")
    _ = args
    return 0


def _vswhere_has_vc() -> bool:
    vswhere = Path(os.environ.get("ProgramFiles(x86)", r"C:\Program Files (x86)")) / "Microsoft Visual Studio" / "Installer" / "vswhere.exe"
    if not vswhere.is_file():
        return False
    out = subprocess.run(
        [str(vswhere), "-products", "*", "-requires", "Microsoft.VisualStudio.Component.VC.Tools.x86.x64", "-property", "installationPath"],
        capture_output=True,
        text=True,
        check=False,
    ).stdout
    return bool(out.strip())


def cmd_setup(args: argparse.Namespace) -> int:
    platform = resolve_desktop(get_platform(args.platform))
    if ensure_rust() != 0:
        return 1
    if ensure_client_toolchain(platform["id"]) != 0:
        return 1
    if platform["id"] == "linux" and host_os() == "linux" and shutil.which("apt-get"):
        run(["sudo", "apt-get", "update", "-y"], cwd=PROJECT)
        run(["sudo", "apt-get", "install", "-y", *LINUX_DESKTOP_PACKAGES], cwd=PROJECT)
    if platform["id"] == "android":
        sdk = android_sdk()
        if not sdk or not android_ndk(sdk):
            echo("Android SDK/NDK not found. Install Android Studio, then SDK Manager > SDK Tools > NDK (Side by side).")
    if platform["id"] == "ios" and host_os() != "macos":
        echo("iOS builds need a Mac with Xcode.")
    echo(f"{platform['label']} toolchain ready. Check with: {CTRL_NAME}.bat native doctor")
    return 0


def cmd_run(args: argparse.Namespace) -> int:
    platform = resolve_desktop(get_platform(args.platform))
    _prepare(platform)
    cmd = ["dx", "serve", "--platform", platform["dx"], "--open", "false"]
    if platform["id"] == "web":
        cmd += ["--port", "5000", "--addr", "0.0.0.0"]
    if args.release or args.variant == "release":
        cmd.append("--release")
    device = getattr(args, "device", None)
    if platform["id"] == "android":
        adb_reverse_api()
        # Without --device dx targets the emulator ABI (x86_64), which a phone rejects.
        if not device and adb_devices():
            device = adb_devices()[0]
            echo(f"android device: {device}")
    if device:
        cmd += ["--device", device]
    echo(f"[{CTRL_NAME}] {platform['label']} client with hot reload. The API must be running (dev run all).")
    return run(cmd, env=_client_env(args, platform), check=False)


def _bundle(platform: dict, args: argparse.Namespace) -> Path:
    _prepare(platform)
    out_dir = DIST / platform["id"]
    out_dir.mkdir(parents=True, exist_ok=True)
    cmd = ["dx", "bundle", "--platform", platform["dx"], "--out-dir", str(out_dir)]
    if args.variant == "release":
        cmd.append("--release")
    package_types = args.package_types.split(",") if getattr(args, "package_types", None) else []
    for package_type in package_types:
        cmd += ["--package-types", package_type.strip()]
    target = getattr(args, "target", None)
    if platform["id"] == "android":
        # dx otherwise picks the emulator ABI; real phones are arm64.
        target = target or "aarch64-linux-android"
    if target:
        cmd += ["--target", target]
    run(cmd, env=_client_env(args, platform))
    echo(f"artifacts: {out_dir}")
    for item in sorted(out_dir.iterdir()):
        echo(f"  {item.name}")
    return out_dir


def cmd_build(args: argparse.Namespace) -> int:
    if args.platform == "all":
        here = host_os()
        targets = [
            p for p in load_platforms()
            if p["id"] not in ("desktop", "web") and p.get("status") == "ready" and here in p.get("hosts", [])
        ]
    else:
        targets = [resolve_desktop(get_platform(args.platform))]
    for platform in targets:
        echo(f"\n== {platform['label']} ==")
        _bundle(platform, args)
    return 0


def cmd_clean(args: argparse.Namespace) -> int:
    if args.platform == "all":
        if DIST.is_dir():
            shutil.rmtree(DIST, ignore_errors=True)
            echo(f"removed {DIST}")
        if shutil.which("cargo"):
            run(["cargo", "clean"], check=False)
        return 0
    platform = resolve_desktop(get_platform(args.platform))
    out_dir = DIST / platform["id"]
    if out_dir.is_dir():
        shutil.rmtree(out_dir, ignore_errors=True)
        echo(f"removed {out_dir}")
    dx_dir = FRONTEND / "target" / "dx"
    if dx_dir.is_dir():
        for build in dx_dir.glob(f"*/*/{platform['dx']}"):
            shutil.rmtree(build, ignore_errors=True)
            echo(f"removed {build}")
    return 0


def build_native_subparser(sub: argparse._SubParsersAction) -> None:
    native = sub.add_parser(
        "native",
        help="Desktop, Android, and iOS clients (dx serve / dx bundle)",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=(
            "examples:\n"
            f"  {CTRL_NAME}.bat native list\n"
            f"  {CTRL_NAME}.bat native doctor\n"
            f"  {CTRL_NAME}.bat native setup android\n"
            f"  {CTRL_NAME}.bat native run win\n"
            f"  {CTRL_NAME}.bat native run android\n"
            f"  {CTRL_NAME}.bat native build win --variant release\n"
            f"  {CTRL_NAME}.bat native build android --variant release --api https://api.example.com/api/v1\n"
            f"  {CTRL_NAME}.bat native build all --variant release\n"
        ),
    )
    nested = native.add_subparsers(dest="native_command")
    ids = [item["id"] for item in load_platforms()] + list(ALIASES)

    nested.add_parser("list", help="List client platforms").set_defaults(func=cmd_list)
    nested.add_parser("doctor", help="Check Rust, dx, targets, WebView, Android SDK/NDK, Xcode").set_defaults(func=cmd_doctor)

    setup = nested.add_parser("setup", help="Install rustup targets and platform packages")
    setup.add_argument("platform", choices=ids)
    setup.set_defaults(func=cmd_setup)

    run_parser = nested.add_parser("run", help="dx serve: build, launch, and hot reload a client")
    run_parser.add_argument("platform", choices=ids)
    run_parser.add_argument("--release", action="store_true", help="release build")
    run_parser.add_argument("--variant", default="debug", choices=["debug", "release"], help="same as --release when release")
    run_parser.add_argument("--device", default=None, help="Android/iOS device or simulator name")
    run_parser.add_argument("--api", default=None, help="PUBLIC_API_BASE_URL baked into the build")
    run_parser.set_defaults(func=cmd_run)

    build = nested.add_parser("build", help="dx bundle: installers/packages into frontend/dist/<platform>")
    build.add_argument("platform", choices=[*ids, "all"])
    build.add_argument("--variant", default="release", choices=["debug", "release"])
    build.add_argument("--package-types", default=None, help="comma list, e.g. msi,nsis or apk,aab (default: dx's)")
    build.add_argument("--api", default=None, help="PUBLIC_API_BASE_URL baked into the build")
    build.add_argument("--target", default=None, help="rustc triple (android default: aarch64-linux-android)")
    build.set_defaults(func=cmd_build)

    clean = nested.add_parser("clean", help="Remove bundles (all: also cargo clean)")
    clean.add_argument("platform", choices=[*ids, "all"])
    clean.set_defaults(func=cmd_clean)

    native.set_defaults(func=lambda _: native.print_help() or 0)
