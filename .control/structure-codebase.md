---
type: structure
scope: codebase
verified: '2026-11-16'
commit: '8195140'
---

# Codebase Structure

## Verified

2026-11-16, read from the tracked tree (`git ls-files`) at commit `8195140`, so everything
`.gitignore` excludes (`target/`, `stage/`, `dist/`, `nsis/`, `out/`, `.what-rendered/`,
`.how-rendered/`) is left out.

## Top level

```text
wira-desk/
├── crates/                    # [container]/[lib] the Cargo workspace: daemon, settings, shared
├── assets/                    # [docs] logo, icon, favicon, and installer wizard source images
├── docs/                      # [docs] configuration reference, threat model, packaging and distribution notes, research
├── packaging/                 # [tooling] Inno Setup script, installer language file, Scoop bucket and WinGet manifests
├── scripts/                   # [tooling] PowerShell build, release, and runtime verification scripts
├── .github/                   # [tooling] CI workflows (ci.yml, release.yml), dependabot, issue and PR templates
├── .githooks/                 # [tooling] pre-push hook, enabled with core.hooksPath
├── .cargo/                    # [tooling] Cargo build configuration (config.toml)
├── _bmad/                     # [tooling] BMad installation and team overrides
├── _bmad-output/              # [generated] BMad and WDI skill run workspace
├── .claude/                   # [tooling] Claude Code skills and settings
├── .agents/                   # [tooling] agent instructions for non-Claude CLIs (AGENTS.md)
├── .constitution/             # [docs] WDI Method rules and product rules, see structure-document.md
├── .control/                  # [docs] registries, decisions, questions, structure maps, memlog, see structure-document.md
├── .what/                     # [docs] promises corpus, see structure-document.md
├── .how/                      # [docs] mechanism corpus, see structure-document.md
├── .scratch/                  # [docs] active spec workspaces and tickets, see structure-document.md
├── .archive/                  # [docs] closed spec workspaces, see structure-document.md
└── .work/                     # [docs] committed working area for skill runs and briefs
```

## Containers

### daemon

`wiradesk.exe`, the elevated background tray daemon. Ships as a single binary with an embedded
manifest requesting `requireAdministrator`.

```text
crates/daemon/
├── src/
│   ├── main.rs                # ★ ENTRY + composition root: elevation check, thread spawn, message loop
│   ├── hook.rs                # WH_KEYBOARD_LL / WH_MOUSE_LL hook thread, throttle, chord matching
│   ├── ring.rs                # ★ lock-free hook→worker command channel
│   ├── worker.rs              # ★ command drain: composes cycling, context, arrangement, switcher
│   ├── arrangement/           # convention: one file per placement concern
│   │   ├── mod.rs             # planning API; models and planners re-exported from wiradesk-core
│   │   ├── snap.rs, thirds.rs, stack.rs, monitor.rs   # facades over wiradesk_core::arrangement
│   │   └── win32.rs           # ★ SetWindowPos application, border clamp
│   ├── cycling/               # convention: one file per stage of same-app cycling
│   │   ├── mod.rs             # cycling contract; vocabulary re-exported from wiradesk-core
│   │   ├── source.rs          # live Z-order and executable-identity discovery
│   │   ├── eligibility.rs, selection.rs   # facades over wiradesk_core::cycling
│   │   └── activation.rs      # bounded window activation
│   ├── context/               # convention: spatial, desktop, and VM/RDP eligibility gates
│   │   ├── mod.rs             # context-safety contract; policy re-exported from wiradesk-core
│   │   └── spatial.rs, virtual_desktop.rs, vm_bypass.rs
│   ├── switcher/              # same-app visual switcher overlay
│   │   ├── mod.rs             # switcher controller
│   │   ├── layout.rs, selection.rs   # facades over wiradesk_core::switcher
│   │   ├── overlay.rs         # GDI + DWM overlay window
│   │   └── thumbnail.rs       # DWM thumbnail handles
│   ├── tray.rs                # tray icon, hidden host window, main message loop
│   ├── menu.rs                # tray context menu, Settings launch (ShellExecute)
│   ├── icon.rs                # runtime HICON rasterization from icon_data.rs
│   ├── icon_data.rs           # [generated, see below] tray icon geometry
│   ├── health.rs              # hook heartbeat timer
│   ├── autostart.rs           # Task Scheduler ONLOGON registration
│   ├── acl.rs                 # install-location DACL check
│   ├── updatecheck.rs         # daily update check
│   ├── config.rs              # reload on WM_APP_RELOAD_CONFIG
│   ├── log.rs                 # Tier 2 warning log
│   ├── metrics.rs             # convergence measurement seam, debug builds only
│   ├── error.rs               # Tier 1 fatal MessageBox
│   └── util.rs
├── wiradesk.rc, wiradesk.ico  # resource script and icon
├── wiradesk.manifest          # requireAdministrator
└── build.rs                   # ★ embeds wiradesk.rc; WIRADESK_SKIP_MANIFEST test seam
```

