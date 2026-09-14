---
type: mandate
id: DEC-034
status: accepted
touches:
  - .control/decisions/DEC-034-autopilot-mandate-for-spec-20-delivery.md
  - .control/memlog/autopilot-DEC-034.md
  - .control/registry/decisions.yaml
  - .control/registry/defects.yaml
  - .control/registry/specs.yaml
  - .scratch/spec-20-static-crt-and-software-renderer/SPEC.md
  - .scratch/spec-20-static-crt-and-software-renderer/issues/01-defect-def-22-static-crt-settings-renderer.md
  - 3p.md
  - crates/settings/Cargo.toml
  - Cargo.lock
  - .cargo/config.toml
  - packaging/wiradesk.iss
  - scripts/verify-release-binary.ps1
  - .github/workflows/release.yml
  - .github/workflows/ci.yml
  - crates/shared/src/lib.rs
  - crates/shared/src/binary.rs
  - .how/settings/SDD-settings.md
  - .constitution/project/codebase-stack-guide.md
  - deny.toml
supersedes: null
superseded_by: null
created: '2026-09-14'
accepted_by: kodesh87 (Product Owner, in session), 2026-09-14
---

# DEC-034 — Autopilot mandate for SPEC-20 delivery

## Decision

`wdi-autopilot` is mandated to carry Wira Desk from its current position — G1-G4 passed, `SPEC-1` through
`SPEC-19` closed, `SPEC-20` open with its ticket planned and peer-reviewed — through every runnable
`FR`/spec/ticket to completion, without further owner check-ins until Finish or a parked row. Parameters
live on this decision's row in `decisions.yaml` under `mandate:`.

In scope:
- `SPEC-20`: Static CRT linking via Slint software renderer migration and installer cleanup (`DEF-22`, ticket `SPEC-20-01`)

### Architectural & Scope Boundaries (SPEC-20)

1. **Defect DEF-22 Static CRT Linking via Slint Software Renderer Migration & Installer Cleanup (SPEC-20-01):**
   - Replace Slint's `renderer-skia` feature with `renderer-software` in `crates/settings/Cargo.toml`, retaining `backend-winit`, `accessibility`, and `compat-1-2`. Direct `i-slint-backend-winit` sets `default-features = false`. Eliminate `skia-bindings` and `skia-safe` from the dependency tree.
   - Configure target-scoped static CRT via `.cargo/config.toml` under `[target.x86_64-pc-windows-msvc]` with `rustflags = ["-C", "target-feature=+crt-static"]`.
   - Remove `vc_redist.x64.exe` `[Files]` and `[Run]` entries, fallback conditions, and comments from `packaging/wiradesk.iss`.
   - Make `scripts/verify-release-binary.ps1` strict by removing `-AllowDynamicCrtIfBundled` and installer text inspection. Verify and scan `wiradesk.exe` and `wiradesk-settings.exe` from the release artifacts directory against prohibited MSVC CRT DLL import families (`VCRUNTIME*.dll`, `MSVCP*.dll`, `UCRTBASE.dll`, `api-ms-win-crt-*.dll`).
   - Align CI and release workflows (`.github/workflows/ci.yml` and `.github/workflows/release.yml`) with the strict binary scanner.
   - Update tests in `crates/shared/src/lib.rs` and `crates/shared/src/binary.rs` to assert `cfg!(target_feature = "crt-static")` on MSVC and test all prohibited import families, case-insensitivity, and error handling for malformed input.
   - Update documentation in `AD-11a` (`.how/settings/SDD-settings.md`), `.constitution/project/codebase-stack-guide.md`, and `deny.toml`.

Execution constraints specified by owner:
1. Coding directly implemented by coordinator with mandatory self code review.
2. Code review performed 2 times: self code review (in-session default, no separate dispatch) + independent peer review shell-out via:
   `kiro-cli chat --model gpt-5.6-terra --effort high -a --no-interactive "<prompt>"`
3. Automated smoke testing executed by agent.
4. Peer review for coding / document review / peer analisa and perbaikan dokumen serahkan ke Kiro GPT-5.6 Terra shell-out — mandate: if draft/code touches architecture spine, SRS, SDD, or SPEC, run `wdi-review` and edit documents directly without prompting.
5. Application build/run: one shared worktree used across all build/run operations in this run (`worktree: .`). Before peer review joins build/run, inform it of the current worktree path — peer MUST NOT create its own worktree, and MUST NOT build/run concurrently while coordinator is building/running.

## Why

Unattended delivery of SPEC-20 defect remediation across the full cycle from ticket implementation, unit and integration testing, code review, documentation drift reconciliation, and release gating.

## Cost if wrong

Iteration stops at capacity or blocked state; changes roll back or park; owner intervention required at finish.
