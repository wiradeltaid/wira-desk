---
id: SPEC-32-04
title: Extract cycling selection algorithms, facts evaluation, and bypass policy
spec: SPEC-32
status: open
depends_on: [SPEC-32-03]
---

# SPEC-32-04 — Extract cycling selection algorithms, facts evaluation, and bypass policy

## Scope
1. Extract pure selection algorithms from `crates/daemon/src/cycling/`:
   - `cycling/eligibility.rs` (`evaluate_facts`).
   - `cycling/selection.rs` (`select_next`, `attempt_order`).
   - `cycling/mod.rs` (`cycle_order_directed`, `WindowFacts`).
2. Extract spatial evaluation and bypass policy:
   - `context/mod.rs` (`evaluate_spatial`, `BypassPolicy::classify`).
3. Maintain Windows class name constants (`Shell_TrayWnd`, `Progman`) as default parameters in the Windows adapter to preserve exact Windows cycling semantics.
4. Rewire `crates/daemon/src/worker.rs` to consume cycling algorithms from `wiradesk-core`.

## Verification
- All cycling and context unit tests pass with zero modification to expected test outputs.
