# Second Opinion Review Packet — SPEC-21 / DEF-23

## 1. Drafted Spec and Ticket Paths
- Spec: `.scratch/spec-21-installer-hardening-and-downgrade-prevention/SPEC.md`
- Ticket: `.scratch/spec-21-installer-hardening-and-downgrade-prevention/issues/01-defect-def-23-installer-downgrade-prevention-and-safety.md`
- Registry entries:
  - Defect: `DEF-23` in `.control/registry/defects.yaml`
  - Spec: `SPEC-21` in `.control/registry/specs.yaml`

## 2. Original Raw Notes from Owner (Unedited)
```
coba cek installer yang ada di snapdown (scriptnya), seharusnya diterapkan hal2 yg barik, misal konfirmasi target instalasi, konfirmasi path, lalu pembatasan downgrade. Coba cek scriptnya apa yang bisa diadopsi. ya buatkan
```

## 3. Standing Mandate
*"If this draft touches the architecture spine, an SRS, an SDD, or a SPEC, you're authorized to run `wdi-review` on it yourself and edit the document directly to apply its stamp — no need to ask first, that permission is already given for this dispatch."*

Note for non-native CLI (Kiro): You do not have the native `Skill` tool. To execute the `wdi-review` mandate, open and follow `.claude/skills/wdi-review/SKILL.md` as plain instructions directly. When applying the `spec_reviewed` stamp to `SPEC-21` in `.control/registry/specs.yaml`, use date `2026-09-14` and git SHA `6ae977a` with the applicable lenses (`[structure, prose, edge-case-hunter]`).

## 4. Review Scope & Context
Review the proposed SPEC-21 and SPEC-21-01 against:
1. The current `packaging/wiradesk.iss` script in Wira Desk.
2. The reference patterns in Snapdown's `packaging/snapdown.iss`.
3. Architectural differences between Snapdown (per-user non-elevated HKCU) and Wira Desk (per-machine elevated HKLM, dual-process daemon + UI).
4. Edge cases:
   - Behavior during `/VERYSILENT` silent automated updates (e.g. winget/in-app updater).
   - Behavior when upgrading vs reinstalling identical version vs attempting downgrade.
   - Fail-closed behavior if processes fail to exit.
   - 3-component SemVer compliance and edge cases in version comparisons.

Please provide an adversarial and edge-case review of SPEC-21 and its ticket, improve them where necessary, apply the `spec_reviewed` stamp to `.control/registry/specs.yaml` if approved, and report your findings.
