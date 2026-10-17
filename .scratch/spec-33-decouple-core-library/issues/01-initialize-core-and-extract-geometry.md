---
id: SPEC-33-01
title: Initialize crates/core and extract geometry planners and commands
spec: SPEC-33
status: closed
depends_on: []
---

# SPEC-33-01 — Initialize crates/core and extract geometry planners and commands

## Scope
1. Create `crates/core` in Cargo workspace:
   - `Cargo.toml`: `name = "wiradesk-core"`, `license = "MIT"`, `publish = false`.
   - `<core>/src/lib.rs` with `#![forbid(unsafe_code)]`.
   - Create `crates/core/LICENSE` with verbatim MIT license text.
2. Extract pure domain geometry and commands into `wiradesk-core`:
   - Geometry planners: `arrangement/{mod.rs, snap.rs, thirds.rs, stack.rs, monitor.rs}` (`arrangement/win32.rs` remains strictly in `crates/daemon`).
   - Command enum: `commands.rs`.
   - Data model: `window.rs` (`WindowId`).
3. Wire backward-compatible facades in `crates/daemon/src/arrangement/` and `crates/shared/src/commands.rs`.

## Verification
- `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test -p wiradesk-core` passes 100% green.
- Existing arrangement tests in `daemon` continue to pass without modification.
