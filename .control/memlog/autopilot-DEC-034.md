---
artifact: .control/decisions/DEC-034-autopilot-mandate-for-spec-20-delivery.md
skill: wdi-autopilot
date: 2026-09-14
---

# Memlog — autopilot run DEC-034

## Resume

Iteration: 1 (boundary: HEAD)
Run branch: autopilot/DEC-034, PR opened for owner review
Stopped at: Done (all specifications closed: SPEC-20 delivered, promise progress 100%, 90/90 counted RTM rows green)
Blocked: —
Parked: —
Next: —

## Decisions

| When | Where | Decided | Instead of | Cost if wrong | Landed in |
|---|---|---|---|---|---|
| Preflight | Mandate acceptance | Accepted DEC-034 for SPEC-20 delivery per owner launch | Stopping for manual gate check-ins | Supersede DEC-034 | .control/decisions/DEC-034-autopilot-mandate-for-spec-20-delivery.md, decisions.yaml |
| Preflight | Build safety | Pinned single target dir in main checkout to prevent race conditions and thrashing | Per-worktree target dir | Workspace crate fingerprints thrash | decisions.yaml |
| Preflight | Peer review | Configured Kiro GPT-5.6 Terra shell-out for independent code and doc review | Single-agent self-review | Reviewer independence lost | decisions.yaml |
| Iter 1 | SPEC-20-01 | Replaced Slint renderer-skia with renderer-software and disabled defaults on i-slint-backend-winit | Retaining skia-bindings dynamic C++ runtime | Executables crash with 0xC0000135 on clean Windows | crates/settings/Cargo.toml, Cargo.lock |
| Iter 1 | SPEC-20-01 | Configured target-scoped static CRT (+crt-static) exclusively for x86_64-pc-windows-msvc | Broad workspace build rustflags | Cross-compilation targets contaminated | .cargo/config.toml |
| Iter 1 | SPEC-20-01 | Removed dormant vc_redist installer bundling and AllowDynamicCrtIfBundled bypass | Claiming installer bundling that never physically stages | Loose binaries and WinGet fail package validation | packaging/wiradesk.iss, scripts/verify-release-binary.ps1, .github/workflows/ci.yml, release.yml |
| Iter 1 | SPEC-20-01 | Hardened PE import scanners to fail closed on out-of-bounds RVAs, missing null terminators, and unterminated strings per Kiro peer review | Treating malformed import tables as clean zero imports | Malformed binaries evade CRT import detection | crates/shared/src/binary.rs, scripts/verify-release-binary.ps1 |
| Iter 1 | SPEC-20-01 | Canonicalized CRT family predicate across Rust and PowerShell for vcruntime, msvcp, ucrtbase, and api-ms-win-crt | Prefix-only vs wildcard divergence | Scanner and test assertions diverge on edge cases | crates/shared/src/binary.rs, scripts/verify-release-binary.ps1 |
