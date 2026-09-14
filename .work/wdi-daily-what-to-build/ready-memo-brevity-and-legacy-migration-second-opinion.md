# Second Opinion Review Request: SPEC-23 / DEF-25

Current shared worktree on branch `main`.
(Do NOT create a separate worktree, and do NOT run concurrent cargo/iscc builds).

## 1. Drafted Specification and Ticket
- Ticket document: `.scratch/spec-23-ready-memo-brevity-and-legacy-migration-retirement/issues/01-defect-def-25-ready-memo-brevity-and-legacy-migration-retirement.md`
- Spec registry entry: `.control/registry/specs.yaml` (`SPEC-23`)
- Defect registry entry: `.control/registry/defects.yaml` (`DEF-25`)
- Target code & packaging: `packaging/wiradesk.iss`, `scripts/verify-installer-safety.ps1`, `crates/shared/src/migrate.rs`, `crates/daemon/src/main.rs`, `crates/settings/src/main.rs`

## 2. Original Raw Notes (Unedited)
```text
Catatan:
1. Sepertinya copy writing disini terlalu bertele2 bukan? - coba diskusikan dengan terra:
`Destination location:
      C:\Program Files\Wira Desk

Configuration and logs:
      %APPDATA%\WiraDesk
      Preserved if present; Setup never bundles or overwrites user state.
      A missing config.toml opens first-run onboarding; completing it writes a fresh default configuration.
      The log file is created on first demand when the daemon writes a log entry.

Auto-start task:
      WiraDesk (optional elevated logon task)
      Setup does not create or enable auto-start.
      Auto-start can be enabled later from Settings or the tray icon.`
2. Kenapa Ketika jalanin wiradesk.exe config.toml default adalah ini? - bukankah snap half left dan snap half right salah? - bukankah kita ada memutuskan default valuenya sebenarnya apa yang terbaru, mengingat fitur kita sudah banyak sekali
`
[general]
auto_start = true

[switcher]
shortcut = "win+backtick"
fallback_shortcut = "alt+backtick"

[snapping]
snap_half_left = "ctrl+win+left"
snap_half_right = "ctrl+win+right"
snap_maximize = "ctrl+win+enter"

[layout]
enable_overlapping_stack = false
stack_width_percent = 50
stack_shortcut = "ctrl+win+down"

[vm_bypass]
bypass_processes = [
    "mstsc.exe",
    "vmconnect.exe",
    "vmware.exe",
    "VirtualBoxVM.exe",
    "MobaXterm.exe",
]
bypass_classes = ["VMwareUnityWindow"]
`
3. Kenapa wiradesk.log yang dibuat tidak dimulai dari blank txt?, malah saya dapati log2 lama saya:
`
[2026-07-15 06:07:06] debug: simulated Tier-2 warning
[2026-07-15 06:27:46] debug: simulated Tier-2 warning
[2026-07-24 06:23:09] debug: simulated Tier-2 warning
[2026-07-24 06:29:58] debug: simulated Tier-2 warning
[2026-07-24 06:31:31] debug: simulated Tier-2 warning
[2026-07-24 06:42:52] debug: simulated Tier-2 warning
[2026-07-24 06:48:42] debug: simulated Tier-2 warning
[2026-08-02 14:53:29] Config reload skipped: file is not valid TOML; keeping current settings
MIGRATE: config imported from legacy installation
[2026-09-14 17:34:05] Reserved shortcut configured for snapping.snap_half_left; falling back to default ctrl+alt+left
[2026-09-14 17:34:05] Reserved shortcut configured for snapping.snap_half_right; falling back to default ctrl+alt+right
`
4. Baik nomor 2 dan 3, saya sudah test berulang2, bahwa itu file baru dan memang muncul apa adanya seperti itu.

Packaging & Setup (packaging/wiradesk.iss — DEF-24 / SPEC-22-01)
  1. Joined Sentence Refusal Dialogs (InitializeSetup)
     [x] Saat versi 0.2.2 atau 0.2.1 terpasang di sistem, jalankan installer versi lebih lama (misal WiraDesk-0.2.0-x64-setup.exe atau WiraDesk-0.2.1-x64-setup.exe)
     [x] Pastikan dialog penolakan downgrade menampilkan kalimat utuh tanpa jeda baris prematur:
         'If you wish to install an older version, please uninstall the current version first.' (tidak ada baris terpisah untuk "version first.")
     [x] Jika registry DisplayVersion rusak/kosong/tidak valid, pastikan pesan penolakan tampil menyatu:
         'Setup cannot verify version compatibility. Please uninstall the current version before continuing.' (tidak ada baris terpisah untuk "version before continuing.")

  2. Transparent Ready Page Disclosure (UpdateReadyMemo)
     [x] Jalankan WiraDesk-0.2.2-x64-setup.exe (atau 0.2.1) hingga halaman wizard "Ready to Install"
     [x] Periksa ringkasan pada bagian "Configuration and logs:" memuat:
         - Lokasi: %APPDATA%\WiraDesk
         - Preserved if present; Setup never bundles or overwrites user state.
         - A missing config.toml opens first-run onboarding; completing it writes a fresh default configuration.
         - The log file is created on first demand when the daemon writes a log entry.
     [x] Periksa ringkasan Auto-start task menegaskan Setup tidak membuat/mengaktifkan task ini dan tidak menyebutnya sebagai service.

  3. Upgrade & Reinstall Sequence
     [x] Jalankan WiraDesk-0.2.1-x64-setup.exe di atas instalasi 0.2.0: pastikan proses upgrade berjalan lancar tanpa peringatan downgrade
     [x] Jalankan kembali WiraDesk-0.2.1-x64-setup.exe saat 0.2.1 terpasang: pastikan reinstall versi sama diizinkan
     [x] Jalankan WiraDesk-0.2.2-x64-setup.exe di atas instalasi 0.2.1: pastikan proses upgrade ke versi lebih baru berjalan mulus
```

