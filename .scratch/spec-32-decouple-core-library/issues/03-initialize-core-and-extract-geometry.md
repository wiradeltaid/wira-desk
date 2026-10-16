---
id: SPEC-32-03
title: Initialize crates/core and extract geometry planners and commands
spec: SPEC-32
status: open
depends_on: [SPEC-32-02]
---

# SPEC-32-03 — Initialize crates/core and extract geometry planners and commands

## Scope
1. Create `crates/core` in Cargo workspace:
   - `Cargo.toml`: `name = "wiradesk-core"`, `license = "MIT"`, `publish = false`.
   - `<core>/src/lib.rs` with `#![forbid(unsafe_code)]`.
2. Extract pure domain modules from `crates/daemon` and `crates/shared`:
   - Geometry planners: `arrangement/mod.rs`, `arrangement/snap.rs`, `arrangement/thirds.rs`, `arrangement/stack.rs`, `arrangement/monitor.rs`.
   - Switcher layout model: `switcher/layout.rs`, `switcher/selection.rs`.
   - Command enum: `commands.rs`.
   - Update descriptor decision: `update::decide`.
3. Rewire `crates/daemon` and `crates/settings` to import extracted geometry from `wiradesk-core`.

## Verification
- `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace` passes 100% green without any regression.
