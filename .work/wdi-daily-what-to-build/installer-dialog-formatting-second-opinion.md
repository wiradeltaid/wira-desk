# Second Opinion Review Request: SPEC-22 / DEF-24

Current shared worktree on branch `main`.
(Do NOT create a separate worktree, and do NOT run concurrent cargo/iscc builds).

## 1. Drafted Specification and Ticket
- Spec document: `.scratch/spec-22-installer-dialog-formatting-and-clean-config/SPEC.md`
- Ticket document: `.scratch/spec-22-installer-dialog-formatting-and-clean-config/issues/01-defect-def-24-installer-dialog-formatting-and-clean-config.md`
- Spec registry entry: `.control/registry/specs.yaml` (`SPEC-22`)
- Defect registry entry: `.control/registry/defects.yaml` (`DEF-24`)
- Target code & packaging: `packaging/wiradesk.iss`, `scripts/verify-installer-safety.ps1`

## 2. Original Raw Notes (Unedited)
```text
Catatan:
1. Tulisan `version first` dia jadi di new line, harusnya dia gabung dengan baris sebelumnya
2. config dan wiradesk log sepertinya apa yang ada di codebase terbawa yah? bukankah harusnya dia create dari nol? untuk create default hotkey dan juga log dari new file?

Installer & Setup (packaging/wiradesk.iss — DEF-23 / SPEC-21-01)
  1. Transparent Installation Summary (UpdateReadyMemo)
     [x] Jalankan WiraDesk-0.2.0-x64-setup.exe secara interaktif hingga halaman "Ready to Install"
     [x] Pastikan ringkasan menampilkan Destination location (C:\Program Files\Wira Desk)
     [x] Pastikan ringkasan menampilkan lokasi Configuration and logs (%APPDATA%\WiraDesk)
     [x] Pastikan ringkasan menampilkan Auto-start task: WiraDesk (optional elevated logon task) dengan penegasan bahwa Setup tidak membuat/mengaktifkan task ini dan tidak menyebutnya sebagai service

  2. Version Upgrade & Reinstall (InitializeSetup)
     [x] Selesaikan instalasi versi 0.2.0 (First install)
     [x] Jalankan WiraDesk-0.2.1-x64-setup.exe; pastikan proses upgrade berjalan lancar tanpa peringatan downgrade
     [x] Jalankan kembali WiraDesk-0.2.1-x64-setup.exe saat versi 0.2.1 terpasang; pastikan reinstall versi yang sama diizinkan

  3. Downgrade Prevention (InitializeSetup)
     [x] Saat versi 0.2.1 terpasang di sistem, jalankan WiraDesk-0.2.0-x64-setup.exe
     [x Pastikan muncul kotak dialog penolakan downgrade yang menjelaskan versi 0.2.1 lebih baru telah terpasang dan versi 0.2.0 ditolak sebelum ada ekstraksi file
     [ ] (Opsional unattended) Jalankan WiraDesk-0.2.0-x64-setup.exe /VERYSILENT di terminal; pastikan keluar dengan exit code 1 tanpa menampilkan jendela dialog modal -> saaya gak test

  4. Fail-Closed Process Shutdown (PrepareToInstall / StopDaemonAndSettings)
     [x] Buka aplikasi Wira Desk daemon, lalu jalankan installer; pastikan Setup meminta daemon menutup secara sopan (WM_CLOSE) dan proses keluar bersih sebelum file diganti
     [x] Buka Settings (wiradesk-settings.exe) dengan draf belum tersimpan, lalu jalankan installer; pastikan Settings diminta tutup tanpa force-kill (/F)
```

## 3. Standing Mandate
If this draft touches the architecture spine, an SRS, an SDD, or a SPEC, you're authorized to run `wdi-review` on it yourself and edit the document directly to apply its stamp — no need to ask first, that permission is already given for this dispatch. Since `kiro-cli` has no native Skill tool, open and follow `.claude/skills/wdi-review/SKILL.md` as plain instructions rather than invoking a skill by name.

Please evaluate:
1. Does the drafted `SPEC-22` and ticket `SPEC-22-01` accurately capture Note 1 (premature newline before "version first" and similar dialog wrapping)?
2. Does the spec provide a clear architectural explanation for Note 2 (clarifying that the installer never bundles `config.toml` or `wiradesk.log`, that `%APPDATA%\WiraDesk` is persistent user state preserved across upgrades, and that on first install the app creates defaults from scratch)?
3. Review the spec lenses (`[structure, prose, edge-case-hunter]`), provide your review verdict/findings, apply the `spec_reviewed` trace in `.control/registry/specs.yaml` under `SPEC-22` if accepted (or edit the spec directly to resolve findings), and record review notes in `docs/3p.md`.
