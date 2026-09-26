# Changelog

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versioning is
[semantic](https://semver.org/). Below `1.0` the **minor** digit carries the breaking change -
`0.1.x` to `0.2.0` is the incompatible step.

Two things depend on the shape of this file, so the headings are not free-form:

- `release.yml` extracts the `## [x.y.z]` section matching the git tag and publishes it as the
  release notes. A tag with no matching section **fails the release** rather than shipping
  notes nobody wrote.
- The in-app updater shows the section for the version it is offering, so this is what a user
  reads before deciding to update. Write it for them, not for the commit log.

Who may move which digit is a rule, not a convention - see `AGENTS.md`, "Versioning authority".
Work that needs a minor or major bump belongs under **Unreleased** and stays there until the
owner decides.

## [Unreleased]

### Changed

- Settings About Pane Layout Polish: Natural word wrapping without awkward mid-sentence breaks for update toggle disclosure, non-breaking space for Windows 11, three-line license block sequence, and removal of duplicate GitHub button in Card 3 while preserving the Source code text link in Card 1.
- Update Check Error Message Differentiation: Update check descriptor query failures explicitly cite `wiradelta.com answered with status {code}. You can try again.`, reserving download server error terminology strictly for installer binary downloads.
- Canonical Domain Move to `wiradelta.com`: Update descriptor endpoint, studio links, browser allowlist, About pane text, and support address all move to `wiradelta.com`; `wiradelta.id` is no longer accepted by the client-side browser allowlist (ODR-011).

## [0.2.5] - 2026-09-24

### Added

- Directional Custom-Percentage Top Snap: Configurable top-edge snapping default set to 33% (`Ctrl+Alt+Shift+Up`) while lateral and bottom edges remain at 67%, with full backward compatibility and persistent configuration.
- Canonical Update Check Subsystem: Update descriptor checks migrate to the self-hosted endpoint `https://wiradelta.id/api/v1/update/wira-desk/` enforcing strict no-redirect policy (`Redirects::Never`), sharing identical 4-part User-Agent contract (`WiraDesk/<version> (Windows <major>.<minor>.<build>; <arch>)`) with installer downloads.
- Portable Zip Distribution: Release workflows package executables, license, and notices into `WiraDesk-<version>-x64-portable.zip`, eliminating loose executable uploads, protected by an automated release artifact verifier in CI.
- Settings Embedded URL Registry: Centralized registry for all studio links with trailing slashes (`/`), with hardened browser opening allowlist.
- Formal Indonesian Legal Copies: Synchronized `PRIVACY.id.md` and `SECURITY.id.md` with official copy stamps and dual-language alignment.

### Changed

- Settings About and General Pane Copy: Word-for-word alignment with approved studio legal specifications, renaming "Support development" to "Send a tip", standardizing hold delay notation to `(100 to 500 ms)`, and locking all critical legal labels.
- Threat Model Alignment: Documented dual outbound network paths, visual switcher window title inspection, and 1 MB log rotation.

## [0.2.4] - 2026-09-15

### Added

- Visual Window Switcher: Same-app visual switcher overlay displaying live window tiles and thumbnails with adaptive layout, keyboard navigation (including Shift-held backward cycling), and mouse selection.
- Driverless Mouse Desktop Navigation: Native mouse hook support mapping thumb back/forward buttons and horizontal tilt wheel left/right to desktop switching, snapping, and arrangement presets without manufacturer drivers.
- Header Defaults Buttons: Pane-level `↺ Defaults` buttons in Shortcuts, General, and Mouse headers with conditional visibility appearing only when active draft differs from defaults.
- Real-time Tray Warning Reset: Automatic clearance of the system tray warning indicator upon successful configuration reload, while maintaining critical hook error precedence.

### Changed

- Edge-snap default percentage updated from 50% to 67% (`Ctrl+Alt+Shift+Left/Right/Up/Down`).
- General Defaults isolation: Restoring General settings to defaults now preserves the Windows auto-start preference.
- Settings window & modal polish: Compact 560px window height, polished About pane card layout cleanly enclosing the Reset button, and redesigned modal confirmation dialog matching Onboarding aesthetics without blue focus outlines.
- Installer safety & downgrade prevention: Inno Setup installer enforces 64-bit HKLM registry checks, strict SemVer downgrade rejection, zero-bundling invariant, and fail-closed process shutdown.

### Fixed

- Window candidate filtering: Sanitized switcher candidates to exclude WinUI 3 popup bridges, PopupHost helpers, and invisible assistant windows.
- Start menu suppression during cycling and multi-window blind backward traversal stability.
- Clean retirement of legacy migration shims.

## [0.2.0] - 2026-09-08

### Added

- Custom-percentage edge snap: snap the active window to a screen edge at a percentage you set
  per direction in Settings, instead of the fixed half (`Ctrl+Alt+Shift+Left/Right/Up/Down`).
- Snap to thirds: snap the active window to the left, middle, or right third of the screen
  (`Ctrl+Alt+1/2/3`).
- Per-action shortcut on/off: every remappable shortcut — not just Overlapping Stack, previously
  the only one with a switch of its own — now has its own enable/disable control in Settings.
  Turning one off drops its chord from the keyboard hook entirely, so the key combination reaches
  whatever app is focused unchanged, exactly as if Wira Desk were not installed; the stored chord
  itself is untouched, and turning the action back on needs no re-entry.

### Changed

- The Shortcuts pane is grouped by what an action snaps *to*: **Switching**, **Snap to half**,
  **Snap to third**, **Snap to custom**, and **Resize, move & arrange**. The four custom-percentage
  rows now sit beside the halves and thirds they resemble instead of under moving windows around,
  and a row no longer repeats its group's name — under *Snap to custom* a row reads "Snap to left
  edge". See `DEC-014`.
- The **Layout** pane is gone, and Settings now has four panes: General, Shortcuts, VM &
  Exceptions, About. Its one remaining control — the overlapping-stack width percentage — moves
  onto the **Overlapping Stack** row in the Shortcuts pane, beside that action's own shortcut and
  its on/off switch, so the setting sits with the action it belongs to. Nothing stored changes and
  no value is migrated; the width you had is the width you keep. See `DEC-014`.
- A shortcut row shows its description **on hover or keyboard focus** instead of as a permanently
  visible line. Several descriptions were longer than the space they had, so an ellipsis was the
  only part of them some rows ever showed; the full text is now readable, and reachable without a
  mouse. The row's controls — percentage stepper, chord keycap, and on/off switch — sit on one
  vertical centre, and the five groups fit the default window width without sideways scrolling.
- If you have bound two actions to the same chord, the one that keeps it may have changed.
  Maximize now sits behind every snap variant in the order that resolves a collision, so a snap
  action wins where Maximize used to. Nothing is migrated and no chord is renamed; on the shipped
  defaults nothing collides, so most installs see no difference. See `DEC-014` for why the order
  moved and `DEC-009` for how a collision is reported.
- Overlapping Stack's default shortcut moved from `Ctrl+Alt+Shift+Down` to `Ctrl+Alt+Shift+S`,
  freeing the arrow keys under `Ctrl+Alt+Shift` for the custom-percentage snap above. See
  `DEC-011`. An install that already customized (or kept) the old default is not migrated; see
  the decision for what happens on that chord at startup.

## [0.1.5] - 2026-09-03

### Changed

- License changed from MIT to GPL-3.0-only, across the project and all three workspace crates.
  Slint (the Settings UI toolkit) is now consumed under its own GPL-3.0-only branch rather than
  the Slint Royalty-free License 2.0, so the mandatory royalty-free-tier disclosure no longer
  applies. See `LICENSE`, `NOTICE`, and `README.md`.

### Fixed

- The Key Check pane in Settings now tells a chord swallowed by another program's keyboard hook
  apart from the daemon actually being stopped. Previously both showed "Daemon Not Running,"
  which pointed you at restarting a daemon that was never the problem.

## [0.1.4] - 2026-08-29

No functional change from 0.1.3. This version exists to be found: an installed 0.1.3 checking
`releases/latest/download/latest.json` for the first time against a real newer tag, with no
debug seam or stand-in descriptor involved anywhere -- the last half of gate 2 this repository
had not yet proven for real. See 3p.md for what it verified.

## [0.1.3] - 2026-08-29

No functional change from 0.1.2. This version exists to run `release.yml` for the first time
ever — the tag-vs-crate check, the Inno Setup build inside CI, and the real
`releases/latest/download/latest.json` this repository's own updater will check against, none
of which had executed even once before this tag. A deliberate, disposable step before any
version is promoted as the one people should install; see 3p.md for what it verified.

## [0.1.2] - 2026-08-28

No change from 0.1.1, which was itself never published. This version exists to carry a
rebuild — one that verifies upgrade-in-place from an installed 0.1.1, and one that tests
whether a freshly built unsigned binary is refused by Smart App Control purely for being new.

**Three unreleased sections now stand below this one, and that is two too many.** At release
time they collapse into a single section for whichever version is tagged first: a published
changelog should not advertise versions nobody could ever install. That collapse is the
owner's call, because which version becomes the first public release is a release decision,
not a patch.

## [0.1.1] - 2026-08-28

0.1.0 was never published, so nothing here is a regression from a shipped version. What it
fixes is a defect that only appears when Settings is opened from Program Files rather than
from the tray icon.

### Fixed

- **Settings opened outside the tray could not reach Wira Desk, and said nothing about it.**
  The background process runs elevated; Settings inherits that when the tray launches it and
  runs at ordinary privilege when you start it yourself. Windows discards messages sent from
  the second to the first, so shortcut recording missed keys and a saved change never
  reached the running program — under a status line that read like an ordinary success. Both
  messages are now admitted explicitly, a refused message is retried instead of being
  remembered as sent, and the status line distinguishes "Wira Desk is not running" from
  "Wira Desk refused it", because those need different actions from you.
- Uninstalling removed every installed file but left an empty `Wira Desk` folder behind in
  Program Files. The folder is now removed too, and only when it is genuinely empty.
- `scripts/bump-version.ps1` read the manifest as the system codepage and wrote it back with a
  byte-order mark, corrupting the prose above the version it was there to change. It also
  redirected `cargo`'s stderr, which under Windows PowerShell 5.1 ends the script on the first
  progress line — after the manifest was written and before `Cargo.lock` was, leaving a lock
  that fails CI's `--locked` build. The refreshed lock is now verified against the file rather
  than assumed from an exit code.

## [0.1.0] - 2026-08-27

Initial public release of Wira Desk.

### Added

- Same-application window cycling, and an overlapping stack arrangement with a configurable
  width ratio. The stack is **on by default**: the setting gates only the arrangement, so
  off meant the shortcut silently did nothing rather than the feature being disabled.
- A daily check for new versions, on by default and switchable off in Settings. It is the
  only network request the product makes, and `PRIVACY.md` describes it line by line.
- A settings window with shortcut configuration, a live key-check diagnostic, and a first-run
  tutorial.
- Optional start at sign-in, through a Windows scheduled task.
- A per-machine installer, built by CI on every push rather than only at release time, and
  published with SHA-256 checksums.
- Installation through winget, as a second channel alongside the installer.

### Security

- The scheduled task no longer trusts the path it stored: an install that moved, or a task
  written by an older version, is rewritten rather than followed. A location writable by a
  non-administrator is reported, because a task that runs elevated at every sign-in with no
  prompt turns such a directory into a privilege-escalation route.
- The installer toolchain is pinned by SHA-256 with a security floor asserted during the build,
  so a version carrying a known privilege-escalation flaw cannot be reintroduced quietly.

### Known limitations

- **The binaries are not code-signed.** Windows SmartScreen will warn, and the elevation prompt
  will show an unverified publisher. `SECURITY.md` says what that does and does not tell you.
- On a Windows 11 machine with **Smart App Control** enforcing, the daemon will not start at
  all. Unlike SmartScreen this offers no way through — it judges the file itself, and an
  unsigned binary no reputation service has seen yet is refused with only an entry in the
  CodeIntegrity event log to show for it. Code signing is the fix; there is no setting in
  Wira Desk that changes it.
- `wiradesk.log` is capped by size and rotated to a single `.old` generation; there is no
  longer-term retention.

**Factory reset:** delete `%APPDATA%\WiraDesk\config.toml` only — not the folder. Migration
re-runs if the legacy `%APPDATA%\WinTick\` directory still exists; that directory is
intentionally preserved for rollback, so removing the entire `WiraDesk` folder is not a reset.
