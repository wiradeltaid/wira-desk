---
id: SPEC-33-02
title: Extract switcher layout calculations and clean split of update decision
spec: SPEC-33
status: closed
depends_on: [SPEC-33-01]
---

# SPEC-33-02 — Extract switcher layout calculations and clean split of update decision

## Scope
1. Extract pure switcher card layout and navigation algorithms into `wiradesk-core`:
   - `switcher/layout.rs`: Aspect ratio calculations, scale factor, row packing (`pack_rows`), and `SwitcherLayout` geometry. (`switcher/overlay.rs` and `thumbnail.rs` remain in `daemon`).
   - `switcher/selection.rs`: Grid navigation (`select_next`, `select_prev`, `select_up`, `select_down`).
2. Extract clean split of update descriptor logic into `core/src/update.rs`:
   - `decide`, `split_https`, `Release`, `Decision`, `Rejected`.
   - Leave `windows-sys` WinHttp, `std::env::var`, and live network downloading strictly in `crates/shared/src/update.rs`.
3. Wire compatibility re-exports in `crates/daemon/src/switcher/` and `crates/shared/src/update.rs`.

## Verification
- `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test -p wiradesk-core` and `cargo test -p shared` pass 100% green.
- Zero `windows-sys` or Win32 imports in `core/src/update.rs`.
