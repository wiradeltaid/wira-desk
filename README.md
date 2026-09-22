# Wira Desk

> Lightweight, native same-app window cycling, zone snapping, and driverless mouse navigation for Windows 11 - written in Rust 🦀

[English](README.md) | [Bahasa Indonesia](README.id.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [Português (Brasil)](README.pt-BR.md) | [Русский](README.ru.md)  
[Website](https://wiradelta.id/wira-desk) | [Download](https://github.com/wiradeltaid/wira-desk/releases) | [Changelog](CHANGELOG.md) | [Contributing](CONTRIBUTING.md) | [License](LICENSE) | [Security](SECURITY.md) | [Privacy](PRIVACY.md)

---

> **If you run PowerToys only for FancyZones, and Logi Options+ only for the thumb buttons, this replaces both - one tray process instead of two.**
>
> What this does not replace: PowerRename, Awake, Color Picker, or custom drawn FancyZones layouts; Logitech Flow, per-app profiles, battery, or DPI switching.

## Installation

### Via Scoop (Recommended)

```powershell
scoop bucket add wiradesk https://github.com/wiradeltaid/scoop-wiradesk
scoop install wiradesk
```

### Setup Executable

Download the installer (`WiraDesk-*-x64-setup.exe`) from the [releases page](https://github.com/wiradeltaid/wira-desk/releases) (mirrored on [SourceForge](https://sourceforge.net/projects/wira-desk/files/latest/download)) and verify SHA-256:

```powershell
Get-FileHash .\WiraDesk-*-x64-setup.exe -Algorithm SHA256
```

Installs elevated to `%ProgramFiles%\Wira Desk`. Auto-start is opt-in and configurable from Settings or the tray icon.

### Loose Binaries (Portable)

Download loose `wiradesk.exe` and `wiradesk-settings.exe` into an administrator-only directory and run `wiradesk.exe` as Administrator.

---

## Features

- **Same-App Window Cycling:** ``Win + ` `` cycles only windows of the active app on the current monitor and virtual desktop (fallback: ``Alt + ` ``). Tap to cycle instantly, or hold 300 ms for the visual switcher overlay with live thumbnails.
- **One-Key Zone Snapping:** Instant window snapping to halves (50%), thirds (33%), or directional custom percentages (default 67%, top edge default 33%) without opening a zone editor.
- **Driverless Mouse Navigation:** Maps thumb buttons (`XBUTTON1`/`XBUTTON2`) and horizontal tilt wheel to virtual desktop switching or 20 customizable presets without background vendor utilities.

### Default Shortcuts

| Shortcut | Action |
|---|---|
| ``Win + ` `` | Cycle windows of current app (hold 300 ms for visual switcher overlay) |
| ``Alt + ` `` | Fallback cycling shortcut |
| `Ctrl+Alt+Left/Right/Up/Down` | Snap active window to that half (50%) |
| `Ctrl+Alt+Shift+Left/Right/Up/Down` | Snap window to that edge at custom percentage (left/right/bottom default 67%, top default 33%) |
| `Ctrl+Alt+1/2/3` | Snap window to left, middle, or right third |
| `Ctrl+Alt+Enter` | Maximize window |
| `Ctrl+Alt+Shift+Enter` | Move window to next monitor |
| `Ctrl+Alt+Shift+S` | Stack 3 windows at configurable width |

### Mouse Presets

Thumb buttons default to previous/next virtual desktop; tilt wheel defaults to Show Desktop / Task View. Each is remappable to any of 20 presets in Settings. Cursor coordinates are never read; see [`PRIVACY.md`](PRIVACY.md).

---

## Why

While PowerToys includes Window Hopper and vendor tools manage mouse buttons, they consume 150-500 MB of RAM across multiple background processes. Wira Desk runs as a single native background daemon consuming approximately 4.0 MB private memory (under 5 MB budget), with zero telemetry.

---

## Configuration & Development

- **Configuration:** Settings live under `%APPDATA%\WiraDesk\config.toml`. See [docs/CONFIGURATION.md](docs/CONFIGURATION.md) for the complete TOML reference.
- **Development:** Built with Rust and MSVC. See [DEVELOPMENT.md](DEVELOPMENT.md) for build, testing, and unsafe code guidelines.
- **Contributing:** Contributions are welcome - see [CONTRIBUTING.md](CONTRIBUTING.md).

---

## About & Legal

**Wira Delta Indonesia** is the studio behind this project. Built and maintained by [@kodesh87](https://github.com/kodesh87).

- **License:** [GPL-3.0-only](LICENSE). Third-party acknowledgements are listed in [NOTICE](NOTICE). Built with [Slint](https://slint.dev).
- **Privacy & Security:** Zero telemetry, no account, no background updater service. See [PRIVACY.md](PRIVACY.md) and [SECURITY.md](SECURITY.md).
- **The Name and the Icon:** The GPL grants rights over code, not names or logos. The names **Wira Desk** and **Wira Delta Indonesia**, and the product icon, remain property of Wira Delta Indonesia.
