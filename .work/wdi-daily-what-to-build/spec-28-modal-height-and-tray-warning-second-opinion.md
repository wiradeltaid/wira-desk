# Review Request: SPEC-28 Modal Dialog Height and Real-time Tray Warning Clearance

You are performing an independent peer review of the drafted SPEC-28 package.

## 1. Drafted Spec and Ticket Paths
- Spec: `.scratch/spec-28-modal-dialog-height-and-realtime-tray-warning-cleanup/SPEC.md`
- Ticket 1: `.scratch/spec-28-modal-dialog-height-and-realtime-tray-warning-cleanup/issues/01-compact-modal-dialog-height.md`
- Ticket 2: `.scratch/spec-28-modal-dialog-height-and-realtime-tray-warning-cleanup/issues/02-realtime-tray-warning-clearance-across-config-sources.md`

## 2. Raw Original Notes (Unedited)
```
Catatan:
1. Ketapa `Restore all preferences to defaults` desain window dialognya seperti ini? terlalu full height, saya gak paham. kamu ada konfirmasikan sebelumnya? harusnya khan gak perlu height setinggi ini.
2. Juga icon tray warning itu gak langsung hilang - setelah masalah diperbaiki. Apa memang gak bisa realtime yah?

[Image #2]
```

## 3. Standing Mandate
If this draft touches the architecture spine, an SRS, an SDD, or a SPEC, you're authorized to run `wdi-review` on it yourself and edit the document directly to apply its stamp — no need to ask first, that permission is already given for this dispatch.
(Note: You do not have a native Skill tool; please open and follow `.claude/skills/wdi-review/SKILL.md` as plain instructions rather than invoking a skill by name.)

## 4. Instructions for Reviewer (Terra / Kiro)
1. Read the drafted SPEC and tickets carefully against the codebase:
   - `crates/settings/ui/main_window.slint`: verify why the Factory Reset dialog stretched to full window height (the inner `Rectangle` inside `modal_focus := FocusScope` has `width: 440px;` but lacks `height: dialog_layout.preferred-height;`), and whether the proposed fix accurately and robustly ensures compact, content-proportional height centered in the window.
   - `crates/daemon/src/tray.rs`, `crates/daemon/src/hook.rs`, `crates/daemon/src/log.rs`: investigate the lifecycle of shortcut collision warnings (`WARN_CAUSE_CONFIG_COLLISION` vs `WARN_CAUSE_GENERIC`), how `handle_log_warning` latches warning causes, and how `handle_config_reload_outcome` can reliably clear all config-derived warnings (collisions and rejection) and auto-start ACL warnings when auto-start is disabled, so the tray icon returns to `Normal` in real time.
2. Evaluate whether the acceptance criteria, user stories, and test decisions are complete, rigorous, and directly answer the owner's feedback.
3. Review whether any architectural invariants, precedence rules (e.g. Critical hook state precedence), or Slint layout contracts are violated.
4. Output your peer review findings clearly with:
   - Verdict: ACCEPT or CHANGES REQUESTED
   - Critique of the proposed solution
   - Any missing edge cases or recommendations
