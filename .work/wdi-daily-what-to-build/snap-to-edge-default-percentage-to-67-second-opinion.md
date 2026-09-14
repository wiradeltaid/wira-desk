# Second Opinion Review Request: SPEC-24 (Snap to Edge Default Percentage to 67%)

Current shared worktree on branch `main`.
(Do NOT create a separate worktree, and do NOT run concurrent cargo/iscc builds).

## 1. Drafted Specification and Ticket
- Spec document: `.scratch/spec-24-snap-to-edge-default-percentage-to-67/SPEC.md`
- Ticket document: `.scratch/spec-24-snap-to-edge-default-percentage-to-67/issues/01-feature-default-snap-percentage-to-67.md`
- Spec registry entry: `.control/registry/specs.yaml` (`SPEC-24`)
- Target code: `crates/shared/src/constants.rs`, `crates/shared/src/config.rs`, `crates/settings/src/main.rs`, and doc updates in `.what/window-management/04-usecases/UC-9-snap-window-custom-percentage.md`

## 2. Original Raw Notes (Unedited)
```text
Catatan:
1. Berarti Ketika upgrade, atau install ulang dengan tom exist, migrasi sudah di kawal yah supaya aman? termasuk misalnya kalua kita ingin tambah fitur baru, maka tom diupdate otomatis?
2. snap to edge semuanya defaultnya adalah 67%

Packaging & Setup (packaging/wiradesk.iss — DEF-24, DEF-25 / SPEC-22-01, SPEC-23-01)
1. Concise Ready to Install Memo
   [x] Jalankan installer dist\WiraDesk-0.2.1-x64-setup.exe (atau 0.2.2) hingga halaman wizard "Ready to Install"
   [x] Periksa ringkasan pada bagian "Configuration and logs:":
   - Memuat lokasi: %APPDATA%\WiraDesk
   - Memuat kalimat ringkas: Preserved across updates; clean installs start fresh.
   - Tidak memuat teks verbose lama mengenai mekanisme internal onboarding atau pembuatan on-demand file log
   [x] Periksa ringkasan Auto-start task: memuat WiraDesk (optional elevated logon task) dan menegaskan Setup tidak membuat/mengaktifkan task ini secara otomatis
2. Joined Sentence Refusal Dialogs (Downgrade Guard)
   [x] Saat versi 0.2.2 atau 0.2.1 terpasang di sistem, jalankan installer versi lebih lama (misal dist\WiraDesk-0.2.0-x64-setup.exe)
   [x] Pastikan dialog penolakan downgrade menampilkan kalimat utuh tanpa jeda baris prematur:
   If you wish to install an older version, please uninstall the current version first. (tidak ada baris terpisah untuk "version first.")
   [x] Jika nilai DisplayVersion di registry rusak atau tidak valid, pastikan pesan penolakan tampil utuh:
   Setup cannot verify version compatibility. Please uninstall the current version before continuing. (tidak ada baris terpisah untuk "version before continuing.")
3. Upgrade & Reinstall Sequence
   [x] Jalankan dist\WiraDesk-0.2.1-x64-setup.exe di atas instalasi 0.2.0: pastikan proses upgrade berjalan lancar tanpa peringatan downgrade
   [x] Jalankan kembali dist\WiraDesk-0.2.1-x64-setup.exe saat versi 0.2.1 terpasang: pastikan reinstall versi yang sama diizinkan
   [x] Jalankan dist\WiraDesk-0.2.2-x64-setup.exe di atas instalasi 0.2.1: pastikan proses upgrade ke versi lebih baru berjalan mulus

Clean Configuration & WinTick Retirement (crates/daemon, crates/settings, crates/shared — DEF-25 / SPEC-23-01)
1. Clean Install Isolation & Modern Defaults
   [x] Hapus direktori %APPDATA%\WiraDesk (meskipun folder residu legacy %APPDATA%\WinTick ada di komputer)
   [x] Jalankan aplikasi Settings (target\release\wiradesk-settings.exe atau dari hasil install)
   [x] Selesaikan atau lewati onboarding dan periksa file %APPDATA%\WiraDesk\config.toml yang tercipta
   [x] Pastikan bindmpan adalah:
   - snapping.snap_half_left = "ctrl+alt+left"
   - snapping.snap_half_right = "ctrl+alt+right"
   - layout.stack_shortcut = "ctrl+alt+shift+s"
   [x] Pastikan tidawali ctrl+win+... yang muncul
   [x] Jalankan daemon (wiradesk.exe), periksa %APPDATA%\WiraDesTIDAK ADA barisMIGRATE: config imported from legacy installation maupun entri log lama dari Juli 2026
```

## 3. Standing Mandate
If this draft touches the architecture spine, an SRS, an SDD, or a SPEC, you're authorized to run `wdi-review` on it yourself and edit the document directly to apply its stamp — no need to ask first, that permission is already given for this dispatch. Since `kiro-cli` has no native Skill tool, open and follow `.claude/skills/wdi-review/SKILL.md` as plain instructions rather than invoking a skill by name.

## 4. Specific Focus for Reviewer
- Evaluate whether setting `DEFAULT_SNAP_PERCENT = 67` in `crates/shared/src/constants.rs` cleanly satisfies requirement (2) for all 4 edges (`percent_left`, `percent_right`, `percent_top`, `percent_bottom`).
- Address question (1): Explain and verify how configuration upgrade and forward-compatibility works in Wira Desk (`#[serde(default)]` preserving existing values on disk, while omitted/new fields populate default values when read or saved).
- Check if any edge cases exist with Settings UI input, bounds clamping (1..=99), or serialization.
