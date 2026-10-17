---
id: SPEC-33-03
title: Extract cycling selection algorithms, facts evaluation, and spatial/bypass policy
spec: SPEC-33
status: closed
depends_on: [SPEC-33-02]
---

# SPEC-33-03 — Extract cycling selection algorithms, facts evaluation, and spatial/bypass policy

## Scope
1. Extract pure window cycling algorithms and models into `core/src/cycling/`:
   - `cycling/eligibility.rs`: `evaluate_facts`.
   - `cycling/selection.rs`: `select_next`, `select_after`, `attempt_order`.
   - `cycling/mod.rs`: `AppIdentity`, `WindowFacts`, `Candidate`, `ActiveContext`, `cycle_order`, `cycle_order_directed`.
   - Note: `cycling/{activation, source}.rs` and timer management (`SetTimer`/`KillTimer` in `worker.rs`) remain strictly in `crates/daemon`.
2. Extract spatial evaluation and foreground bypass policy into `core/src/context/`:
   - `context/mod.rs`: `evaluate_spatial`, `SpatialFacts`, `SpatialContext`, `ForegroundIdentity`, `BypassPolicy::classify`.
   - Note: Windows-specific adapters (`context/{spatial, virtual_desktop, vm_bypass}.rs`) remain in `crates/daemon`.
3. Wire compatibility re-exports and facades in `crates/daemon/src/cycling/` and `crates/daemon/src/context/`.

## Verification
- All cycling and context unit tests pass with zero modification to expected test outputs.
- `cargo check -p wiradesk-core` verifies clean compilation without OS dependencies.
