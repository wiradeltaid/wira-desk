# Development Guide — Wira Desk

This guide covers building, testing, and developing Wira Desk locally on Windows 11.

---

## 1. Prerequisites

- **Operating System:** Windows 11 x64.
- **Toolchain:** Rust (latest stable) with `x86_64-pc-windows-msvc` target.
- **Build Tools:** Visual Studio C++ Build Tools (MSVC toolchain).
- **Packaging (Optional):** Inno Setup 6.7+ (for building installer packages).

---

## 2. Workspace Architecture

Wira Desk is structured as a Cargo workspace with three crates:

| Crate | Responsibility |
|---|---|
| `crates/daemon` | Elevated background process managing Win32 global hooks (`WH_KEYBOARD_LL`, `WH_MOUSE_LL`), system tray icon, window activation, and visual switcher overlay. |
| `crates/settings` | Standalone GUI companion app built with Slint (software renderer, static CRT linking) for configuring shortcuts, snapping percentages, and mouse button mappings. |
| `crates/shared` | Common domain models, TOML configuration serialization, shortcut chord parsing, and Windows API wrappers. |

---

## 3. Building from Source

### Production Build (Optimized Release)

To compile both release binaries with static CRT linkage (`+crt-static`):

```powershell
.\build.ps1 -Mode prod
```

Or using Cargo directly:

```powershell
cargo build --workspace --release --locked
```

The compiled binaries will be placed in:
- `target\release\wiradesk.exe` (Elevated tray daemon)
- `target\release\wiradesk-settings.exe` (Settings companion app)

---

## 4. Running the Test Suite

Because the daemon executable embeds an elevation manifest for UIPI window activation, testing requires setting `WIRADESK_SKIP_MANIFEST = '1'` so the test runner can launch cleanly without triggering an elevation prompt:

```powershell
$env:WIRADESK_SKIP_MANIFEST = '1'
cargo test --workspace
```

---

## 5. Code Quality & Formatting

Before submitting pull requests, ensure code formatting and compiler warnings pass without findings:

```powershell
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

---

## 6. Unsafe Code & Safety Documentation

Wira Desk uses low-level Win32 FFI for window management and message hooks. All `unsafe` blocks are held to strict compiler lints (`undocumented_unsafe_blocks` and `missing_safety_doc` are set to `deny`).

Every `unsafe` block must include a `// SAFETY:` comment stating the exact precondition relied upon (buffer capacity, thread affinity, or handle validity lifetime).

---

## 7. Hygiene & Public Export Checks

Repository hygiene and facts consistency are enforced by verification scripts:

```powershell
# Verify canonical public numbers against source code
pwsh .\scripts\verify-public-facts.ps1

# Verify public export hygiene (no private paths or internal requirement identifiers)
pwsh .\scripts\verify-public-export.ps1 -Path . -SkipHistory
```
