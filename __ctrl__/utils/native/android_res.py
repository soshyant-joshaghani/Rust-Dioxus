"""App icon and name for the Android client.

dx 0.7 writes its stock launcher icon into the generated Gradle project on every build and has no
Dioxus.toml key to replace it. So the ctrl renders our own res/ from frontend/assets/icons, and a
Gradle init script overlays it on the debug and release source sets (build-type resources override
main, so there is no duplicate-resource clash). The init script does nothing unless FOXG_ANDROID_RES
is set, which only `native run|build android` does.
"""

from __future__ import annotations

import re
from pathlib import Path
from xml.sax.saxutils import escape

from PIL import Image, ImageDraw

ENV_VAR = "FOXG_ANDROID_RES"
INIT_SCRIPT_NAME = "foxg-dioxus-res.gradle"

# Adaptive icon: 108dp canvas, the maskable PNG already keeps the logo in the 66dp safe zone.
FOREGROUND_SOURCE = "icon-512-maskable.png"
LEGACY_SOURCE = "icon-512.png"
BACKGROUND = "#09090b"  # PWA theme_color in assets/manifest.webmanifest
DENSITIES = {"mdpi": 1.0, "hdpi": 1.5, "xhdpi": 2.0, "xxhdpi": 3.0, "xxxhdpi": 4.0}

INIT_SCRIPT = f"""\
// Written by rust-dioxus-ctrl (FoxG). Inert unless {ENV_VAR} is set by `native run|build android`.
// Overlays the kit's launcher icon and app name on the Gradle project dx generates.
def foxgRes = System.getenv("{ENV_VAR}")
if (foxgRes) {{
    allprojects {{
        plugins.withId("com.android.application") {{
            ["debug", "release"].each {{ name ->
                android.sourceSets.maybeCreate(name).res.srcDir(foxgRes)
            }}
        }}
    }}
}}
"""


def _app_name(frontend: Path) -> str:
    """[application] name in Dioxus.toml (regex: the ctrl runs on Python 3.10, which has no tomllib)."""
    raw = (frontend / "Dioxus.toml").read_text(encoding="utf-8")
    section = re.search(r"^\[application\]\s*$(.*?)(?=^\[|\Z)", raw, re.M | re.S)
    name = re.search(r'^\s*name\s*=\s*"([^"]+)"', section.group(1), re.M) if section else None
    return name.group(1) if name else "App"


def _write(path: Path, data: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if not path.is_file() or path.read_text(encoding="utf-8") != data:
        path.write_text(data, encoding="utf-8")


def _legacy_icon(source: Image.Image, size: int) -> Image.Image:
    """Pre-API-26 launchers: the logo on a rounded dark square."""
    canvas = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    ImageDraw.Draw(canvas).rounded_rectangle(
        (0, 0, size - 1, size - 1), radius=size // 5, fill=BACKGROUND
    )
    logo = source.resize((int(size * 0.84), int(size * 0.84)), Image.LANCZOS)
    offset = (size - logo.width) // 2
    canvas.alpha_composite(logo, (offset, offset))
    return canvas


def render_res(frontend: Path) -> Path:
    """Write frontend/target/android-res and return it. Cheap; re-renders only when the icon changed."""
    icons = frontend / "assets" / "icons"
    out = frontend / "target" / "android-res"
    stamp = out / ".source-mtime"
    sources = [icons / FOREGROUND_SOURCE, icons / LEGACY_SOURCE, frontend / "Dioxus.toml"]
    mtime = str(max(p.stat().st_mtime_ns for p in sources))

    if not (stamp.is_file() and stamp.read_text() == mtime):
        foreground = Image.open(icons / FOREGROUND_SOURCE).convert("RGBA")
        legacy = Image.open(icons / LEGACY_SOURCE).convert("RGBA")
        for density, scale in DENSITIES.items():
            folder = out / f"mipmap-{density}"
            folder.mkdir(parents=True, exist_ok=True)
            fg = round(108 * scale)
            foreground.resize((fg, fg), Image.LANCZOS).save(folder / "foxg_launcher_foreground.png")
            _legacy_icon(legacy, round(48 * scale)).save(folder / "ic_launcher.png")
        out.mkdir(parents=True, exist_ok=True)
        stamp.write_text(mtime)

    _write(
        out / "mipmap-anydpi-v26" / "ic_launcher.xml",
        '<?xml version="1.0" encoding="utf-8"?>\n'
        '<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">\n'
        '    <background android:drawable="@color/foxg_launcher_background" />\n'
        '    <foreground android:drawable="@mipmap/foxg_launcher_foreground" />\n'
        "</adaptive-icon>\n",
    )
    _write(
        out / "values" / "foxg.xml",
        '<?xml version="1.0" encoding="utf-8"?>\n<resources>\n'
        f'    <color name="foxg_launcher_background">{BACKGROUND}</color>\n'
        f'    <string name="app_name">{escape(_app_name(frontend))}</string>\n'
        "</resources>\n",
    )
    return out


def install_init_script(env: dict[str, str]) -> Path:
    home = Path(env.get("GRADLE_USER_HOME") or Path.home() / ".gradle")
    path = home / "init.d" / INIT_SCRIPT_NAME
    _write(path, INIT_SCRIPT)
    return path


def apply(env: dict[str, str], frontend: Path) -> None:
    """Render res/, install the init script, and point Gradle at the res/ through the env."""
    res = render_res(frontend)
    install_init_script(env)
    env[ENV_VAR] = str(res)
