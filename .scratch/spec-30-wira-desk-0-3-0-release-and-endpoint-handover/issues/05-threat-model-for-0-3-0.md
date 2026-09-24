---
id: SPEC-30-05
component: settings
satisfies: [FR-24, FR-25]
blocked_by: [SPEC-30-04]
status: closed
touches:
  - docs/threat-model.md
tests:
  - docs::tests::threat_model_matches_security_and_network_boundary
---

# 05: Feature — Threat model for 0.3.0

**What to build:** Update `docs/threat-model.md` to accurately document the 0.3.0 network architecture, visual switcher window title inspection, existing log rotation, and residual temp staging risks matching `SECURITY.md` Section 9 verbatim.

**Blocked by:** SPEC-30-04 (ensures `SECURITY.md` is finalized and synchronized before threat model verification)

**Status:** closed

## Implementation Details

1. **Dual Network Boundary:** Update network boundary documentation to specify two distinct paths: descriptor checks to `https://wiradelta.id/api/v1/update/wira-desk/` without redirects and with 30-day aggregate server retention per the server contract (`spec-endpoint.md` §3.4, §4.6), and installer downloads from GitHub Releases with SHA-256 validation.
2. **Window Title Enumeration:** Document that the visual switcher overlay reads active window titles to render application cards.
3. **Log Rotation:** Correct the outdated claim that log rotation is absent; document 1 MB log rotation with one `.old` file (`crates/daemon/src/log.rs:28`).
4. **Residual Risks:** Align residual risks section item-for-item with newly synchronized `SECURITY.md` Section 9, specifically detailing the temp staging directory window before installer launch.

## Acceptance Criteria

- [x] Every guard and documentation check is observed red on today's codebase before implementation, then green.
- [x] Outdated claims regarding single network boundary, absent window title usage, and absent log rotation are completely removed.
- [x] Residual risks list matches `SECURITY.md` Section 9 verbatim in content and items.
- [x] PR description cites `file:line` references for every newly documented code behavior.
- [x] Workspace verification succeeds: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --no-fail-fast`.
