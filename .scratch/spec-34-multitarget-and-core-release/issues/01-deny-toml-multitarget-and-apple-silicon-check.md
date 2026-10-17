---
id: SPEC-34-01
title: Update deny.toml to multi-target and verify Apple Silicon compilation
spec: SPEC-34
status: open
depends_on: []
---

# SPEC-34-01 — Update deny.toml to multi-target and verify Apple Silicon compilation

## Scope
1. Update `deny.toml` `[graph]` section:
   ```toml
   targets = [
       "x86_64-pc-windows-msvc",
       "aarch64-apple-darwin",
   ]
   ```
2. Execute cross-compilation check against Apple Silicon:
   `cargo check -p wiradesk-core --target aarch64-apple-darwin`
3. Confirm that `crates/core` contains zero copyleft dependencies and compiles cleanly with 0 warnings and 0 errors.

## Verification
- Apple Silicon target check exits 0 with zero warnings and zero errors.
- `deny.toml` syntax valid and targets array verified.