## 3. Standing Mandate
If this draft touches the architecture spine, an SRS, an SDD, or a SPEC, you're authorized to run `wdi-review` on it yourself and edit the document directly to apply its stamp — no need to ask first, that permission is already given for this dispatch. Since `kiro-cli` has no native Skill tool, open and follow `.claude/skills/wdi-review/SKILL.md` as plain instructions rather than invoking a skill by name.

## 4. Specific Technical Context for Reviewer
1. **Note 1 (Ready memo copy brevity):**
   The owner correctly observed that the 3-line explanation under "Configuration and logs:" in `UpdateReadyMemo`:
   ```
   Preserved if present; Setup never bundles or overwrites user state.
   A missing config.toml opens first-run onboarding; completing it writes a fresh default configuration.
   The log file is created on first demand when the daemon writes a log entry.
   ```
   is overly verbose ("bertele-tele") for an end-user installer summary. It over-explains internal implementation details.
   Recommendation: replace with concise copy, e.g.:
   `Preserved across updates; clean installs initialize on first run.`
   or
   `User configuration and logs are preserved across updates.`

2. **Notes 2 & 3 (Root Cause of Obsolete Config and Ancient Logs):**
   Investigation confirmed why notes 2 and 3 occurred:
   `crates/shared/src/migrate.rs` contains legacy migration logic `migrate_appdata()` (marked `// LEGACY: remove in v0.3.0`).
   On developer/long-lived machines, `%APPDATA%\WinTick` still exists from early prototype development in July 2026.
   When the owner deleted `%APPDATA%\WiraDesk` to test a clean install, `migrate_appdata()` automatically detected that `%APPDATA%\WiraDesk` was missing and `%APPDATA%\WinTick` was present, and silently copied:
   - `%APPDATA%\WinTick\config.toml` (which had July 2026 pre-DEC-008 shortcuts `ctrl+win+left`, `ctrl+win+right`, `ctrl+win+down`)
   - `%APPDATA%\WinTick\wintick.log` (which had July 2026 log entries)
   - Appended `MIGRATE: config imported from legacy installation`.
   Wira Desk's actual `shared::Config::default()` already has the modern defaults (`ctrl+alt+left`, `ctrl+alt+right`, `ctrl+alt+shift+s`, etc.), but `migrate_appdata()` overwrote them with the obsolete WinTick config before onboarding could run!
   Retiring/removing this legacy `WinTick` migration (`M-01`) completely eliminates this silent pollution and ensures clean installs always start clean.

Please evaluate:
1. Does `SPEC-23-01` (`DEF-25`) accurately address Note 1 (streamlining Ready memo wording) and Notes 2/3/4 (retiring the obsolete WinTick migration so clean installs get true modern defaults and clean logs)?
2. Review the spec lenses (`[structure, prose, edge-case-hunter]`), provide your review verdict/findings, apply the `spec_reviewed` trace in `.control/registry/specs.yaml` under `SPEC-23` if accepted (or edit the ticket/registry directly to resolve findings), and record review notes in `docs/3p.md`.
