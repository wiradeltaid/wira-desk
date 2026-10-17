---
id: SPEC-32-06
title: Multi-target cross-compilation verification and minor bump declaration to 0.4.0
spec: SPEC-32
status: open
depends_on: [SPEC-32-05]
---

# SPEC-32-06 — Multi-target cross-compilation verification and minor bump declaration to 0.4.0

## Scope
1. Update `deny.toml` to support multi-target auditing:
   - Include `aarch64-apple-darwin` and `x86_64-pc-windows-msvc`.
   - Ensure `crates/core` is strictly verified against copyleft licenses.
2. Execute cross-compilation check:
   `cargo check -p wiradesk-core --target aarch64-apple-darwin`
3. Execute full local Windows test harness:
   `cargo fmt --all --check`
   `cargo clippy --workspace --all-targets -- -D warnings`
   `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace --no-fail-fast`
4. Declare minor version bump to `0.4.0` under `CHANGELOG.md ## [Unreleased]` due to architectural workspace restructuring and component additions.

## Verification
- CI workflow passes on all matrix runners.
- Apple Silicon check passes with zero warnings.
- Windows physical smoke test succeeds.
