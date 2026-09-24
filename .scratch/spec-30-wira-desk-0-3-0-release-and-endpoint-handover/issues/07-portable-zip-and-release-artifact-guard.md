---
id: SPEC-30-07
component: window-management
satisfies: [FR-24]
blocked_by: []
status: closed
touches:
  - .github/workflows/release.yml
  - .github/workflows/ci.yml
  - scripts/verify-release-artifacts.ps1
tests:
  - scripts::tests::verify_release_artifacts_rejects_loose_binaries
  - scripts::tests::verify_release_artifacts_validates_zip_contents
---

# 07: Feature — Portable zip and release artifact guard

**What to build:** Package release artifacts into a single portable archive `WiraDesk-<version>-x64-portable.zip` containing both executables, `LICENSE.txt`, and `NOTICE.txt`, eliminate loose binary uploads, remove stale WinGet PR references from `release.yml:223`, and guard artifact integrity in CI and release workflows via `scripts/verify-release-artifacts.ps1`.

**Blocked by:** None (can start immediately, in parallel with SPEC-30-01 and SPEC-30-02)

**Status:** closed

## Implementation Details

1. **Portable Zip Packaging:** Update `.github/workflows/release.yml` to bundle `wiradesk.exe`, `wiradesk-settings.exe`, `LICENSE.txt`, and `NOTICE.txt` into `WiraDesk-<version>-x64-portable.zip`.
2. **Elimination of Loose Executables:** Ensure `dist/` staging does not upload individual executables. Restrict `SHA256SUMS` strictly to the setup installer and the portable zip.
3. **Artifact Verification Script:** Create `scripts/verify-release-artifacts.ps1` to assert that staging contains exactly `{installer, zip, SHA256SUMS, latest.json}`, the zip contains exactly the four required files, and checksums match.
4. **CI Integration & Workflow Cleanup:** Update `.github/workflows/ci.yml` to stage release artifacts and execute the verification script on every PR. In `.github/workflows/release.yml:223`, remove the obsolete "WinGet package submission pending ... (PR #426321)" text and replace with generic release notes text without old PR references.

## Acceptance Criteria

- [x] Every guard and verification script is observed red on today's codebase before implementation, then green.
- [x] `scripts/verify-release-artifacts.ps1` executed against loose binaries layout fails as red.
- [x] Packaging layout producing installer + zip + checksums passes the verification script as green.
- [x] Staging and verification step in `ci.yml` passes cleanly.
- [x] `.github/workflows/release.yml:223` no longer contains reference to PR #426321.
- [x] Workspace verification succeeds: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, and `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --no-fail-fast`.
