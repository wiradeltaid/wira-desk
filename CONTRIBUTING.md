# Contributing

Contributions are welcome via pull request.

## Before submitting

```powershell
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

The test suite needs one environment variable. The daemon links an elevation manifest,
which would otherwise apply to the test harness too and make it fail before any test
ran:

```powershell
$env:WIRADESK_SKIP_MANIFEST = '1'
cargo test --workspace
```

## Licensing and sign-off

Wira Desk takes contributions under the Developer Certificate of Origin (DCO) 1.1, not a
Contributor License Agreement. You keep the copyright in what you write. Your sign-off
states that you have the right to submit it under the licence of the path it lands in.

### Your contribution takes the licence of its path

The workspace carries two licences. Each change is licensed under the licence of the path
it lands in; a pull request that touches both sides is licensed path by path.

| Path | Package | Licence |
| --- | --- | --- |
| `crates/core/` | `wiradesk-core` | MIT |
| `crates/daemon/` | `daemon` (binary `wiradesk`) | GPL-3.0-only |
| `crates/settings/` | `settings` (binary `wiradesk-settings`) | GPL-3.0-only |
| `crates/shared/` | `shared` | GPL-3.0-only |
| Everything else, unless a file states otherwise | — | GPL-3.0-only |

Where the DCO says "the open source license indicated in the file", it means the `license`
field in the crate's `Cargo.toml` and the `LICENSE` file beside it; outside `crates/`, the
root `LICENSE`.

`crates/core` is MIT so it can be used in projects under any licence, including proprietary
ones. Code submitted there must be your own work, or come from a source under an
MIT-compatible permissive licence with that source's notice kept. It must not be copied or
adapted from GPL-licensed code, including the GPL crates in this repository. Moving code
from `crates/daemon`, `crates/settings`, or `crates/shared` into `crates/core` relicenses
it, and only the maintainers can make that move.

### Signing off

Sign every commit with `git commit -s`. Git builds the trailer from your `user.name` and
`user.email`:

```text
Signed-off-by: Your Name <you@example.com>
```

Use a name you are known by and an email that reaches you; anonymous sign-offs cannot be
accepted. The email should match the commit author, since that is what the check compares.

If you forgot, `git commit --amend -s` fixes the last commit and
`git rebase --signoff main` fixes every commit on your branch; then update the pull request
with `git push --force-with-lease`.

The sign-off certifies the DCO 1.1, reproduced verbatim in [`DCO`](DCO) and published at
<https://developercertificate.org>.

## What CI enforces

Four gates, all of which must pass:

- **fmt, clippy `-D warnings`, tests, release build** on Windows.
- **Secret scan** — `gitleaks` over both history and the working tree, using
  `.gitleaks.toml`. Do not widen that allowlist to make a finding go away; triage it.
- **Dependencies** — `cargo-deny check` (advisories, licences, bans, sources) using
  `deny.toml`, whose graph is pinned to `x86_64-pc-windows-msvc`. A new dependency
  carrying a licence outside the allow-list will fail, deliberately.

## Unsafe code

This is a Win32 FFI codebase, so `unsafe` is unavoidable — undocumented `unsafe` is
not. `undocumented_unsafe_blocks` and `missing_safety_doc` are `deny` in the workspace
lints, so every `unsafe` block needs a `SAFETY:` comment and the build fails without
one.

Write the precondition the block actually relies on, not a description of the call.
The useful ones usually name a buffer capacity against the size argument passed to
Win32, which component owns a handle and where it is released exactly once, which
thread the call must run on, or why an invalid handle is tolerable because the API
reports failure rather than faulting. `docs/threat-model.md` explains why this
boundary is the one worth holding to a standard.

## Repository conventions

`CLAUDE.md`, `AGENTS.md`, and `.cursorrules` hold the instructions AI coding tools read, and
they are byte-identical on purpose — three tools, three filenames, one set of rules. Change
one and change all three in the same commit; drift between them is silent, and the tool that
reads the stale copy is the one that misbehaves.

`.constitution/project/constitution.md` records how work is conducted here. `3p.md` and `docs/3p.md`
track progress for code and documentation respectively - read the relevant one before starting
and add an entry when finished.

When updating `README.md`, any localized front-page mirrors in `docs/` must be updated in the same commit to prevent documentation drift. Technical and reference documents (`DEVELOPMENT.md`, `docs/CONFIGURATION.md`, `CHANGELOG.md`, `SECURITY.md`) remain authoritative in English only.

## Planning with BMAD and WDI Method

`_bmad-output/` holds the planning archive from [BMAD-METHOD](https://github.com/bmad-code-org/BMAD-METHOD).
Read `_bmad-output/README.md` first — it explains what is historical and what was redacted.

The tooling **is committed** so contributors get the same agent setup without re-running the
installer:

| Path | Purpose |
| --- | --- |
| `_bmad/` | BMAD configuration, team overrides in `_bmad/custom/`, and `config.user.toml` |
| `.claude/skills/` | Skills for Claude Code |
| `.agents/skills/` | Skills for Cursor and Antigravity |
| `.work/` | Ephemeral agent workspace (committed when present) |
| `.constitution/` | WDI Method guides plus product rules |
| `.control/` | Registries and structure maps |

After cloning, install or refresh only if versions drift:

```powershell
npx bmad-method install
npx wdi-method update --yes
```

Contributing to the code does not require running those commands.

## AppData Files Inventory

All runtime files reside under `%APPDATA%\WiraDesk`:
- `config.toml`: Active user preferences (shortcuts, layout, mouse navigation). Persisted atomically via `.tmp` swap.
- `wiradesk.log`: Tier 2 warning and diagnostic log for non-fatal runtime events, capped at 1 MB active with a single `.old` backup generation (~2 MB total bounded footprint).
- `wiradesk-debug-trace.log`: Debug assertion trace log for elevated runtime and hook diagnostics, capped at 1 MB active with a single `.old` backup generation, compiled out in production release builds.
