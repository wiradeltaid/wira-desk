# Second-Opinion Review Request for SPEC-27

Please perform an independent second-opinion review of the drafted specification and tickets for SPEC-27 against the codebase and the original user findings.

## Drafted Artifacts
- SPEC: `.scratch/spec-27-tray-warning-reset-auto-start-isolation-and-settings-ui-polish/SPEC.md`
- Ticket 1: `.scratch/spec-27-tray-warning-reset-auto-start-isolation-and-settings-ui-polish/issues/01-real-time-tray-warning-reset-on-config-reload.md`
- Ticket 2: `.scratch/spec-27-tray-warning-reset-auto-start-isolation-and-settings-ui-polish/issues/02-general-auto-start-isolation-about-card-and-modal-polish.md`

## Standing Mandate
If this draft touches the architecture spine, an SRS, an SDD, or a SPEC, you're authorized to run `wdi-review` on it yourself and edit the document directly to apply its stamp — no need to ask first, that permission is already given for this dispatch.
Note: You have no native Skill tool: open and follow `.claude/skills/wdi-review/SKILL.md` as plain instructions rather than invoking a skill by name.

## Original User Notes (Verbatim)

# Review 2026-09-15 09:18

## Finding 1

![Finding 1](<D:/SnapdownVault2/bundles/01a0a2db-bc1c-75a2-a36c-4d2dee6b4d61/finding_1_burned.png>)

### Marker Notes

1. icon ada warning ini gak bisa tereset realtime yah? ketika misal warning sudah di solved, saya perhatikan dia tetap merah

## Finding 2

![Finding 2](<D:/SnapdownVault2/bundles/01a0a2db-bc1c-75a2-a36c-4d2dee6b4d61/finding_2_burned.png>)

### Marker Notes

1. tombol ini harusnya jangan keluar dari area card. atau cardnya dibesarin?

## Finding 3

![Finding 3](<D:/SnapdownVault2/bundles/01a0a2db-bc1c-75a2-a36c-4d2dee6b4d61/finding_3_burned.png>)

### Marker Notes

1. deteksi default itu tidak melihat apakah start wira desk itu toggle on atau off. artinya jangan mengubah start ini.

## Finding 4

![Finding 4](<D:/SnapdownVault2/bundles/01a0a2db-bc1c-75a2-a36c-4d2dee6b4d61/finding_4_burned.png>)

### Notes

ini jelek sekali, height window terlalu besar, dan juga maunya desain sistemnya dimiripkan dengan onboarding, tidak sama persis, tapi styling windownya, lalu buttonnya.

### Marker Notes

1. apakah ini memang ada outline biru?. di tombol2 lain gak ada outline bukan? maksudnya gak ada efek selected = outline. Ketika restore ini berarti mematikan auto start juga. beda dengan default di general

## Finding 5

![Finding 5](<D:/SnapdownVault2/bundles/01a0a2db-bc1c-75a2-a36c-4d2dee6b4d61/finding_5_burned.png>)

### Marker Notes

1. hapus judul ini `Troubleshooting & Recovery`. Sedangkan `Restore all..` tetaap ada

---

## Instructions for Reviewer:
1. Examine the drafted SPEC.md and ticket files against the 5 user findings above and the relevant codebase:
   - `crates/daemon/src/tray.rs` and `crates/daemon/src/config.rs` (real-time tray icon warning unlatch on `WM_APP_RELOAD_CONFIG`)
   - `crates/settings/src/app.rs` (excluding `auto_start` from `general_differs_from_default` and preserving it in `restore_general_defaults`)
   - `crates/settings/ui/panes/about_pane.slint` (removing `Troubleshooting & Recovery`, enclosing `[Reset all settings…]` button cleanly inside Card 3)
   - `crates/settings/ui/main_window.slint` (reducing `normal_height` to 560px, redesigning the Factory Reset confirmation dialog with 12px border radius, onboarding-like styling, and removing the 2px blue focus outline)
2. Review whether acceptance criteria, tests, and boundaries are complete, unambiguous, and directly testable.
3. Apply `wdi-review` lenses (structure, prose, edge-case-hunter). If accepted, apply the review stamp to SPEC.md and return your findings and verdict.
