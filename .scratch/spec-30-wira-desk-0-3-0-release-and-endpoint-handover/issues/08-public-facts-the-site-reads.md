---
id: SPEC-30-08
component: settings
satisfies: [FR-24, FR-25, FR-26]
blocked_by:
  - SPEC-30-01
  - SPEC-30-02
  - SPEC-30-06
status: closed
touches:
  - docs/public-facts.yaml
  - scripts/verify-public-facts.ps1
tests:
  - scripts::tests::verify_public_facts_all_derived_facts_pass
  - scripts::tests::verify_public_facts_memory_protocol_executed
---

# 08: Feature — Public facts the site reads

**What to build:** Update `docs/public-facts.yaml` and its verification script `scripts/verify-public-facts.ps1` to reflect verified 0.3.0 constants, system values, and endpoints pinned for consumption by the website.

**Blocked by:** SPEC-30-01, SPEC-30-02, SPEC-30-06

**Status:** ready-for-agent

## Implementation Details

1. **New and Updated Facts (Provenance: handover §7 and owner answers §12):**
   - `switcher_hold_delay_default_ms` / `_min_ms` / `_max_ms`: 300 / 100 / 500 ms (derived from `crates/shared/src/config.rs:385-387`).
   - `snap_percent_default` (67) and `snap_percent_top_default` (33) (derived from `crates/shared/src/constants.rs:260,263`).
   - `update_descriptor_url`: `https://wiradelta.id/api/v1/update/wira-desk/` (derived from `crates/shared/src/update.rs`).
   - `update_first_check_delay_s` (120) and `update_check_interval_h` (24) (derived from `crates/daemon/src/updatecheck.rs:26,29`).
   - `canonical_url`: `https://wiradelta.id/wira-desk/` (with trailing slash, derived from URL registry).
   - `autostart_onboarding_default`: `true` (derived from `crates/settings/src/app.rs:886`).
   - `winget_available`: `false` (locked in tag metadata; read by website from `main` post-tag per Q3 decision).
   - `license_identifier`: `GPL-3.0-only` (with Scoop bucket manifest added to `appears_in`).
   - **Memory Measurement Protocol (Q7):** Remeasure memory on candidate tag commit using `scripts/measure-memory.ps1` with 60-second idle measurement, recording `metric`, `measured_on`, and `measured_build` commit SHA.
2. **Verification Script Extensions:** Update `scripts/verify-public-facts.ps1` to validate all new derived facts against codebase constants and configuration defaults.

## Acceptance Criteria

- [x] Modifying any source constant temporarily turns `verify-public-facts.ps1` red, confirming test sensitivity.
- [x] `verify-public-facts.ps1` passes 100% green against current codebase values.
- [x] `docs/public-facts.yaml` explicitly sets `winget_available: false` for the 0.3.0 release tag.
- [x] Measured memory facts are recorded with valid `metric`, `measured_on`, and `measured_build` commit SHA fields after executing the 60-second idle measurement protocol.
- [x] Workspace verification succeeds: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --no-fail-fast`.