**Flow:** `hook.rs` (hook callback) → `ring.rs` → `worker.rs` → `cycling/`, `context/`,
`arrangement/`, `switcher/` (planning, pure parts from `wiradesk-core`) → Win32 APIs (application).

### settings

`wiradesk-settings.exe`, the on-demand configuration GUI. Launched by the daemon via
`ShellExecute`; carries no elevation manifest of its own.

```text
crates/settings/
├── src/
│   ├── main.rs                # ★ ENTRY + composition root: builds MainWindow, wires every callback
│   ├── app.rs                 # ★ settings model, shortcut capture, first-run tutorial state
│   ├── persistence.rs         # validation, atomic config save, daemon reload signal
│   ├── daemon_watch.rs        # binds Settings' lifetime to the daemon's
│   ├── hookbridge.rs          # hidden receiver window for WM_APP_RECORDED_CHORD
│   ├── theme.rs               # OS theme and typography
│   ├── update.rs              # update download, verify, launch
│   ├── sha256.rs              # installer checksum over CNG
│   ├── urls.rs                # embedded wiradelta.com URLs
│   └── *_slint_snapshot.rs    # convention: Slint snapshot and interaction tests, one per UI surface
├── ui/                        # Slint markup, compiled by slint-build
│   ├── main_window.slint      # ★ root window, frameless shell, pane routing
│   ├── theme.slint            # ★ Palette global, dark/light tokens
│   ├── onboarding.slint       # first-run tutorial
│   ├── components/            # convention: shared widgets, one per file
│   └── panes/                 # convention: one file per Settings tab (<tab>_pane.slint)
├── wiradesk-settings.rc, wiradesk-settings.ico
└── build.rs                   # ★ embeds the .rc, compiles ui/main_window.slint via slint-build
```

**Flow:** `ui/*.slint` (callbacks) → `main.rs` (wiring) → `app.rs` (model) → `persistence.rs`
(validate, atomic save, daemon signal).

### core

No code of this container is in this repository. `wiradesk-core` lives in
`wiradeltaid/wira-desk-core` and is pinned by tag in `crates/daemon/Cargo.toml` and
`crates/shared/Cargo.toml`. The heading stays only because `components.yaml` still registers `core`
as `built: true` without `repo:`; see the drift reported with this refresh.

## Libraries

### shared

Config schema, IPC constants, shortcut parsing, and Windows utilities common to both binaries.

```text
crates/shared/
└── src/
    ├── lib.rs                 # ★ public surface
    ├── config.rs              # config.toml schema, defaults, atomic save
    ├── constants.rs           # ★ WM_APP_* message ids, window class/title strings, paths
    ├── commands.rs            # u8 command enum; facade over wiradesk_core::commands
    ├── shortcut.rs            # chord parsing, canonicalization, reserved chords
    ├── https.rs               # WinHTTP GET with a size ceiling, used by the updater
    ├── update.rs              # update endpoint wiring; decision re-exported from wiradesk-core
    └── binary.rs              # PE import inspection
```

**Consumed by:** `daemon`, `settings`

## Generated

| Path | Generated by |
| --- | --- |
| `crates/daemon/src/icon_data.rs` | hand-run image-processing pass over `assets/*` at logo-update time, not a build step |
| `.control/generated/` | `.constitution/method/scripts/validate.py --generate` |

---

★ = key file: entry point, wiring root, the single place a rule is enforced, or a file that must be
opened before behaviour in its folder can be changed.
